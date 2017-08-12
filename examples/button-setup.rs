//! Loads the button class/object up, just like the awesome library.
#![allow(unused_variables)]

#[macro_use] extern crate awesome_wayland;
#[macro_use] extern crate lazy_static;
extern crate lua_sys;
extern crate libc;
use lua_sys::*;
use libc::c_int;

use awesome_wayland::{Lua, luaA, LuaErr, Awesome};
use awesome_wayland::object::class::{Class};
use awesome_wayland::callbacks;
use awesome_wayland::callbacks::*;
use std::path::PathBuf;
use std::default::Default;

// TODO Do the same for LUA_CLASS_FUNCS
// can't do a macro easily though, might just write them by hand...

fn main() {
    unsafe {
        let lua = luaL_newstate();
        luaL_openlibs(lua);
        luaA::object_setup(lua);
        button_class_setup(lua);
    }
}

unsafe fn button_class_setup(lua: *mut lua_State) {
    LUA_OBJECT_FUNCS!(luaA::button_class, Class, button_class_add_signal);
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
            func: Some(button_new)
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
    let button_new = *(&button_new as *const _ as *const unsafe extern "C" fn(*mut lua_sys::lua_State) -> *mut awesome_wayland::object::class::Object);
    let mut button_class = luaA::button_class.lock().unwrap();
    luaA::class_setup(lua, &mut *button_class, c_str!("button"), NULL as _,
                      button_new, None, None,
                      Some(luaA::class_index_miss_property),
                      Some(luaA::class_newindex_miss_property),
                      &button_methods, &button_meta)
}

pub unsafe extern fn button_new(lua: *mut lua_State) -> libc::c_int {
    let mut button_class = luaA::button_class.lock().unwrap();
    luaA::class_new(lua, &mut *button_class)
}

pub unsafe extern fn button_class_connect_signal(lua: *mut lua_State)
                                                 -> libc::c_int {
    eprintln!("signal usage with add_signal()");
    0
}
pub unsafe extern fn button_class_disconnect_signal(lua: *mut lua_State)
                                                    -> libc::c_int {
    let check_string = luaL_checklstring(lua, 1, ::std::ptr::null_mut());
    let mut button_class = luaA::button_class.lock().unwrap();
    luaA::class_connect_signal_from_stack(lua,
                                          &mut *button_class,
                                          check_string,
                                          2);
    0
}
pub unsafe extern fn button_class_emit_signal(lua: *mut lua_State)
                                              -> libc::c_int {
    let check_string = luaL_checklstring(lua, 1, ::std::ptr::null_mut());
    let mut button_class = luaA::button_class.lock().unwrap();
    luaA::class_emit_signal(lua, &mut *button_class,
                            check_string, lua_gettop(lua) -1);
    0
}
pub unsafe extern fn button_class_instances(lua: *mut lua_State)
                                            -> libc::c_int {
    let button_class = luaA::button_class.lock().unwrap();
    lua_pushinteger(lua, button_class.instances as lua_Integer);
    1
}
pub unsafe extern fn button_set_index_miss_handler(lua: *mut lua_State)
                                                   -> libc::c_int {
    let mut button_class = luaA::button_class.lock().unwrap();
    luaA::registerfct(lua, 1, &mut button_class.newindex_miss_handler)
}
pub unsafe extern fn button_set_newindex_miss_handler(lua: *mut lua_State)
                                                      -> libc::c_int {
    let mut button_class = luaA::button_class.lock().unwrap();
    luaA::registerfct(lua, 1, &mut button_class.newindex_miss_handler)
}
