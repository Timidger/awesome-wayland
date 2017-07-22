//! Callbacks for the `lua` object in the Lua libraries

use ::lua::Lua;

pub trait Awesome {
    fn awesome_quit(&mut self, lua: Lua);
    fn awesome_exec(&mut self, lua: Lua);
    fn awesome_spawn(&mut self, lua: Lua);
    fn awesome_restart(&mut self, lua: Lua);
    fn awesome_connect_signal(&mut self, lua: Lua);
    fn awesome_disconnect_signal(&mut self, lua: Lua);
    fn awesome_emit_signal(&mut self, lua: Lua);
    fn awesome_systray(&mut self, lua: Lua);
    fn awesome_load_image(&mut self, lua: Lua);
    fn awesome_set_preferred_icon_size(&mut self, lua: Lua);
    fn awesome_register_xproperty(&mut self, lua: Lua);
    fn awesome_set_xproperty(&mut self, lua: Lua);
    fn awesome_get_xproperty(&mut self, lua: Lua);
    fn awesome___index(&mut self, lua: Lua);
    fn awesome___newindex(&mut self, lua: Lua);
    fn awesome_xkb_set_layout_group(&mut self, lua: Lua);
    fn awesome_xkb_get_layout_group(&mut self, lua: Lua);
    fn awesome_xkb_get_group_names(&mut self, lua: Lua);
    fn awesome_xrdb_get_value(&mut self, lua: Lua);
    fn awesome_kill(&mut self, lua: Lua);
    fn awesome_sync(&mut self, lua: Lua);
}
