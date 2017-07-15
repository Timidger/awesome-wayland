//! Callbacks for the `keygrabber` object in the Lua libraries

use ::lua::Lua;
use libc::c_int;

pub trait Keygrabber {
    /* Methods */
    fn run(&mut self, lua: Lua);
    fn stop(&mut self, lua: Lua);
    fn isrunning(&mut self, lua: Lua);
    fn __index(&mut self, lua: Lua);
    fn __newindex(&mut self, lua: Lua);
}
