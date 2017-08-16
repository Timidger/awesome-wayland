//! Callbacks for the `button` object in the Lua libraries

use ::lua::Lua;
use super::default;
use libc::c_int;

// TODO This is a class, need to setup the class properly...
// Can probably do that in the `register` macro.

pub trait Button {
    /* Methods */
    fn button_add_signal(&self, lua: Lua) -> c_int;
    fn button_connect_signal(&self, lua: Lua) -> c_int;
    fn button_disconnect_signal(&self, lua: Lua) -> c_int;
    fn button_emit_signal(&self, lua: Lua) -> c_int;
    fn button_instances(&self, lua: Lua) -> c_int;
    fn button_set_index_miss_handler(&self, lua: Lua) -> c_int;
    fn button_set_newindex_miss_handler(&self, lua: Lua) -> c_int;
    fn button___call(&self, lua: Lua) -> c_int;
    /* Meta */
    fn button___tostring_meta(&self, lua: Lua) -> c_int {
        default::__tostring_meta(lua)
    }
    fn button_connect_signal_meta(&self, lua: Lua) -> c_int {
        default::connect_signal_meta(lua)
    }
    fn button_disconnect_signal_meta(&self, lua: Lua) -> c_int {
        default::disconnect_signal_meta(lua)
    }
    // TODO Give these the default impls
    /* LUA_CLASS_META methods */
    fn button___index_meta(&self, lua: Lua) -> c_int {
        default::__index_meta(lua)
    }
    fn button___newindex_meta(&self, lua: Lua) -> c_int {
        default::__newindex_meta(lua)
    }
    /* Properties  */
    properties!([
        button,
        modifiers
    ]);
}


use lua_sys::*;
use ::luaA;
use ::object::class::{Class, Object};

pub unsafe fn button_class_setup(lua: *mut lua_State) {
    LUA_OBJECT_FUNCS!(luaA::button_class, Class, button_new);
    LUA_CLASS_FUNCS!(luaA::button_class, button_class_add_signal,
                     button_class_connect_signal,
                     button_class_disconnect_signal,
                     button_class_emit_signal,
                     button_class_instances,
                     button_set_index_miss_handler,
                     button_set_newindex_miss_handler);
    let button_methods = [
        luaL_Reg {
            name: c_str!("add_signal"),
            func: Some(button_class_add_signal)
        },
        luaL_Reg {
            name: c_str!("connect_signal"),
            func: Some(button_class_connect_signal)
        },
        luaL_Reg {
            name: c_str!("disconnect_signal"),
            func: Some(button_class_disconnect_signal)
        },
        luaL_Reg {
            name: c_str!("emit_signal"),
            func: Some(button_class_emit_signal)
        },
        luaL_Reg {
            name: c_str!("instances"),
            func: Some(button_class_instances)
        },
        luaL_Reg {
            name: c_str!("set_index_miss_handler"),
            func: Some(button_set_index_miss_handler)
        },
        luaL_Reg {
            name: c_str!("set_newindex_miss_handler"),
            func: Some(button_set_newindex_miss_handler)
        },
        luaL_Reg {
            name: c_str!("__call"),
            func: Some(luaA::button_new)
        },
        luaL_Reg {
            name: ::std::ptr::null_mut(),
            func: None
        }
    ];

    let button_meta = [
        luaL_Reg {
            name: c_str!("__tostring"),
            func: Some(luaA::object_tostring)
        },
        luaL_Reg {
            name: c_str!("connect_signal"),
            func: Some(luaA::object_connect_signal_simple)
        },
        luaL_Reg {
            name: c_str!("disconnect_signal"),
            func: Some(luaA::object_disconnect_signal_simple)
        },
        luaL_Reg {
            name: c_str!("emit_signal"),
            func: Some(luaA::object_emit_signal_simple)
        },
        luaL_Reg {
            name: c_str!("__index"),
            func: Some(luaA::class_index)
        },
        luaL_Reg {
            name: c_str!("__newindex"),
            func: Some(luaA::class_newindex)
        },
        luaL_Reg {
            name: ::std::ptr::null_mut(),
            func: None
        }
    ];
    let NULL = ::std::ptr::null_mut();
    let mut button_class = luaA::button_class.lock().unwrap();
    luaA::class_setup(lua, &mut *button_class, c_str!("button"), NULL as _,
                      button_new, None, None,
                      Some(luaA::class_index_miss_property),
                      Some(luaA::class_newindex_miss_property),
                      &button_methods, &button_meta)
}
