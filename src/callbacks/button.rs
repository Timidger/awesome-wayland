//! Callbacks for the `button` object in the Lua libraries

use ::lua::Lua;
use super::default;
use libc::c_int;

// TODO This is a class, need to setup the class properly...
// Can probably do that in the `register` macro.

pub trait Button {
    /* Methods */
    fn button_add_signal(&self, lua: Lua) -> c_int;
    fn button_connect_signal(&self, lua: Lua) -> c_int;
    fn button_disconnect_signal(&self, lua: Lua) -> c_int;
    fn button_emit_signal(&self, lua: Lua) -> c_int;
    fn button_instances(&self, lua: Lua) -> c_int;
    fn button_set_index_miss_handler(&self, lua: Lua) -> c_int;
    fn button_set_newindex_miss_handler(&self, lua: Lua) -> c_int;
    fn button___call(&self, lua: Lua) -> c_int;
    /* Meta */
    fn button___tostring_meta(&self, lua: Lua) -> c_int {
        default::__tostring_meta(lua)
    }
    fn button_connect_signal_meta(&self, lua: Lua) -> c_int {
        default::connect_signal_meta(lua)
    }
    fn button_disconnect_signal_meta(&self, lua: Lua) -> c_int {
        default::disconnect_signal_meta(lua)
    }
    // TODO Give these the default impls
    /* LUA_CLASS_META methods */
    fn button___index_meta(&self, lua: Lua) -> c_int {
        default::__index_meta(lua)
    }
    fn button___newindex_meta(&self, lua: Lua) -> c_int {
        default::__newindex_meta(lua)
    }
    /* Properties  */
    properties!([
        button,
        modifiers
    ]);
}
