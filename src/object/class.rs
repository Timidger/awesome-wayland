//! These methods set up a interface to be a class for Lua.

use libc;
use lua_sys::*;
use super::signal::Signal;
use super::property::Property;

/// Method that allocates new objects for the class.
pub type AllocatorF = unsafe extern fn(*mut lua_State) -> *mut Object;
/// Method that is called when the object is garbage collected.
pub type CollectorF = unsafe fn(*mut Object);
/// Function to call when accessing a property in some way.
pub type PropF = unsafe fn(*mut lua_State, *mut Object) -> libc::c_int;
/// Function to call to check if an object is valid.
pub type CheckerF = unsafe fn(*mut Object) -> bool;

#[repr(C)]
pub struct Object {
    pub signals: Vec<Signal>
}

/// A Lua object that is a class.
#[repr(C)]
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
    pub index_miss_handler: libc::c_int,
    pub newindex_miss_handler: libc::c_int
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
