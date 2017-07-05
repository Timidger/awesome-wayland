//! Defines safe functions so that the rest of the library doesn't have to use
//! the raw Lua context directly.


pub mod callbacks;
mod awesome;
mod lua;

pub use self::lua::{Lua, LuaErr, ConfigErr};
pub use self::awesome::{Awesome};
pub use self::callbacks::*;
