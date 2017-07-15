//! Callbacks for the `client` object in the Lua libraries

use ::lua::Lua;
use libc::c_int;

pub trait Client {
    /* Methods */
    class_methods!(client);
    fn get(&mut self, awesome: Lua);
    fn __index(&mut self, awsemoe: Lua);
    fn __newindex(&mut self, awsemoe: Lua);
    /* Meta */
    object_methods_meta!(client);
    class_methods_meta!();
    fn keys(&mut self, awesome: Lua);
    fn isvisible(&mut self, awesome: Lua);
    fn geometry(&mut self, awesome: Lua);
    fn apply_size_hints(&mut self, awesome: Lua);
    fn tags(&mut self, awesome: Lua);
    fn kill(&mut self, awesome: Lua);
    fn swap(&mut self, awesome: Lua);
    fn raise(&mut self, awesome: Lua);
    fn lower(&mut self, awesome: Lua);
    fn unmanange(&mut self, awesome: Lua);
    fn titlebar_top(&mut self, awesome: Lua);
    fn titlebar_right(&mut self, awesome: Lua);
    fn titlebar_bottom(&mut self, awesome: Lua);
    fn titlebar_left(&mut self, awesome: Lua);
    fn get_icon(&mut self, awesome: Lua);
    /* Properties */
    // TODO
}
