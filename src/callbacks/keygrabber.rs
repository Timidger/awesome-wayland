//! Callbacks for the `keygrabber` object in the Lua libraries

use ::lua::Lua;
use libc::c_int;

pub trait Keygrabber {
    /* Methods */
    fn keygrabber_run(&mut self, lua: Lua) -> c_int;
    fn keygrabber_stop(&mut self, lua: Lua) -> c_int;
    fn keygrabber_isrunning(&mut self, lua: Lua) -> c_int;
    fn keygrabber___index(&mut self, lua: Lua) -> c_int;
    fn keygrabber___newindex(&mut self, lua: Lua) -> c_int;
}
