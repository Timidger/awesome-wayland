//! Callbacks for the `button` object in the Lua libraries

use ::lua::Lua;
use libc::c_int;

// TODO This is a class, need to setup the class properly...
// Can probably do that in the `register` macro.

pub trait Button {
    class_methods!();
    fn new() -> Self;
    /* Methods */
    fn __call(&mut self, awesome: Lua);
    /* Meta */
    // TODO impl these macros/constants
    // LUA_OBJECT_META(button)
    // LUA_CLASS_META
    /* Attributes */
    fn button(&mut self, awesome: Lua);
    fn modifiers(&mut self, awesome: Lua);
}
