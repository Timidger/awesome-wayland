//! Wrapper around the raw Lua context. These define safe methods that ensure
//! correct use of the Lua stack.

use lua_sys::*;
use std::path::PathBuf;
use std::ffi::{CString, CStr};
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;

const ALREADY_DEFINED: i32 = 0;

/// Wrapper around the raw Lua context. When necessary, the raw Lua context can
/// be retrived.
#[derive(Debug)]
pub struct Lua(pub *mut lua_State);

unsafe impl Send for Lua {}
unsafe impl Sync for Lua {}

/// Errors while interacting with Lua
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LuaErr {
    /// There was an error loading the configuration file
    Load(FFIErr),
    /// Evaluation error from Lua
    Eval(String),
    /// There was an FFI error during evalution
    EvalFFI(FFIErr),
    /// There was an error loading in arguments from the Lua call
    ArgumentInvalid,
    /// A variable was already defined.
    AlreadyDefined(String),
    /// Could not find configuration file with the given path.
    /// Reason given from Lua as a string.
    FileNotFound(PathBuf, String)
}

/// Errors while interfacing with C
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FFIErr {
    /// String had invalid UTF-8 encoding.
    InvalidUTF(String),
    /// String contained a null byte.
    NullByte(String),
    /// Could not convert from C string to UTF-8 encoded one
    Conversion(CString)
}

impl Lua {
    pub fn new() -> Self {
        unsafe {
            let lua = luaL_newstate();
            if lua.is_null() {
                panic!("luaL_newstate returned NULL");
            }
            luaL_openlibs(lua);
            Lua(lua)
        }
    }

    /// Constructs the Lua object from an already initialized Lua context.
    /// # Safety
    /// You should not use the passed in Lua state after this
    /// except through the `Lua` interface (and that includes
    /// using the raw pointer directly)
    pub unsafe fn from_ptr(lua: *mut lua_State) -> Self {
        Lua(lua)
    }

    /// Loads and runs the Lua file that the path points to.
    pub fn load_and_run(&self, path: PathBuf) -> Result<(), LuaErr> {
        let path_str = path.to_str()
            .ok_or_else(||
                        LuaErr::Load(FFIErr::InvalidUTF(format!("{:?}", path))))
            .and_then(|s| CString::new(s)
                      .map_err(|_|
                               LuaErr::Load(FFIErr::NullByte(format!("{:?}", path)))))?;
        unsafe {
            let lua = &mut *self.0;
            let mut status = luaL_loadfile(lua, path_str.as_ptr());
            if status != 0 {
                // If something went wrong, error message is at the top of
                // the stack.
                let error = lua_tostring(lua, -1);
                let error = CStr::from_ptr(error).to_string_lossy().into_owned();

                Err(LuaErr::FileNotFound(path.clone(), error))?
            }

            // Run the file
            status = lua_pcallk(lua, 0, LUA_MULTRET, 0, 0, None);
            if status != 0 {
                let error = lua_tostring(lua, -1);
                let error = CStr::from_ptr(error).to_string_lossy().into_owned();

                Err(LuaErr::Eval(error))?
            }
        }
        Ok(())
    }

