//! Callbacks for the `screen` object in the Lua libraries

use ::lua::Lua;
use libc::c_int;

pub trait Screen {
    /* Methods */
    class_methods!(button);
    fn count(&mut self, awesome: Lua);
    fn __index(&mut self, awesome: Lua);
    fn __newindex(&mut self, awesome: Lua);
    fn __call(&mut self, awesome: Lua);
    fn fake_add(&mut self, awesome: Lua);
    /* Meta */
    object_methods_meta!(button);
    class_methods_meta!();
    fn fake_remove(&mut self, awesome: Lua);
    fn fake_resize(&mut self, awesome: Lua);
    fn swap(&mut self, awesome: Lua);
    /* Properties  */
    properties!([
        geometry,
        index,
        outputs,
        workarea
    ]);
}
