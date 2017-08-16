//! Callbacks for the `tag` object in the Lua libraries

use ::luaA;
use ::lua::Lua;
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
        unsafe {
            luaA::object_tostring(lua.0)
        }
    }
    fn tag_connect_signal_meta(&self, lua: Lua) -> c_int {
        unsafe {
            luaA::object_connect_signal_simple(lua.0)
        }
    }
    fn tag_disconnect_signal_meta(&self, lua: Lua) -> c_int {
        unsafe {
            luaA::object_disconnect_signal_simple(lua.0)
        }
    }
    // Class meta methods
    fn tag___index_meta(&self, lua: Lua) -> c_int {
        unsafe {
            luaA::class_index(lua.0)
        }
    }
    fn tag___newindex_meta(&self, lua: Lua) -> c_int {
        unsafe {
            luaA::class_newindex(lua.0)
        }
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
