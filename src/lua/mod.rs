//! Defines safe functions so that the rest of the library doesn't have to use
//! the raw Lua context directly.


pub mod callbacks;
mod awesome;
pub use self::awesome::{Awesome, LuaErr, ConfigErr};
