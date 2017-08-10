//! Callbacks for the `Mousegrabber` object in the Lua libraries

use ::lua::Lua;
use libc::c_int;

pub trait Mousegrabber {
    /* Methods */
    fn mousegrabber_run(&self, lua: Lua) -> c_int;
    fn mousegrabber_stop(&self, lua: Lua) -> c_int;
    fn mousegrabber_isrunning(&self, lua: Lua) -> c_int;
    fn mousegrabber___index(&self, lua: Lua) -> c_int;
    fn mousegrabber___newindex(&self, lua: Lua) -> c_int;
}
