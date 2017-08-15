//! Loads the button class/object up, just like the awesome library.
#![allow(unused_variables)]

#[macro_use] extern crate awesome_wayland;
#[macro_use] extern crate lazy_static;
extern crate lua_sys;
extern crate libc;
use lua_sys::*;
use libc::c_int;

use awesome_wayland::{Lua, luaA, LuaErr, Awesome};
use awesome_wayland::object::class::{Class, Object};
use awesome_wayland::callbacks;
use awesome_wayland::callbacks::*;
use std::path::PathBuf;
use std::default::Default;

fn main() {
    let LUA = Lua::new();
    unsafe {
        let lua = LUA.0;
        luaA::object_setup(lua);
        button_class_setup(lua);
    }
    match LUA.load_and_run(PathBuf::from("examples/button-setup.lua")) {
        Ok(_) => {},
        Err(LuaErr::Load(_)) => {
            println!("Could not find lua file! Please run this from the root \
                      of the project directory");
            ::std::process::exit(1);
        },
        err => err.unwrap()
    }
}

unsafe fn button_class_setup(lua: *mut lua_State) {
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
