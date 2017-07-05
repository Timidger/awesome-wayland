//! Main exports of the library

// Library for handling C types
extern crate libc;
// Bindings to the system's Lua
extern crate lua_sys;
// So we can have static variables (that are thread safe)
#[macro_use] extern crate lazy_static;

#[macro_use] mod utils;
mod lua;

pub use lua::Lua;
