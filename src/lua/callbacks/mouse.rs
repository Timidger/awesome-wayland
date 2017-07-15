//! Callbacks for the `Mouse` object in the Lua libraries

use ::lua::Lua;
use libc::c_int;

pub trait Mouse {
    /* Methods */
    fn __index(&mut self, lua: Lua);
    fn __newindex(&mut self, lua: Lua);
    fn coords(&mut self, lua: Lua);
    fn object_under_pointer(&mut self, lua: Lua);
    fn set_index_miss_handler(&mut self, lua: Lua);
    fn set_newindex_miss_handler(&mut self, lua: Lua);
    /* Meta */
    /* Properties */
}
