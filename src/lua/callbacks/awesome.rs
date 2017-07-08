//! Callbacks for the `awesome` object in the Lua libraries

use ::lua::Lua;

pub trait Awesome {
    fn new() -> Self;
    fn quit(&mut self, awesome: Lua);
    fn exec(&mut self, awesome: Lua);
    fn spawn(&mut self, awesome: Lua);
    fn restart(&mut self, awesome: Lua);
    fn connect_signal(&mut self, awesome: Lua);
    fn disconnect_signal(&mut self, awesome: Lua);
    fn emit_signal(&mut self, awesome: Lua);
    fn systray(&mut self, awesome: Lua);
    fn load_image(&mut self, awesome: Lua);
    fn set_preferred_icon_size(&mut self, awesome: Lua);
    fn register_xproperty(&mut self, awesome: Lua);
    fn set_xproperty(&mut self, awesome: Lua);
    fn get_xproperty(&mut self, awesome: Lua);
    fn __index(&mut self, awesome: Lua);
    fn __newindex(&mut self, awesome: Lua);
    fn xkb_set_layout_group(&mut self, awesome: Lua);
    fn xkb_get_layout_groub(&mut self, awesome: Lua);
    fn xkb_get_group_names(&mut self, awesome: Lua);
    fn xrdb_get_value(&mut self, awesome: Lua);
    fn kill(&mut self, awesome: Lua);
    fn sync(&mut self, awesome: Lua);
}
