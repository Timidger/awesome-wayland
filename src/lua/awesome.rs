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
///
/// The callbacks will only live as long as the `T`
/// (which is owned by `Awesome`), so once this struct is dropped the callbacks
/// are immediately removed from the Lua thread to ensure safety.
#[derive(Debug)]
pub struct Awesome<T>
    where T: callbacks::Awesome + Beautiful + Button +
    Client + Drawin + Keygrabber +
    Mousegrabber + Mouse + Root +
    Screen + Tag {
    /// The safe Lua wrapper
    lua: Lua,
    /// The user-provided data that is operated on by the callbacks.
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
    where T: Default + callbacks::Awesome + Beautiful + Button + Client +
    Drawin + Keygrabber + Mousegrabber + Mouse + Root + Screen + Tag {

    /// Constructs a new `Awesome` instance, and calls the default constructor
    /// for the `T` value.
    pub fn new() -> Self {
        let lua = Lua::new();
        let callbacks = T::default();
        Awesome{
            lua,
            callbacks
        }
    }
}

impl<T> Awesome<T>
    where T: callbacks::Awesome + Beautiful + Button + Client + Drawin +
    Keygrabber + Mousegrabber + Mouse + Root + Screen + Tag {

    /// Load the rc.lua configuration file from the specified path.
    pub fn load_configuration(&mut self, path: PathBuf)
                              -> Result<(), AwesomeErr> {
        Ok(self.lua.load_and_run(path)?)
    }
}
