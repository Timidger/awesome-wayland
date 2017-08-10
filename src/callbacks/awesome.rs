//! Callbacks for the `lua` object in the Lua libraries

use ::lua::Lua;
use libc::c_int;

pub trait Awesome {
    fn awesome_quit(&mut self, lua: Lua) -> c_int;
    fn awesome_exec(&mut self, lua: Lua) -> c_int;
    fn awesome_spawn(&mut self, lua: Lua) -> c_int;
    fn awesome_restart(&mut self, lua: Lua) -> c_int;
    fn awesome_connect_signal(&mut self, lua: Lua) -> c_int;
    fn awesome_disconnect_signal(&mut self, lua: Lua) -> c_int;
    fn awesome_emit_signal(&mut self, lua: Lua) -> c_int;
    fn awesome_systray(&mut self, lua: Lua) -> c_int;
    fn awesome_load_image(&mut self, lua: Lua) -> c_int;
    fn awesome_set_preferred_icon_size(&mut self, lua: Lua) -> c_int;
    fn awesome_register_xproperty(&mut self, lua: Lua) -> c_int;
    fn awesome_set_xproperty(&mut self, lua: Lua) -> c_int;
    fn awesome_get_xproperty(&mut self, lua: Lua) -> c_int;
    fn awesome___index(&mut self, lua: Lua) -> c_int;
    fn awesome___newindex(&mut self, lua: Lua) -> c_int;
    fn awesome_xkb_set_layout_group(&mut self, lua: Lua) -> c_int;
    fn awesome_xkb_get_layout_group(&mut self, lua: Lua) -> c_int;
    fn awesome_xkb_get_group_names(&mut self, lua: Lua) -> c_int;
    fn awesome_xrdb_get_value(&mut self, lua: Lua) -> c_int;
    fn awesome_kill(&mut self, lua: Lua) -> c_int;
    fn awesome_sync(&mut self, lua: Lua) -> c_int;
}
