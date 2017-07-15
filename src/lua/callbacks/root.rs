//! Callbacks for the `Root` object in the Lua libraries

use ::lua::Lua;
use libc::c_int;

pub trait Root {
    /* Methods */
    fn buttons(&mut self, lua: Lua);
    fn keys(&mut self, lua: Lua);
    fn cursor(&mut self, lua: Lua);
    fn fake_input(&mut self, lua: Lua);
    fn drawins(&mut self, lua: Lua);
    fn wallpaper(&mut self, lua: Lua);
    fn size(&mut self, lua: Lua);
    fn size_mm(&mut self, lua: Lua);
    fn tags(&mut self, lua: Lua);
    fn __index(&mut self, lua: Lua);
    fn __newindex(&mut self, lua: Lua);
    /* Meta */
    /* Properties */
}
