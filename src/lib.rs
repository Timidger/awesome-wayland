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
// XCB definitions, this is mostly so that we can use old values as Lua
// expects them, and to ensure that the right types are defined per arch
extern crate xcb;
// So we can have static variables (that are thread safe)
#[macro_use] extern crate lazy_static;
// Cairo bindings, letting us draw on the screen
// and pass surfaces to the compositor
extern crate cairo;
// GLib bindings
extern crate glib_sys;
// libsn bindings
extern crate libsn_sys;
// xorg-util bindings
extern crate xcb_util_sys;
// xorg-util-xrm bindings
extern crate xcb_util_xrm_sys;
// xorg-cursor bindings
extern crate xcb_cursor_sys;
// xkbcommon bindings
extern crate xkbcommon_sys;

// generated cairo bindings, so that we can have the cairo creation using xcb
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
mod cairo_xcb {
    use ::libc;
    include!("cairo_xcb_gen.rs");
}

#[macro_use] mod utils;
mod lua;

pub mod object;
pub mod awesome;
pub mod callbacks;
pub use lua::*;
pub use utils::*;
pub mod globalconf;

pub use globalconf::GLOBAL_CONF;
pub use awesome::Awesome;
