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
    /// A variable was already defined.
    AlreadyDefined(String),
    /// Could not find configuration file with the given path.
    /// Reason given from Lua as a string.
    FileNotFound(PathBuf, String)
}

/// Errors while interfacing with C
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FFIErr {
    /// Path had invalid UTF-8 encoding.
    InvalidUTF(PathBuf),
    /// Path contained a null byte.
    NullByte(PathBuf)
}

impl From<FFIErr> for LuaErr {
    fn from(err: FFIErr) -> Self {
    LuaErr::Load(err)
    }
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
            .ok_or_else(|| FFIErr::InvalidUTF(path.clone()))
            .and_then(|s| CString::new(s)
                      .map_err(|_| FFIErr::NullByte(path.clone())))?;
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

            // Run configuration file
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
                .map_err(|_| FFIErr::NullByte(name.into()))?;
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
                    .ok_or_else(|| FFIErr::InvalidUTF(path.clone()))
                    .and_then(|s| CString::new(s)
                              .map_err(|_| FFIErr::NullByte(path.clone())))?;
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

    pub fn return_table<T>(&self, table: HashMap<String, T>) {
        unsafe {
            let lua = self.0;
            lua_newtable(lua);
            // TODO Push values to table
        }
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
