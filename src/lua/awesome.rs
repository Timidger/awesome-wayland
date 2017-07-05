//! Defines the main structure to be used in this library.
//!
//! It wraps Lua, and ensures that the correct callbacks are defined for
//! each of the methods used by the Awesome Lua libraries.
use std::path::PathBuf;
use super::lua::{Lua, LuaErr};
use super::callbacks::{self, Beautiful, Button, Client, Drawin, Keygrabber,
                       Mousegrabber, Mouse, Root, Screen, Tag};

/// Represents the bindings to the awesome libraries.
/// Contains the raw Lua context, as well as the struct that has all of the
/// necessary callbacks defined that are called from Lua.
#[derive(Debug)]
pub struct Awesome<T>
    where T: callbacks::Awesome + Beautiful + Button +
    Client + Drawin + Keygrabber +
    Mousegrabber + Mouse + Root +
    Screen + Tag {
    /// The safe Lua wrapper
    lua: Lua,
    /// The user-provided data.
    /// It also provides the callbacks,
    callbacks: T
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AwesomeErr {
    Lua(LuaErr)
}

impl From<LuaErr> for AwesomeErr {
    fn from(err: LuaErr) -> Self {
        AwesomeErr::Lua(err)
    }
}

impl<T> Awesome<T>
    where T: callbacks::Awesome + Beautiful + Button +
    Client + Drawin + Keygrabber +
    Mousegrabber + Mouse + Root +
    Screen + Tag {
    pub fn new(callbacks: T) -> Self {
        let lua = Lua::new();
        Awesome{
            lua,
            callbacks
        }
    }

    /// Load the rc.lua configuration file from the specified path.
    pub fn load_configuration(&mut self, path: PathBuf)
                              -> Result<(), AwesomeErr> {
        Ok(self.lua.load_and_run(path)?)
    }
}
