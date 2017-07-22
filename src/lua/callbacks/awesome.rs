//! Callbacks for the `lua` object in the Lua libraries

use ::lua::Lua;

pub trait Awesome {
    fn quit(&mut self, lua: Lua);
    fn exec(&mut self, lua: Lua);
    fn spawn(&mut self, lua: Lua);
    fn restart(&mut self, lua: Lua);
    fn awesome_connect_signal(&mut self, lua: Lua);
    fn awesome_disconnect_signal(&mut self, lua: Lua);
    fn awesome_emit_signal(&mut self, lua: Lua);
    fn systray(&mut self, lua: Lua);
    fn load_image(&mut self, lua: Lua);
    fn set_preferred_icon_size(&mut self, lua: Lua);
    fn register_xproperty(&mut self, lua: Lua);
    fn set_xproperty(&mut self, lua: Lua);
    fn get_xproperty(&mut self, lua: Lua);
    fn __index(&mut self, lua: Lua);
    fn __newindex(&mut self, lua: Lua);
    fn xkb_set_layout_group(&mut self, lua: Lua);
    fn xkb_get_layout_group(&mut self, lua: Lua);
    fn xkb_get_group_names(&mut self, lua: Lua);
    fn xrdb_get_value(&mut self, lua: Lua);
    fn kill(&mut self, lua: Lua);
    fn sync(&mut self, lua: Lua);
}
