//! Callbacks for the `button` object in the Lua libraries

use ::lua::Lua;
use libc::c_int;

// TODO This is a class, need to setup the class properly...
// Can probably do that in the `register` macro.

pub trait Button {
    /* Methods */
    class_methods!(button);
    fn __call(&mut self, awesome: Lua);
    /* Meta */
    object_methods_meta!(button);
    class_methods_meta!();
    /* Properties  */
    fn button(&mut self, awesome: Lua);
    fn modifiers(&mut self, awesome: Lua);
}
