//! Callbacks for the `drawable` object in the Lua libraries

use ::luaA;
use ::lua::Lua;
use libc::{self, c_int};
use ::object::Signal;
use ::object::class::{Class, Object};
use lua_sys::*;
use cairo::surface::Surface;

pub type RefreshCallback = fn(*mut libc::c_void);

LUA_OBJECT_FUNCS!(luaA::DRAWABLE_CLASS, Class, new);

#[repr(C)]
pub struct DrawableState {
    pub signals: Vec<Signal>,
    /// The pixmap we are drawing to
    pub pixmap: *mut libc::c_void,
    /// Surface for drawing
    // TODO Switch to cairo::Surface type?
    pub surface: Option<Surface>,
    /// The geometry of the drawable (in root window coordinates)
    pub geometry: luaA::area_t,
    /// Surface contents are undefined if this is false
    // TODO Fix that ^
    pub refreshed: bool,
    /// Callback for refreshing
    pub refresh_callback: RefreshCallback,
    /// Data for refresh callback
    pub refresh_data: *mut libc::c_void
}

#[allow(non_snake_case)]
pub trait Drawable {
    // Class Methods

    fn drawable_add_signal(&self, lua: &Lua) -> c_int;

    fn drawable_connect_signal(&self, lua: &Lua) -> c_int;

    fn drawable_disconnect_signal(&self, lua: &Lua) -> c_int;

    fn drawable_emit_signal(&self, lua: &Lua) -> c_int;

    fn drawable_instances(&self, lua: &Lua) -> c_int;

    fn drawable_set_index_miss_handler(&self, lua: &Lua) -> c_int;

    fn drawable_set_newindex_miss_handler(&self, lua: &Lua) -> c_int;
    // Object meta methods

    fn drawable___tostring_meta(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::object_tostring(lua.0)
        }
    }

    fn drawable_connect_signal_meta(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::object_connect_signal_simple(lua.0)
        }
    }

    fn drawable_disconnect_signal_meta(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::object_disconnect_signal_simple(lua.0)
        }
    }
    // Class meta methods

    fn drawable___index_meta(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::class_index(lua.0)
        }
    }

    fn drawable___newindex_meta(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::class_newindex(lua.0)
        }
    }

    fn drawable_refresh(&self, lua: &Lua) -> c_int;

    fn drawable_geometry(&self, lua: &Lua) -> c_int;
}

pub unsafe fn wipe(d: *mut Object) {
    drawable_unset_surface(d as _);
}

pub unsafe fn drawable_unset_surface(d: *mut DrawableState) {
    let d = &mut *d;
    if let Some(mut surface) = d.surface.take() {
        surface.finish();
        // When surface is dropped, it calls `cairo_surface_destroy`
    }
    if d.pixmap != 0 as *mut _ {
        // TODO FIXME free pixmap
    }
    d.refreshed = false;
    // TODO
    //d.pixmap = XCB_NONE;
}