    /// Registers the methods in the array to the given variable name.
    ///
    /// The requirement for the name to be static is to ensure that memory
    /// does not leak. The mechanism to ensure that names can be dynamically
    /// allocated is not available at this time.
    pub fn register_methods(&self, name: &'static str, methods: &[luaL_Reg])
                            -> Result<(), LuaErr> {
        unsafe {
            let l = self.0;
            // NOTE: This is safe because we guarentee that name is static
            let c_name = CStr::from_bytes_with_nul(name.as_bytes())
                .map_err(|_| LuaErr::EvalFFI(FFIErr::NullByte(name.into())))?;
            let result = luaL_newmetatable(l, c_name.as_ptr());
            if result == ALREADY_DEFINED {
                // variable is still pushed to the stack
                lua_pop(l, 1);
                return Err(LuaErr::AlreadyDefined(name.into()))
            }
            /* Set __index to be the metatable */
            // move meta table to top of stack
            lua_pushvalue(l, -1);
            // Set the __index to be the metatable
            // NOTE Pops the value from the stack
            lua_setfield(l, -2, c_str!("__index"));

            /* Add the methods to the table */
            lua_newtable(l);
            luaL_setfuncs(l, methods.as_ptr(), 0);
            // TODO This is only valid for versions >= 5.2.
            // For old versions, there is a different way
            // and this should support that.
            // <= 5.1 code it is just: luaL_register
            lua_pushvalue(l, -1);
            // NOTE Pops the value from the stack
            lua_setglobal(l, c_name.as_ptr());

            /* Set "self" to be the metatable */
            lua_pushvalue(l, -1);
            lua_setmetatable(l, -2);

            // Pop the table we made, as well as the metatable
            lua_pop(l, 2);
        }
        Ok(())
    }

    /// Loads the library given by `lib_path` and stores the result
    /// in a global named `name`.
    pub fn load_library(&self, name: &str, lib_path: PathBuf)
                        -> Result<(), LuaErr> {
        unsafe {
            let c_name = CString::new(name)
                .expect("Name contained null bytes");
            self.load_and_run(lib_path)?;
            println!("Setting global to {:#?}", c_name);
            lua_setglobal(self.0, c_name.as_ptr());
            // TODO needed?
            ::std::mem::forget(c_name);
        }
        Ok(())
    }

    /// Adds a variable number of paths to the lookup path within lua.
    /// These lookup path is used to find libraries.
    ///
    /// Note that these values are appended on to the current lookup path.
    pub fn add_lib_lookup_path(&self, paths: &[PathBuf]) -> Result<(), LuaErr> {
        let len = paths.len();
        unsafe {
            let lua = self.0;
            // Push these on to the stack
            lua_getglobal(lua, c_str!("package"));
            lua_getfield(lua, 1, c_str!("path"));
            for path in paths {
                let c_path = path.to_str()
                    .ok_or_else(||
                                LuaErr::EvalFFI(FFIErr::InvalidUTF(path.to_str().unwrap().into())))
                    .and_then(|s| CString::new(s)
                              .map_err(|_|
                                       LuaErr::EvalFFI(FFIErr::NullByte(path.to_str().unwrap().into()))))?;
                lua_pushfstring(lua, c_path.as_ptr());
                // TODO needed?
                ::std::mem::forget(c_path);
            }
            // concatenate with thing on top of the stack when we were called
            // + 1 because we want to include the path.
            lua_concat(lua, (len + 1) as i32);
            // Now set the path to that value
            lua_setfield(lua, 1, c_str!("path"));
            // pop "package"
            lua_pop(lua, 1);
        }
        Ok(())
    }

    /// Sets up the lookup path to include the default awesoem libs.
    /// These libraries are found in `/usr/share/awesome/lib`.
    ///
    /// Note that these values are added to the current lookup path,
    /// so if you want them to take precedence call this method earlier than
    /// other methods that modify the path
    /// (e.g [add_lib_lookup_path](add_lib_lookup_path)).
    pub fn add_default_awesome_libs(&self) {
        self.add_lib_lookup_path(&[";/usr/share/awesome/lib/?.lua;".into(),
                                   ";/usr/share/awesome/lib/?/init.lua;".into()
        ]);
    }

    /// Gets the argument from indexing a lua table.
    /// This argument is always the second one.
    pub fn get_index_arg_string(&self) -> Result<String, LuaErr> {
        unsafe {
            let lua = self.0;
            let buf = luaL_checklstring(lua, 2, ::std::ptr::null_mut());
            if buf.is_null() {
                return Err(LuaErr::ArgumentInvalid)
            }
            let c_str = CStr::from_ptr(buf);
            Ok(c_str.to_owned().into_string()
               .map_err(|err|
                        LuaErr::EvalFFI(FFIErr::Conversion(err.into_cstring())))?)

        }
    }

    pub fn return_table<T>(&self, table: HashMap<String, T>) {
        unsafe {
            let lua = self.0;
            lua_newtable(lua);
            // TODO Push values to table
        }
    }

    pub fn return_string<S: Into<String>>(&self, string: S)
                                          -> Result<(), LuaErr> {
        let return_val = string.into();
        unsafe {
            let lua = self.0;
            let c_str = CString::new(return_val.clone())
                .map_err(|_| LuaErr::EvalFFI(FFIErr::NullByte(return_val)))?;
            lua_pushstring(lua, c_str.as_ptr());
            // TODO Needed?
            ::std::mem::forget(c_str);
        }
        Ok(())
    }
}


impl Deref for Lua {
    type Target = lua_State;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.0
        }
    }
}

impl DerefMut for Lua {
    fn deref_mut(&mut self) -> &mut lua_State {
        unsafe {
            &mut *self.0
        }
    }
}

/// These are methods that are straight Rust-version copies of their equivalents
/// defined in the Awesome library.
///
/// They should not be used directly and instead you should use [Lua](./Lua).
pub mod luaA {
    // This weird line is so that I can use luaA namespace explicitly here.
    use super::luaA;
    use lua_sys::*;
    use libc;
    pub unsafe fn openlib(lua: *mut lua_State, name: *const libc::c_char,
                          methods: &[luaL_Reg], meta: &[luaL_Reg]) {
        luaL_newmetatable(lua, name);
        lua_pushvalue(lua, -1); // dup meta table
        lua_setfield(lua, -2, c_str!("__index")); // metatable.__index = metatable

        luaA::registerlib(lua, ::std::ptr::null_mut(), meta);
        luaA::registerlib(lua, name, methods);
        lua_pushvalue(lua, -1); // dup self as meta table
        lua_setmetatable(lua, -2); // set self as meta table
        lua_pop(lua, 2);
    }

    pub unsafe fn registerlib(lua: *mut lua_State, libname: *const libc::c_char,
                              l: &[luaL_Reg]) {
        if ! libname.is_null() {
            lua_newtable(lua);
            luaL_setfuncs(lua, l.as_ptr(), 0);
            lua_pushvalue(lua, -1);
            lua_setglobal(lua, libname);
        } else {
            luaL_setfuncs(lua, l.as_ptr(), 0);
        }
    }

    pub unsafe fn typeerror(lua: *mut lua_State, narg: libc::c_int,
                            tname: *const libc::c_char) -> libc::c_int {
        let msg = lua_pushfstring(lua, c_str!("%s expected, got %s"),
                                  tname, lua_typename(lua, narg));
        luaL_traceback(lua, lua, ::std::ptr::null_mut(), 2);
        lua_concat(lua, 2);
        return luaL_argerror(lua, narg, msg);
    }
}
