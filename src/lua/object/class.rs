//! These methods set up a interface to be a class for Lua.

// TODO double check I need these c_* types

use libc::{c_int, c_uint};
use lua_sys::*;
use std::rc::Rc;
use ::lua::Lua;
use super::signal::Signal;
use super::property::Property;

/// Method that allocates new objects for the class.
pub type AllocatorF = fn(&mut Lua) -> Object;
/// Method that is called when the object is garbage collected.
pub type CollectorF = fn(&mut Object);
/// Function to call when accessing a property in some way.
pub type PropF = fn(&mut Lua, &mut Object) -> c_int;
/// Function to call to check if an object is valid.
pub type CheckerF = fn(&mut Object) -> bool;

/// The super class to all [Class](Class)es.
///
/// These can be downcasted into a concrete class type if necessary.
trait Object: ::std::any::Any {
    fn signals(&self) -> Vec<Signal>;
}

/// A Lua object that is a class.
pub struct Class {
    name: String,
    signals: Vec<Signal>,
    // TODO Putting it an Rc, cause idk what else to put it in
    parent: Rc<Class>,
    /// Method that allocates new objects for the class.
    allocator: AllocatorF,
    /// Method that is called when the object is garbage collected.
    collector: CollectorF,
    properties: Vec<Property>,
    index_miss_prop: PropF,
    newindex_miss_prop: PropF,
    checker: CheckerF,
    instances: i32,
    tostring: PropF,
    // TODO Do we need these? These are pointers to methods on the stack
    // And are wildly unsafe. See how they are used first
    index_miss_handler: c_int,
    newindex_miss_handler: c_int
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
