//! Callbacks for the `tag` object in the Lua libraries

use ::lua::Lua;
use super::default;
use libc::c_int;

pub trait Tag {
    // Class Methods
    fn tag_add_signal(&mut self, lua: Lua);
    fn tag_connect_signal(&mut self, lua: Lua);
    fn tag_disconnect_signal(&mut self, lua: Lua);
    fn tag_emit_signal(&mut self, lua: Lua);
    fn tag_instances(&mut self, lua: Lua);
    fn tag_set_index_miss_handler(&mut self, lua: Lua);
    fn tag_set_newindex_miss_handler(&mut self, lua: Lua);
    // Methods
    fn tag___call(&mut self, lua: Lua);
    fn tag___tostring_meta(&mut self, lua: Lua) {
        default::__tostring_meta(lua)
    }
    fn tag_connect_signal_meta(&mut self, lua: Lua) {
        default::connect_signal_meta(lua)
    }
    fn tag_disconnect_signal_meta(&mut self, lua: Lua) {
        default::disconnect_signal_meta(lua)
    }
    // Class meta methods
    fn tag___index_meta(&mut self, lua: Lua) -> c_int {
        default::__index_meta(lua)
    }
    fn tag___newindex_meta(&mut self, lua: Lua) -> c_int {
        default::__index_meta(lua)
    }
    // Meta
    fn tag_clients_meta(&mut self, lua: Lua);
    /* Properties */
    properties!([
        tag_name,
        tag_selected,
        tag_activated
    ]);
}
