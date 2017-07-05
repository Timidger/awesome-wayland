//! Wrapper around the raw Lua context. These define safe methods that ensure
//! correct use of the Lua stack.

use lua_sys::*;
use std::path::PathBuf;
use std::ffi::{CString, CStr};

const ALREADY_DEFINED: i32 = 0;

/// Wrapper around the raw Lua context. When necessary, the raw Lua context can
/// be retrived.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Lua(pub *mut lua_State);

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
            init_path(lua);
            Lua(lua)
        }
    }

    /// Constructs the Lua object from an already initialized Lua context.
    pub unsafe fn from_ptr(lua: *mut lua_State) -> Self {
        Lua(lua)
    }

    /// Loads and runs the Lua file that the path points to.
    pub fn load_and_run(&mut self, path: PathBuf) -> Result<(), LuaErr> {
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
    pub fn register_methods(&mut self, name: &'static str, methods: &[luaL_Reg])
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
}


/// Sets up the awesome libraries, and then executes "rc.lua".
unsafe fn init_path(lua: *mut lua_State) {
    // Sets up the global Lua path
    lua_getglobal(lua, c_str!("package")); // Push "package" to stack
    lua_getfield(lua, 1, c_str!("path")); // Push current path to stack
    // Push strings to stack
    lua_pushfstring(lua, c_str!(";/usr/share/awesome/lib/?.lua;"));
    lua_pushfstring(lua, c_str!(";/usr/share/awesome/lib/?/init.lua;"));
    lua_concat(lua, 3); // Concat those strings to the path
    // NOTE Pops the value from the stack
    lua_setfield(lua, 1, c_str!("path")); // Set path to the concat-ed string
    lua_pop(lua, 2); // pop "package" and "path"
}
