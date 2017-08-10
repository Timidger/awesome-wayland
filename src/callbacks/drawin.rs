//! Callbacks for the `drawin` object in the Lua libraries

use ::lua::Lua;
use super::default;
use libc::c_int;

pub trait Drawin {
    // Class Methods
    fn drawin_add_signal(&self, lua: Lua) -> c_int;
    fn drawin_connect_signal(&self, lua: Lua) -> c_int;
    fn drawin_disconnect_signal(&self, lua: Lua) -> c_int;
    fn drawin_emit_signal(&self, lua: Lua) -> c_int;
    fn drawin_instances(&self, lua: Lua) -> c_int;
    fn drawin_set_index_miss_handler(&self, lua: Lua) -> c_int;
    fn drawin_set_newindex_miss_handler(&self, lua: Lua) -> c_int;
    // Methods
    fn drawin___call(&self, lua: Lua) -> c_int;
    fn drawin_geometry(&self, lua: Lua) -> c_int;
    // Object meta methods
    fn drawin___tostring_meta(&self, lua: Lua) -> c_int {
        default::__tostring_meta(lua)
    }
    fn drawin_connect_signal_meta(&self, lua: Lua) -> c_int {
        default::connect_signal_meta(lua)
    }
    fn drawin_disconnect_signal_meta(&self, lua: Lua) -> c_int {
        default::disconnect_signal_meta(lua)
    }
    // Class meta methods
    fn drawin___index_meta(&self, lua: Lua) -> c_int {
        default::__index_meta(lua)
    }
    fn drawin___newindex_meta(&self, lua: Lua) -> c_int {
        default::__newindex_meta(lua)
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
