//! Default method implementations for common operations.
//! These are implementations that can be over-ruled by the user of the library
//! but they probably don't want to do that for the general case.

#[allow(non_snake_case)]

use ::lua::Lua;
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
    // TODO implement
    0
}

pub fn __newindex_meta(lua: Lua) -> c_int {
    // TODO implement
    0
}
