//! Callbacks for the `drawin` object in the Lua libraries

use ::luaA;
use ::lua::Lua;
use libc::c_int;

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
    fn drawin_geometry(&self, lua: &Lua) -> c_int;
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
    /* Properties */
    properties!([
        drawin_drawable,
        drawin_visible,
        drawin_ontop,
        drawin_cursor,
        drawin_x,
        drawin_y,
        drawin_width,
        drawin_height,
        drawin_type_,
        drawin_shape_bounding,
        drawin_shape_clip,
        drawin_shape_input
    ]);
}
