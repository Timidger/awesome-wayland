//! Callbacks for the `tag` object in the Lua libraries

use ::lua::Lua;
use super::default;
use libc::c_int;

#[allow(non_snake_case)]
pub trait Tag {
    // Class Methods
    fn tag_add_signal(&self, lua: Lua) -> c_int;
    fn tag_connect_signal(&self, lua: Lua) -> c_int;
    fn tag_disconnect_signal(&self, lua: Lua) -> c_int;
    fn tag_emit_signal(&self, lua: Lua) -> c_int;
    fn tag_instances(&self, lua: Lua) -> c_int;
    fn tag_set_index_miss_handler(&self, lua: Lua) -> c_int;
    fn tag_set_newindex_miss_handler(&self, lua: Lua) -> c_int;
    // Methods
    fn tag___call(&self, lua: Lua) -> c_int;
    fn tag___tostring_meta(&self, lua: Lua) -> c_int {
        default::__tostring_meta(lua)
    }
    fn tag_connect_signal_meta(&self, lua: Lua) -> c_int {
        default::connect_signal_meta(lua)
    }
    fn tag_disconnect_signal_meta(&self, lua: Lua) -> c_int {
        default::disconnect_signal_meta(lua)
    }
    // Class meta methods
    fn tag___index_meta(&self, lua: Lua) -> c_int {
        default::__index_meta(lua)
    }
    fn tag___newindex_meta(&self, lua: Lua) -> c_int {
        default::__index_meta(lua)
    }
    // Meta
    fn tag_clients_meta(&self, lua: Lua) -> c_int;
    /* Properties */
    properties!([
        tag_name,
        tag_selected,
        tag_activated
    ]);
}
