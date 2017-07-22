//! Defines safe functions so that the rest of the library doesn't have to use
//! the raw Lua context can be used directly.


// TODO Move macro up and out
#[macro_use] mod object;
pub mod callbacks;
mod awesome;
mod lua;

pub use self::object::{signal, property};
pub use self::object::class::{self, Object, Class, AllocatorF, CollectorF, PropF, CheckerF};
pub use self::lua::{Lua, LuaErr, FFIErr};
pub use self::awesome::{Awesome};
pub use self::callbacks::*;
