//! Callbacks for the `screen` object in the Lua libraries

use ::lua::Lua;
use super::default;
use libc::c_int;

#[allow(non_snake_case)]
pub trait Screen {
    // Class Methods
    fn screen_add_signal(&self, lua: Lua) -> c_int;
    fn screen_connect_signal(&self, lua: Lua) -> c_int;
    fn screen_disconnect_signal(&self, lua: Lua) -> c_int;
    fn screen_emit_signal(&self, lua: Lua) -> c_int;
    fn screen_instances(&self, lua: Lua) -> c_int;
    fn screen_set_index_miss_handler(&self, lua: Lua) -> c_int;
    fn screen_set_newindex_miss_handler(&self, lua: Lua) -> c_int;
    // Methods
    fn screen_count(&self, lua: Lua) -> c_int;
    fn screen___index(&self, lua: Lua) -> c_int;
    fn screen___newindex(&self, lua: Lua) -> c_int;
    fn screen___call(&self, lua: Lua) -> c_int;
    fn screen_fake_add(&self, lua: Lua) -> c_int;
    // Object meta methods
    fn screen___tostring_meta(&self, lua: Lua) -> c_int {
        default::__tostring_meta(lua)
    }
    fn screen_connect_signal_meta(&self, lua: Lua) -> c_int {
        default::connect_signal_meta(lua)
    }
    fn screen_disconnect_signal_meta(&self, lua: Lua) -> c_int {
        default::disconnect_signal_meta(lua)
    }
    // Class meta methods
    fn screen___index_meta(&self, lua: Lua) -> c_int {
        default::__index_meta(lua)
    }
    fn screen___newindex_meta(&self, lua: Lua) -> c_int {
        default::__index_meta(lua)
    }
    // Meta methods
    fn screen_fake_remove(&self, lua: Lua) -> c_int;
    fn screen_fake_resize(&self, lua: Lua) -> c_int;
    fn screen_swap(&self, lua: Lua) -> c_int;
    /* Properties  */
    properties!([
        screen_geometry,
        screen_index,
        screen_outputs,
        screen_workarea
    ]);
}
