//! Callbacks for the `drawin` object in the Lua libraries

use ::lua::Lua;
use super::default;
use libc::c_int;

pub trait Drawin {
    // Class Methods
    fn drawin_add_signal(&mut self, lua: Lua);
    fn drawin_connect_signal(&mut self, lua: Lua);
    fn drawin_disconnect_signal(&mut self, lua: Lua);
    fn drawin_emit_signal(&mut self, lua: Lua);
    fn drawin_instances(&mut self, lua: Lua);
    fn drawin_set_index_miss_handler(&mut self, lua: Lua);
    fn drawin_set_newindex_miss_handler(&mut self, lua: Lua);
    // Methods
    fn drawin___call(&mut self, lua: Lua);
    fn drawin_geometry(&mut self, lua: Lua);
    // Object meta methods
    fn drawin___tostring_meta(&mut self, lua: Lua) {
        default::__tostring_meta(lua)
    }
    fn drawin_connect_signal_meta(&mut self, lua: Lua) {
        default::connect_signal_meta(lua)
    }
    fn drawin_disconnect_signal_meta(&mut self, lua: Lua) {
        default::disconnect_signal_meta(lua)
    }
    // Class meta methods
    fn drawin___index_meta(&mut self, lua: Lua) -> c_int {
        default::__index_meta(lua)
    }
    fn drawin___newindex_meta(&mut self, lua: Lua) -> c_int {
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
