//! Wrapper around the raw Lua context. These define safe methods that ensure
//! correct use of the Lua stack.

use lua_sys::*;
use libc;
use std::path::PathBuf;
use std::ffi::{CString, CStr};
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::sync::{Arc, Mutex};
use super::{Object, Class, AllocatorF, CollectorF, PropF, CheckerF, Property, Signal};

const ALREADY_DEFINED: i32 = 0;

/// Wrapper around the raw Lua context. When necessary, the raw Lua context can
/// be retrived.
#[derive(Debug)]
pub struct Lua(pub *mut lua_State);

unsafe impl Send for Lua {}
unsafe impl Sync for Lua {}

/// Errors while interacting with Lua
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LuaErr {
    /// There was an error loading the configuration file
    Load(FFIErr),
    /// Evaluation error from Lua
    Eval(String),
    /// There was an FFI error during evalution
    EvalFFI(FFIErr),
    /// There was an error loading in arguments from the Lua call
    ArgumentInvalid(&'static str),
    /// A variable was already defined.
    AlreadyDefined(String),
    /// Could not find configuration file with the given path.
    /// Reason given from Lua as a string.
    FileNotFound(PathBuf, String)
}

/// Errors while interfacing with C
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FFIErr {
    /// String had invalid UTF-8 encoding.
    InvalidUTF(String),
    /// String contained a null byte.
    NullByte(String),
    /// Could not convert from C string to UTF-8 encoded one
    Conversion(CString)
}

impl Lua {
    pub fn new() -> Self {
        unsafe {
            let lua = luaL_newstate();
            if lua.is_null() {
                panic!("luaL_newstate returned NULL");
            }
            luaL_openlibs(lua);
            Lua(lua)
        }
    }

    /// Constructs the Lua object from an already initialized Lua context.
    /// # Safety
    /// You should not use the passed in Lua state after this
    /// except through the `Lua` interface (and that includes
    /// using the raw pointer directly)
    pub unsafe fn from_ptr(lua: *mut lua_State) -> Self {
        Lua(lua)
    }

    /// Loads and runs the Lua file that the path points to.
    pub fn load_and_run(&self, path: PathBuf) -> Result<(), LuaErr> {
        let path_str = path.to_str()
            .ok_or_else(||
                        LuaErr::Load(FFIErr::InvalidUTF(path.to_str().unwrap().into())))
            .and_then(|s| CString::new(s)
                      .map_err(|_|
                               LuaErr::Load(FFIErr::NullByte(path.to_str().unwrap().into()))))?;
        unsafe {
            let lua = &mut *self.0;
            let mut status = luaL_loadfile(lua, path_str.as_ptr());
            if status != 0 {
                // If something went wrong, error message is at the top of
                // the stack.
                let error = lua_tostring(lua, -1);
                let error = CStr::from_ptr(error).to_string_lossy().into_owned();

                Err(LuaErr::FileNotFound(path.clone(), error))?
            }

            // Run configuration file
            status = lua_pcallk(lua, 0, LUA_MULTRET, 0, 0, None);
            if status != 0 {
                let error = lua_tostring(lua, -1);
                let error = CStr::from_ptr(error).to_string_lossy().into_owned();

                Err(LuaErr::Eval(error))?
            }
        }
        Ok(())
    }

    /// Registers an object to be a class, with the given methods and
    /// allocator, collector, etc.
    pub fn register_class(&self,
                          class: &Mutex<Class>,
                          name: &'static str,
                          parent: Option<Arc<Class>>,
                          allocator: Option<AllocatorF>,
                          collector: Option<CollectorF>,
                          checker: Option<CheckerF>,
                          index_miss_property: Option<PropF>,
                          newindex_miss_property: Option<PropF>,
                          methods: &[luaL_Reg],
                          meta: &[luaL_Reg])
                          -> Result<(), LuaErr> {
        let mut class_lock = class.lock().expect("Could not lock class mutex");
        unsafe {
            let lua = self.0;
            let class_ptr = (&mut *class_lock) as *mut _ as *mut _;
            // Create object metatable
            lua_newtable(lua);
            // Register it with class pointer as key in the registry
            // e.g class-pointer -> metatable
            lua_pushlightuserdata(lua, class_ptr);
            // Duplicate the object metatable
            lua_pushvalue(lua, -2);
            lua_rawset(lua, LUA_REGISTRYINDEX);
            // Register class pointer with metatable as key in the registery
            // e.g: metatable -> class-pointer
            lua_pushvalue(lua, -1);
            lua_pushlightuserdata(lua, class_ptr);
            lua_rawset(lua, LUA_REGISTRYINDEX);

            // Duplicate object's metatable
            lua_pushvalue(lua, -1);
            // Set garbage collector in the meta table
            lua_pushcfunction(lua, Some(lua_class_gc));
            lua_setfield(lua, -2, c_str!("__gc"));
            // metatable.__index = metatable
            lua_setfield(lua, -2, c_str!("__index"));

            // Register meta
            //luaL_setfuncs(lua, meta.as_ptr(), 0);

            // Register methods
            lua_newtable(lua);
            luaL_setfuncs(lua, methods.as_ptr(), 0);
            lua_pushvalue(lua, -1);
            let c_name = CStr::from_bytes_with_nul(name.as_bytes())
                .map_err(|_| LuaErr::EvalFFI(FFIErr::NullByte(name.into())))?;
            lua_setglobal(lua, c_name.as_ptr());

            // dup self as metatable
            lua_pushvalue(lua, -1);
            // set self as metatable
            lua_setmetatable(lua, -2);
            lua_pop(lua, 2);

            class_lock.collector = collector;
            class_lock.allocator = allocator;
            class_lock.name = name.to_owned();
            class_lock.index_miss_property = index_miss_property;
            class_lock.newindex_miss_property = newindex_miss_property;
            class_lock.checker = checker;
            class_lock.parent = parent;
            class_lock.tostring = None;
            class_lock.instances = 0;
            class_lock.index_miss_handler = LUA_REFNIL;
            class_lock.newindex_miss_handler = LUA_REFNIL;

            // TODO remove
            let signals = &mut class_lock.signals;
            let mut hasher = DefaultHasher::new();
            hasher.write("new".as_bytes());
            let id = hasher.finish();
            let sigfuncs = vec!((&*::std::ptr::null_mut()) as _);
            signals.push(Signal {
                id, sigfuncs
            });

            // TODO Update global list of classes
        }
        Ok(())
    }

    /// Registers the methods in the array to the given variable name.
    ///
    /// Automatically sets up the table, if you want the methods bound to a
    /// class instead, use `register_class`.
    ///
    /// The requirement for the name to be static is to ensure that memory
    /// does not leak. The mechanism to ensure that names can be dynamically
    /// allocated is not available at this time.
    pub fn register_methods(&self, name: &'static str, methods: &[luaL_Reg])
                            -> Result<(), LuaErr> {
        unsafe {
            let l = self.0;
            // NOTE: This is safe because we guarentee that name is static
            let c_name = CStr::from_bytes_with_nul(name.as_bytes())
                .map_err(|_| LuaErr::EvalFFI(FFIErr::NullByte(name.into())))?;
            let result = luaL_newmetatable(l, c_name.as_ptr());
            if result == ALREADY_DEFINED {
                // variable is still pushed to the stack
                lua_pop(l, 1);
                return Err(LuaErr::AlreadyDefined(name.into()))
            }
            /* Set __index to be the metatable */
            // move meta table to top of stack
            lua_pushvalue(l, -1);
            // Set the __index to be the metatable
            // NOTE Pops the value from the stack
            lua_setfield(l, -2, c_str!("__index"));

            /* Add the methods to the table */
            lua_newtable(l);
            luaL_setfuncs(l, methods.as_ptr(), 0);
            // TODO This is only valid for versions >= 5.2.
            // For old versions, there is a different way
            // and this should support that.
            // <= 5.1 code it is just: luaL_register
            lua_pushvalue(l, -1);
            // NOTE Pops the value from the stack
            lua_setglobal(l, c_name.as_ptr());

            /* Set "self" to be the metatable */
            lua_pushvalue(l, -1);
            lua_setmetatable(l, -2);

            // Pop the table we made, as well as the metatable
            lua_pop(l, 2);
        }
        Ok(())
    }

    /// Loads the library given by `lib_path` and stores the result
    /// in a global named `name`.
    pub fn load_library(&self, name: &str, lib_path: PathBuf)
                        -> Result<(), LuaErr> {
        unsafe {
            let c_name = CString::new(name)
                .expect("Name contained null bytes");
            self.load_and_run(lib_path)?;
            println!("Setting global to {:#?}", c_name);
            lua_setglobal(self.0, c_name.as_ptr());
            // TODO needed?
            ::std::mem::forget(c_name);
        }
        Ok(())
    }

    /// Adds a variable number of paths to the lookup path within lua.
    /// These lookup path is used to find libraries.
    ///
    /// Note that these values are appended on to the current lookup path.
    pub fn add_lib_lookup_path(&self, paths: &[PathBuf]) -> Result<(), LuaErr> {
        let len = paths.len();
        unsafe {
            let lua = self.0;
            // Push these on to the stack
            lua_getglobal(lua, c_str!("package"));
            lua_getfield(lua, 1, c_str!("path"));
            for path in paths {
                let c_path = path.to_str()
                    .ok_or_else(||
                                LuaErr::EvalFFI(FFIErr::InvalidUTF(path.to_str().unwrap().into())))
                    .and_then(|s| CString::new(s)
                              .map_err(|_|
                                       LuaErr::EvalFFI(FFIErr::NullByte(path.to_str().unwrap().into()))))?;
                lua_pushfstring(lua, c_path.as_ptr());
                // TODO needed?
                ::std::mem::forget(c_path);
            }
            // concatenate with thing on top of the stack when we were called
            // + 1 because we want to include the path.
            lua_concat(lua, (len + 1) as i32);
            // Now set the path to that value
            lua_setfield(lua, 1, c_str!("path"));
            // pop "package"
            lua_pop(lua, 1);
        }
        Ok(())
    }

    /// Sets up the lookup path to include the default awesoem libs.
    /// These libraries are found in `/usr/share/awesome/lib`.
    ///
    /// Note that these values are added to the current lookup path,
    /// so if you want them to take precedence call this method earlier than
    /// other methods that modify the path
    /// (e.g [add_lib_lookup_path](add_lib_lookup_path)).
    pub fn add_default_awesome_libs(&self) {
        self.add_lib_lookup_path(&[";/usr/share/awesome/lib/?.lua;".into(),
                                   ";/usr/share/awesome/lib/?/init.lua;".into()
        ]);
    }

    /// Generic constructor function for objects. Places the constructed class
    /// into the mutex.
    ///
    /// Constructs a new class. The returned class should be immediantly moved
    /// to a global mutex.
    ///
    /// On success, the number of arguments that will be returned to Lua on its
    /// stack is returned as the `Ok` value.
    pub fn new_class(&self, class: &Mutex<Class>) -> Result<libc::c_int, LuaErr>{
        let mut class_lock = class.lock()
            .expect("Could not lock class");
        unsafe {
            let lua = self.0;
            // TODO Ensure that the value passed in is a table,
            // no way to do that currently with lua_sys :(
            let allocator = class_lock.allocator
                .ok_or(LuaErr::ArgumentInvalid("Global class was not defined"))?;
            let mut obj = allocator(self, &mut *class_lock);

            // push first key before iterating
            lua_pushnil(lua);
            // iterate over the property keys
            while (lua_next(lua, -2)) != 0 {
                /* Check that the key is a string.
                 * We cannot call tostring blindly or Lua will convert a key that is a
                 * number TO A STRING, confusing lua_next() */
                if lua_isstring(lua, 1) != 0 {
                    let prop_name = luaL_checklstring(lua, 1, ::std::ptr::null_mut());
                    if !prop_name.is_null() {
                        let prop_name = CStr::from_ptr(prop_name).to_str()
                            .expect("Property name could not be converted");
                        let prop = self.class_get_property(&*class_lock, prop_name);
                        if let Some(new_f) = prop.map(|prop| prop.new) {
                            new_f(self, &mut *obj);
                        }
                    }
                }
                // Remove value
                lua_pop(lua, 1);
            }
        }
        Ok(1)
    }


    // TODO move?
    /// Get a property of an object, if it exists.
    /// If not found in the object, then its parent classes (if any)
    /// are checked as well.
    pub fn class_get_property<'class>(&self, class: &'class Class, property_name: &str)
                              -> Option<&'class Property> {
        // TODO Look up parent at parent class to see if the property exists there
        for prop in &class.properties {
            if prop.name == property_name {
                return Some(prop)
            }
        }
        None
    }

    /// Gets the argument from indexing a lua table.
    /// This argument is always the second one.
    pub fn get_index_arg_string(&self) -> Result<String, LuaErr> {
        unsafe {
            let lua = self.0;
            let buf = luaL_checklstring(lua, 2, ::std::ptr::null_mut());
            if buf.is_null() {
                return Err(LuaErr::ArgumentInvalid("Argument was not a string"))
            }
            let c_str = CStr::from_ptr(buf);
            Ok(c_str.to_owned().into_string()
               .map_err(|err|
                        LuaErr::EvalFFI(FFIErr::Conversion(err.into_cstring())))?)

        }
    }

    pub fn return_table<T>(&self, table: HashMap<String, T>) {
        unsafe {
            let lua = self.0;
            lua_newtable(lua);
            // TODO Push values to table
        }
    }

    pub fn return_string<S: Into<String>>(&self, string: S)
                                          -> Result<(), LuaErr> {
        let return_val = string.into();
        unsafe {
            let lua = self.0;
            let c_str = CString::new(return_val.clone())
                .map_err(|_| LuaErr::EvalFFI(FFIErr::NullByte(return_val)))?;
            lua_pushstring(lua, c_str.as_ptr());
            // TODO Needed?
            ::std::mem::forget(c_str);
        }
        Ok(())
    }
}

impl Deref for Lua {
    type Target = lua_State;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.0
        }
    }
}

impl DerefMut for Lua {
    fn deref_mut(&mut self) -> &mut lua_State {
        unsafe {
            &mut *self.0
        }
    }
}

unsafe extern "C" fn lua_class_gc(lua: *mut lua_State) -> libc::c_int {
    // TODO Implement
    unimplemented!()
}
