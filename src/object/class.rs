//! These methods set up a interface to be a class for Lua.

// TODO double check I need these c_* types

use libc::{self, c_int, c_uint};
use lua_sys::*;
use ::lua::Lua;
use super::signal::Signal;
use super::property::Property;
use std::cell::UnsafeCell;

/// Method that allocates new objects for the class.
pub type AllocatorF = unsafe extern fn(*mut lua_State) -> *mut Object;
/// Method that is called when the object is garbage collected.
pub type CollectorF = unsafe fn(*mut Object);
/// Function to call when accessing a property in some way.
pub type PropF = unsafe fn(*mut lua_State, *mut Object) -> c_int;
/// Function to call to check if an object is valid.
pub type CheckerF = unsafe fn(*mut Object) -> bool;

/// The super class to all [Class](Class)es.
///
/// These can be downcasted into a concrete class type if necessary.
/*pub trait Object: ::std::any::Any {
    fn signals(&self) -> Vec<Signal>;
}*/

#[repr(C)]
struct array_t<T> {
    tab: *mut T,
    len: libc::c_int,
    size: libc::c_int
}

pub struct Object {
    pub signals: Vec<Signal>
}

/// A Lua object that is a class.
pub struct Class {
    pub name: String,
    pub signals: Vec<Signal>,
    pub parent: *mut Class,
    /// Method that allocates new objects for the class.
    pub allocator: Option<AllocatorF>,
    /// Method that is called when the object is garbage collected.
    pub collector: Option<CollectorF>,
    pub properties: Vec<Property>,
    pub index_miss_prop: Option<PropF>,
    pub newindex_miss_prop: Option<PropF>,
    pub checker: Option<CheckerF>,
    pub instances: i32,
    pub tostring: Option<PropF>,
    // TODO Do we need these? These are pointers to methods on the stack
    // And are wildly unsafe. See how they are used first
    pub index_miss_handler: c_int,
    pub newindex_miss_handler: c_int
}

unsafe impl Send for Class {}
unsafe impl Sync for Class {}

impl Default for Class {
    fn default() -> Self {
        Class {
            name: String::new(),
            signals: Vec::new(),
            parent: 0 as _,
            allocator: None,
            collector: None,
            properties: Vec::new(),
            index_miss_prop: None,
            newindex_miss_prop: None,
            checker: None,
            instances: 0,
            tostring: None,
            index_miss_handler: 0,
            newindex_miss_handler: 0
        }
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
