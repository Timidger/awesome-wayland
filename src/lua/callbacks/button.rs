//! Callbacks for the `button` object in the Lua libraries

use ::lua::Lua;
use libc::c_int;

// TODO This is a class, need to setup the class properly...
// Can probably do that in the `register` macro.

pub trait Button {
    /* Methods */
    fn button_add_signal(&mut self, awesome: Lua);
    fn button_connect_signal(&mut self, awesome: Lua);
    fn button_disconnect_signal(&mut self, awesome: Lua);
    fn button_emit_signal(&mut self, awesome: Lua);
    fn button_instances(&mut self, awesome: Lua);
    fn button_set_index_miss_handler(&mut self, awesome: Lua);
    fn button_set_newindex_miss_handler(&mut self, awesome: Lua);
    fn button___call(&mut self, awesome: Lua);
    /* Meta */
    fn button___tostring_meta(&mut self, awesome: Lua) {
        // TODO implement
    }
    fn button_connect_signal_meta(&mut self, awesome: Lua) {
        // TODO implement
    }
    fn button_disconnect_signal_meta(&mut self, awesome: Lua) {
        // TODO implement
    }
    // TODO Give these the default impls
    /* LUA_CLASS_META methods */
    fn button___index_meta(&mut self, awesome: Lua) -> c_int {
        // TODO luaA_class_index
        0
    }
    fn button___newindex_meta(&mut self, awesome: Lua) -> c_int {
        // TODO luaA_class_newindex
        0
    }
    /* Properties  */
    properties!([
        button,
        modifiers
    ]);
}
