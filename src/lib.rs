//! To use this library, you should need to authenticate at least one struct
//! with the [callbacks](callbacks/index.html) traits. These callbacks define what Rust
//! responds with when the appropriate global + method is called from the
//! Lua library.
//!
//! See the [traits](#traits) section for more details

// Library for handling C types
extern crate libc;
// Bindings to the system's Lua
extern crate lua_sys;
// So we can have static variables (that are thread safe)
#[macro_use] extern crate lazy_static;

#[macro_use] mod utils;
mod lua;

pub mod awesome;
pub mod callbacks;
pub use lua::*;
pub use utils::*;


pub use awesome::Awesome;
