//! Callbacks for the `keygrabber` object in the Lua libraries

use ::lua::Lua;
use libc::c_int;

#[allow(non_snake_case)]
pub trait Keygrabber {
    /* Methods */
    fn keygrabber_run(&self, lua: &Lua) -> c_int;
    fn keygrabber_stop(&self, lua: &Lua) -> c_int;
    fn keygrabber_isrunning(&self, lua: &Lua) -> c_int;
    fn keygrabber___index(&self, lua: &Lua) -> c_int;
    fn keygrabber___newindex(&self, lua: &Lua) -> c_int;
}
