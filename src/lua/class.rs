//! These methods set up a interface to be a class for Lua.

// TODO double check I need these c_* types

use libc::{c_int, c_uint};
use lua_sys::*;
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
    // TODO Objects need to be able to return signals
    //fn signals() -> Vec<Signal>;
}

/// A Lua object that is a class.
pub struct Class {
    name: String,
    signals: Vec<Signal>,
    // TODO Something better than a pointer...
    parent: *mut Class,
    /// Method that allocates new objects for the class.
    allocator: AllocatorF,
    /// Method that is called when the object is garbage collected.
    collector: CollectorF,
    properties: Vec<Property>,
    // TODO These will be func pointers, get better name
    index_miss_prop: PropF,
    newindex_miss_prop: PropF,
    checker: CheckerF,
    // TODO Do these need to be c_int/c_uint?
    instances: c_uint,
    tostring: PropF,
    // TODO Do we need these?
    index_miss_handler: c_int,
    newindex_miss_handler: c_int
}

/// Defines the methods associated with classes. These methods have default
/// implementations, but can be defined by the user if they so choose.
macro_rules! class_methods {
    () => {
        // TODO Give these the default impls
        /* LUA_CLASS_META methods */
        fn __index(&mut self, awesome: Lua) -> c_int {
            unsafe {
                let l = awesome.0;
                /* Try to use the metatable first. */
                if ::lua::class::usemetatable(l, 1, 2) {
                    return 1
                }
                let class = ::lua::class::class_get(l, 1);
            }
            0
        }
        fn __newindex(&mut self, awesome: Lua) -> c_int {
            if ::lua::class::usemetatable(l, 1, 2) {
                return 1
            }
            let class_raw = class_get(l, 1);
            if let Some(prop) = class_property_get(l, class, 2) {
                if prop.newindex != 0 {
                    return prop.newindex(l, checkudata(l, 1, class))
                }
            } else {
                // Property didn't exist
                if class.newindex_miss_handler != LUA_REFNIL {
                    return class_call_handler(l, class.newindex_miss_handler);
                }
                if class.newindex_miss_property {
                    return class.newindex_miss_property(l, checkudata(l, 1, class))
                }
            }
            0
        }

        /* LUA_OBJECT_META methods */
        fn __tostring(&mut self, awesome: Lua);
        fn connect_signal(&mut self, awesome: Lua);
        fn disconnect_signal(&mut self, awesome: Lua);
        fn emit_signal(&mut self, awesome: Lua);
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
