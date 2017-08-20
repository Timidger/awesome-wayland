//! Callbacks for the `lua` object in the Lua libraries

use ::lua::Lua;
use ::luaA;
use ::GLOBAL_CONF;
use libc::c_int;

#[allow(non_snake_case)]
pub trait Awesome {
    // NOTE You should redefine this in your compositor, and call luaA::quit
    // at the end to ensure the g_main_loop has quit properly.
    fn awesome_quit(&self, lua: &Lua) -> c_int;

    fn awesome_exec(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::exec(lua.0)
        }
    }

    fn awesome_spawn(&self, lua: &Lua) -> c_int;

    fn awesome_restart(&self, lua: &Lua) -> c_int;

    fn awesome_connect_signal(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::awesome_connect_signal(lua.0)
        }
    }

    fn awesome_disconnect_signal(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::awesome_disconnect_signal(lua.0)
        }
    }

    fn awesome_emit_signal(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::awesome_emit_signal(lua.0)
        }
    }

    /// Explicitly not defined, because this will probably be compositor specific.
    fn awesome_systray(&self, lua: &Lua) -> c_int;

    fn awesome_load_image(&self, lua: &Lua) -> c_int;

    /// Explicitly not defined, because this will probably be compositor specific.
    fn awesome_set_preferred_icon_size(&self, lua: &Lua) -> c_int;

    fn awesome_register_xproperty(&self, lua: &Lua) -> c_int;

    /// Explicitly not defined, because this will probably be compositor specific.
    fn awesome_set_xproperty(&self, lua: &Lua) -> c_int;

    /// Explicitly not defined, because this will probably be compositor specific.
    fn awesome_get_xproperty(&self, lua: &Lua) -> c_int;

    fn awesome___index(&self, lua: &Lua) -> c_int {
        unsafe {
            awesome_index(lua.0)
        }
    }

    fn awesome___newindex(&self, lua: &Lua) -> c_int;

    fn awesome_xkb_set_layout_group(&self, lua: &Lua) -> c_int;

    fn awesome_xkb_get_layout_group(&self, lua: &Lua) -> c_int;

    fn awesome_xkb_get_group_names(&self, lua: &Lua) -> c_int;

    fn awesome_xrdb_get_value(&self, lua: &Lua) -> c_int;

    fn awesome_kill(&self, lua: &Lua) -> c_int;

    fn awesome_sync(&self, lua: &Lua) -> c_int;
}


use lua_sys::*;
use std::ffi::{CStr, CString};

#[allow(unreachable_code)]
unsafe fn awesome_index(lua: *mut lua_State) -> c_int {
    use ::luaA;
    if luaA::usemetatable(lua, 1, 2) != 0 {
        return 1
    }

    let buf_c = luaL_checklstring(lua, 2, ::std::ptr::null_mut());
    let buf = CStr::from_ptr(buf_c).to_str().unwrap();
    let global_conf = GLOBAL_CONF.try_lock().unwrap();
    match buf {
        "conffile" => {
            unimplemented!();
            //lua_pushstring(lua, CONFFILE);
            1
        },
        "version" | "release" => {
            unimplemented!();
            1
        },
        "startup" => {
            let g_loop = if global_conf.g_loop == ::std::ptr::null_mut() {
                1
            } else { 0 };
            lua_pushboolean(lua, g_loop);
            1
        },
        "startup_errors" => {
            if global_conf.startup_errors.len() == 0 {
                0
            } else {
                let error = CString::new(&*global_conf.startup_errors[0])
                    .expect("Could not convert error string to C string");
                lua_pushstring(lua, error.as_ptr());
                ::std::mem::forget(error);
                1
            }
        },
        "composite_manager_running" => {
            // TODO Probably should expose this to the library somehow
            lua_pushboolean(lua, 1);
            1
        },
        "hostname" => {
            unimplemented!();
            1
        },
        "themes_path" => {
            lua_pushstring(lua, c_str!("@AWESOME_THEMES_PATH@"));
            1
        }
        "icon_path" => {
            lua_pushstring(lua, c_str!("@AWESOME_ICON_PATH@"));
            1
        },
        _ => {
            luaA::default_index(lua)
        }
    }
}
