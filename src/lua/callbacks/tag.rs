//! Callbacks for the `Tag` object in the Lua libraries

use ::lua::Lua;
use libc::c_int;

pub trait Tag {
    /* Methods */
    class_methods!(tag);
    fn __call(&mut self, lua: Lua);
    /* Meta */
    object_methods_meta!(tag);
    class_methods_meta!();
    fn clients(&mut self, lua: Lua);
    /* Properties */
    properties!([
        name,
        selected,
        activated
    ]);
}
