//! Callbacks for the `root` object in the Lua libraries

use ::lua::Lua;
use libc::c_int;

pub trait Root {
    /* Methods */
    fn root_buttons(&mut self, lua: Lua);
    fn root_keys(&mut self, lua: Lua);
    fn root_cursor(&mut self, lua: Lua);
    fn root_fake_input(&mut self, lua: Lua);
    fn root_drawins(&mut self, lua: Lua);
    fn root_wallpaper(&mut self, lua: Lua);
    fn root_size(&mut self, lua: Lua);
    fn root_size_mm(&mut self, lua: Lua);
    fn root_tags(&mut self, lua: Lua);
    fn root___index(&mut self, lua: Lua);
    fn root___newindex(&mut self, lua: Lua);
}
