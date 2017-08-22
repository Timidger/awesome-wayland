//! Callbacks for the `button` object in the Lua libraries

use ::luaA::{self, pushmodifiers};
use ::lua::Lua;
use ::object::signal::Signal;
use ::object::class::{Class, Object};
use libc::c_int;
use lua_sys::*;
use xcb::ffi::xproto::xcb_button_t;


LUA_OBJECT_FUNCS!(luaA::BUTTON_CLASS, Class, button_new);
LUA_CLASS_FUNCS!(luaA::BUTTON_CLASS,
                 button_class_add_signal,
                 button_class_connect_signal,
                 button_class_disconnect_signal,
                 button_class_emit_signal,
                 button_class_instances,
                 button_set_index_miss_handler,
                 button_set_newindex_miss_handler);
LUA_OBJECT_EXPORT_PROPERTY!(button_get_button, ButtonState, button, lua_pushinteger);
LUA_OBJECT_EXPORT_PROPERTY!(button_get_modifiers, ButtonState, modifiers, pushmodifiers);

/// State of the button
#[repr(C)]
pub struct ButtonState {
    pub signals: Vec<Signal>,
    pub modifiers: u16,
    pub button: xcb_button_t
}

#[allow(non_snake_case)]
pub trait Button {
    /* Methods */
    fn button_add_signal(&self, lua: &Lua) -> c_int {
        unsafe {
            button_class_add_signal(lua.0)
        }
    }
    fn button_connect_signal(&self, lua: &Lua) -> c_int {
        unsafe {
            button_class_connect_signal(lua.0)
        }
    }
    fn button_disconnect_signal(&self, lua: &Lua) -> c_int {
        unsafe {
            button_class_disconnect_signal(lua.0)
        }
    }
    fn button_emit_signal(&self, lua: &Lua) -> c_int {
        unsafe {
            button_class_emit_signal(lua.0)
        }
    }
    fn button_instances(&self, lua: &Lua) -> c_int {
        unsafe {
            button_class_instances(lua.0)
        }
    }
    fn button_set_index_miss_handler(&self, lua: &Lua) -> c_int {
        unsafe {
            button_set_index_miss_handler(lua.0)
        }
    }
    fn button_set_newindex_miss_handler(&self, lua: &Lua) -> c_int {
        unsafe {
            button_set_newindex_miss_handler(lua.0)
        }
    }
    fn button___call(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::button_new(lua.0)
        }
    }
    /* Meta */
    fn button___tostring_meta(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::object_tostring(lua.0)
        }
    }
    fn button_connect_signal_meta(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::object_connect_signal_simple(lua.0)
        }
    }
    fn button_disconnect_signal_meta(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::object_disconnect_signal_simple(lua.0)
        }
    }

    fn button_emit_signal_meta(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::object_emit_signal_simple(lua.0)
        }
    }

    /* LUA_CLASS_META methods */
    fn button___index_meta(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::class_index(lua.0)
        }
    }
    fn button___newindex_meta(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::class_newindex(lua.0)
        }
    }
}
