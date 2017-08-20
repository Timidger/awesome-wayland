//! Callbacks for the `lua` object in the Lua libraries

use ::lua::Lua;
use ::luaA;
use libc::c_int;

#[allow(non_snake_case)]
pub trait Awesome {
    // NOTE You should redefine this in your compositor, and call luaA::quit
    // at the end to ensure the g_main_loop has quit properly.
    fn awesome_quit(&self, lua: &Lua) -> c_int;

    fn awesome_exec(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::exec(lua.0)
        }
    }

    fn awesome_spawn(&self, lua: &Lua) -> c_int;

    fn awesome_restart(&self, lua: &Lua) -> c_int;

    fn awesome_connect_signal(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::awesome_connect_signal(lua.0)
        }
    }

    fn awesome_disconnect_signal(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::awesome_disconnect_signal(lua.0)
        }
    }

    fn awesome_emit_signal(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::awesome_emit_signal(lua.0)
        }
    }

    /// Explicitly not defined, because this will probably be compositor specific.
    fn awesome_systray(&self, lua: &Lua) -> c_int;

    fn awesome_load_image(&self, lua: &Lua) -> c_int;

    /// Explicitly not defined, because this will probably be compositor specific.
    fn awesome_set_preferred_icon_size(&self, lua: &Lua) -> c_int;

    fn awesome_register_xproperty(&self, lua: &Lua) -> c_int;

    fn awesome_set_xproperty(&self, lua: &Lua) -> c_int;

    fn awesome_get_xproperty(&self, lua: &Lua) -> c_int;

    fn awesome___index(&self, lua: &Lua) -> c_int;

    fn awesome___newindex(&self, lua: &Lua) -> c_int;

    fn awesome_xkb_set_layout_group(&self, lua: &Lua) -> c_int;

    fn awesome_xkb_get_layout_group(&self, lua: &Lua) -> c_int;

    fn awesome_xkb_get_group_names(&self, lua: &Lua) -> c_int;

    fn awesome_xrdb_get_value(&self, lua: &Lua) -> c_int;

    fn awesome_kill(&self, lua: &Lua) -> c_int;

    fn awesome_sync(&self, lua: &Lua) -> c_int;
}
