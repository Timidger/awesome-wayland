//! Callbacks for the `screen` object in the Lua libraries

use ::lua::Lua;
use super::default;
use libc::c_int;

pub trait Screen {
    // Class Methods
    fn screen_add_signal(&mut self, lua: Lua) -> c_int;
    fn screen_connect_signal(&mut self, lua: Lua) -> c_int;
    fn screen_disconnect_signal(&mut self, lua: Lua) -> c_int;
    fn screen_emit_signal(&mut self, lua: Lua) -> c_int;
    fn screen_instances(&mut self, lua: Lua) -> c_int;
    fn screen_set_index_miss_handler(&mut self, lua: Lua) -> c_int;
    fn screen_set_newindex_miss_handler(&mut self, lua: Lua) -> c_int;
    // Methods
    fn screen_count(&mut self, lua: Lua) -> c_int;
    fn screen___index(&mut self, lua: Lua) -> c_int;
    fn screen___newindex(&mut self, lua: Lua) -> c_int;
    fn screen___call(&mut self, lua: Lua) -> c_int;
    fn screen_fake_add(&mut self, lua: Lua) -> c_int;
    // Object meta methods
    fn screen___tostring_meta(&mut self, lua: Lua) -> c_int {
        default::__tostring_meta(lua)
    }
    fn screen_connect_signal_meta(&mut self, lua: Lua) -> c_int {
        default::connect_signal_meta(lua)
    }
    fn screen_disconnect_signal_meta(&mut self, lua: Lua) -> c_int {
        default::disconnect_signal_meta(lua)
    }
    // Class meta methods
    fn screen___index_meta(&mut self, lua: Lua) -> c_int {
        default::__index_meta(lua)
    }
    fn screen___newindex_meta(&mut self, lua: Lua) -> c_int {
        default::__index_meta(lua)
    }
    // Meta methods
    fn screen_fake_remove(&mut self, lua: Lua) -> c_int;
    fn screen_fake_resize(&mut self, lua: Lua) -> c_int;
    fn screen_swap(&mut self, lua: Lua) -> c_int;
    /* Properties  */
    properties!([
        screen_geometry,
        screen_index,
        screen_outputs,
        screen_workarea
    ]);
}
