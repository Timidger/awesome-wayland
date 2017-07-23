//! Default method implementations for common operations.
//! These are implementations that can be over-ruled by the user of the library
//! but they probably don't want to do that for the general case.

use ::lua::{Object, Lua};
use lua_sys::*;
use libc::c_int;

pub fn __tostring_meta(lua: Lua) -> c_int {
    // TODO implement
    0
}

pub fn connect_signal_meta(lua: Lua) -> c_int {
    // TODO implement
    0
}

pub fn disconnect_signal_meta(lua: Lua) -> c_int {
    // TODO implement
    0
}


pub fn __index_meta(lua: Lua) -> c_int {
    println!("I'm indexing!");
    // TODO implement
    0
}

pub fn __newindex_meta(lua: Lua) -> c_int {
    // TODO implement
    0
}

pub fn index_miss_property(lua: &Lua, obj: &mut Object) -> c_int {
    // TODO signal_object_emit(L, &global_signals, "debug::index::miss", 2);
    0
}

pub fn newindex_miss_property(lua: &Lua, obj: &mut Object) -> c_int {
    // TODO signal_object_emit(L, &global_signals, "debug::newindex::miss", 3);
    0
}
