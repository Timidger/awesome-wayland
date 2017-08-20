//! Callbacks for the `drawin` object in the Lua libraries

use ::luaA;
use ::lua::Lua;
use libc::c_int;
use lua_sys::*;
use luaA::DRAWIN_CLASS;
use ::object::WindowState;
use ::callbacks::drawable::{self, DrawableState};

#[repr(C)]
pub struct DrawinState {
    pub window: WindowState,
    pub ontop: bool,
    pub visible: bool,
    pub cursor: String,
    /// The drawable for this drawin
    pub drawable: *mut DrawableState,
    /// The window geometry
    pub geometry: luaA::area_t,
    /// Do we have a pending geometry change that still needs to be applied?
    pub geometry_dirty: bool
}

#[allow(non_snake_case)]
pub trait Drawin {
    // Class Methods
    fn drawin_add_signal(&self, lua: &Lua) -> c_int;
    fn drawin_connect_signal(&self, lua: &Lua) -> c_int;
    fn drawin_disconnect_signal(&self, lua: &Lua) -> c_int;
    fn drawin_emit_signal(&self, lua: &Lua) -> c_int;
    fn drawin_instances(&self, lua: &Lua) -> c_int;
    fn drawin_set_index_miss_handler(&self, lua: &Lua) -> c_int;
    fn drawin_set_newindex_miss_handler(&self, lua: &Lua) -> c_int;
    // Methods
    fn drawin___call(&self, lua: &Lua) -> c_int;
    // Object meta methods
    fn drawin___tostring_meta(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::object_tostring(lua.0)
        }
    }
    fn drawin_connect_signal_meta(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::object_connect_signal_simple(lua.0)
        }
    }
    fn drawin_disconnect_signal_meta(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::object_disconnect_signal_simple(lua.0)
        }
    }
    // Class meta methods
    fn drawin___index_meta(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::class_index(lua.0)
        }
    }
    fn drawin___newindex_meta(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::class_newindex(lua.0)
        }
    }
}
