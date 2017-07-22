//! Callbacks for the `Mousegrabber` object in the Lua libraries

use ::lua::Lua;
use libc::c_int;

pub trait Mousegrabber {
    /* Methods */
    fn mousegrabber_run(&mut self, lua: Lua);
    fn mousegrabber_stop(&mut self, lua: Lua);
    fn mousegrabber_isrunning(&mut self, lua: Lua);
    fn mousegrabber___index(&mut self, lua: Lua);
    fn mousegrabber___newindex(&mut self, lua: Lua);
}
