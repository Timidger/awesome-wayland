//! These methods set up a interface to be a class for Lua.

// TODO double check I need these c_* types

use libc::{c_int, c_uint};
use lua_sys::*;
use std::sync::{Arc, MutexGuard};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use ::lua::Lua;
use super::signal::Signal;
use super::property::Property;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SignalErr {
    /// A signal was not found when searching for the provided string
    NotFound(String)
}

/// Method that allocates new objects for the class.
pub type AllocatorF = unsafe extern fn(*mut lua_State) -> c_int;
/// Method that is called when the object is garbage collected.
pub type CollectorF = fn(&mut Object);
/// Function to call when accessing a property in some way.
pub type PropF = fn(&Lua, &mut Object) -> c_int;
/// Function to call to check if an object is valid.
pub type CheckerF = fn(&mut Object) -> bool;

/// The super class to all [Class](Class)es.
///
/// These can be downcasted into a concrete class type if necessary.
pub trait Object: ::std::any::Any {
    fn signals(&self) -> &[Signal];
}

/// A Lua object that is a class.
pub struct Class {
    pub name: String,
    pub signals: Vec<Signal>,
    pub parent: Option<Arc<Class>>,
    /// Method that allocates new objects for the class.
    pub allocator: Option<AllocatorF>,
    /// Method that is called when the object is garbage collected.
    pub collector: Option<CollectorF>,
    pub checker: Option<CheckerF>,
    pub properties: Vec<Property>,
    pub index_miss_property: Option<PropF>,
    pub newindex_miss_property: Option<PropF>,
    pub instances: i32,
    pub tostring: Option<PropF>,
    // TODO Do we need these? These are pointers to methods on the stack
    // And are wildly unsafe. See how they are used first
    pub index_miss_handler: c_int,
    pub newindex_miss_handler: c_int
}

impl Class {
    /// Connects a lua function to the given name.
    ///
    /// If a signal already exists with the given name, the function is
    /// appended to the list of functions to call.
    pub fn connect_signal(&mut self, name: &str, func: lua_CFunction) {
        let mut hasher = DefaultHasher::new();
        hasher.write(name.as_bytes());
        let id = hasher.finish();
        if let Some(mut sig) = self.signals.iter_mut().find(|sig| sig.id == id) {
            sig.sigfuncs.push(func);
            return;
        }
        self.signals.push(Signal {
            id,
            sigfuncs: vec![func]
        });
    }

    pub fn emit_signal(&self, lua: &Lua, name: &str, nargs: i32) {
        
    }

    /// Pushes the functions associated with `name` on to the Lua stack.
    /// Note that this is unsafe, because if you don't check that it succeeded
    /// (E.g returned `Ok(())`) and then try to use Lua like the function is
    /// on the stack, you're going to have a bad time.
    ///
    /// As well, even if it succeeds, the caller is responsible to pushing the
    /// required arguments on to the stack.
    pub unsafe fn push_signal(&self, lua: *mut lua_State, name: &str)
                              -> Result<(), SignalErr> {
        let mut hasher = DefaultHasher::new();
        hasher.write(name.as_bytes());
        let id = hasher.finish();
        if let Some(mut sig) = self.signals.iter().find(|sig| sig.id == id) {
            for func in &sig.sigfuncs {
                lua_pushcfunction(lua, *func);
            }
            return Ok(());
        }
        Err(SignalErr::NotFound(name.into()))
    }
}

/// Get an object lua_class
/// l: The lua state
/// idx: The index of the object on the stack
///
/// # SAFETY
/// It's not guaranteed that the index is valid.
/// The lifetime is also not bounded, this might eventually be fixed.
pub unsafe fn class_get<'a>(l: *mut lua_State, idx: c_int) -> Option<&'a Class> {
    let ty = lua_type(l, idx);
    if ty == LUA_TUSERDATA as i32 && lua_getmetatable(l, idx) != 0 {
        /* Use the metatable has key to get the class from the registry */
        lua_rawget(l, LUA_REGISTRYINDEX);
        let class_raw = lua_touserdata(l, -1);
        let class = &*(class_raw as *mut Class);
        lua_pop(l, 1);
        return Some(class);
    }
    None
}

pub unsafe fn usemetatable(l: *mut lua_State, idxobj: c_int, idxfield: c_int) -> bool {
    unimplemented!()
}

impl ::std::default::Default for Class {
    fn default() -> Self {
        Class {
            name: String::new(),
            signals: Vec::new(),
            parent: None,
            allocator: None,
            collector: None,
            checker: None,
            properties: Vec::new(),
            index_miss_property: None,
            newindex_miss_property: None,
            instances: 0,
            tostring: None,
            index_miss_handler: 0,
            newindex_miss_handler: 0
        }
    }
}
