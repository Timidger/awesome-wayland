//! Callbacks for the `Mouse` object in the Lua libraries

use ::lua::Lua;
use libc::c_int;

pub trait Mouse {
    /* Methods */
    fn mouse___index(&mut self, lua: Lua);
    fn mouse___newindex(&mut self, lua: Lua);
    fn mouse_coords(&mut self, lua: Lua);
    fn mouse_object_under_pointer(&mut self, lua: Lua);
    fn mouse_set_index_miss_handler(&mut self, lua: Lua);
    fn mouse_set_newindex_miss_handler(&mut self, lua: Lua);
}
