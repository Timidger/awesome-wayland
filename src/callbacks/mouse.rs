//! Callbacks for the `Mouse` object in the Lua libraries

use ::lua::Lua;
use libc::c_int;

#[allow(non_snake_case)]
pub trait Mouse {
    /* Methods */
    fn mouse___index(&self, lua: &Lua) -> c_int;
    fn mouse___newindex(&self, lua: &Lua) -> c_int;
    fn mouse_coords(&self, lua: &Lua) -> c_int;
    fn mouse_object_under_pointer(&self, lua: &Lua) -> c_int;
    fn mouse_set_index_miss_handler(&self, lua: &Lua) -> c_int;
    fn mouse_set_newindex_miss_handler(&self, lua: &Lua) -> c_int;
}
