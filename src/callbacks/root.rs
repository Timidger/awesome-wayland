//! Callbacks for the `root` object in the Lua libraries

use ::lua::Lua;
use libc::c_int;

pub trait Root {
    /* Methods */
    fn root_buttons(&self, lua: Lua) -> c_int;
    fn root_keys(&self, lua: Lua) -> c_int;
    fn root_cursor(&self, lua: Lua) -> c_int;
    fn root_fake_input(&self, lua: Lua) -> c_int;
    fn root_drawins(&self, lua: Lua) -> c_int;
    fn root_wallpaper(&self, lua: Lua) -> c_int;
    fn root_size(&self, lua: Lua) -> c_int;
    fn root_size_mm(&self, lua: Lua) -> c_int;
    fn root_tags(&self, lua: Lua) -> c_int;
    fn root___index(&self, lua: Lua) -> c_int;
    fn root___newindex(&self, lua: Lua) -> c_int;
}
