//! Defines safe functions so that the rest of the library doesn't have to use
//! the raw Lua context directly.

use lua_sys::*;
use std::path::PathBuf;
use std::ffi::{CString, CStr};

/// Wrapper around the raw Lua context, allows the rest of the library to only
/// use safe operations on the Lua thread.
pub struct Lua(*mut lua_State);

/// Errors while interacting with Lua
pub enum LuaErr {
    /// There was an error loading the configuration file
    Load(ConfigErr),
    /// Evaluation error from Lua
    Eval(String)
}

/// Errors from loading the configuration file.
pub enum ConfigErr {
    /// Path had invalid UTF-8 encoding.
    InvalidUTF(PathBuf),
    /// Path contained a null byte.
    NullByte(PathBuf),
    /// Could not find configuration file with the given path.
    /// Reason given from Lua as a string.
    FileNotFound(PathBuf, String),
}

impl From<ConfigErr> for LuaErr {
    fn from(err: ConfigErr) -> Self {
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

    /// Load the rc.lua configuration file from the specified path.
    pub fn load_configuration(&mut self, path: PathBuf) -> Result<(), LuaErr> {
        let path_str = path.to_str()
            .ok_or_else(|| ConfigErr::InvalidUTF(path.clone()))
            .and_then(|s| CString::new(s)
                      .map_err(|_| ConfigErr::NullByte(path.clone())))?;
        unsafe {
            let lua = &mut *self.0;
            let mut status = luaL_loadfile(lua, path_str.as_ptr());
            if status != 0 {
                // If something went wrong, error message is at the top of
                // the stack.
                let error = lua_tostring(lua, -1);
                let error = CStr::from_ptr(error).to_string_lossy().into_owned();

                Err(ConfigErr::FileNotFound(path.clone(), error))?
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

