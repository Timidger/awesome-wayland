//! Wrapper around the raw Lua context. These define safe methods that ensure
//! correct use of the Lua stack.

use lua_sys::*;
use std::path::PathBuf;
use std::ffi::{CString, CStr};
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;


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
    ArgumentInvalid,
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
                        LuaErr::Load(FFIErr::InvalidUTF(format!("{:?}", path))))
            .and_then(|s| CString::new(s)
                      .map_err(|_|
                               LuaErr::Load(FFIErr::NullByte(format!("{:?}", path)))))?;
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

            // Run the file
            status = lua_pcallk(lua, 0, LUA_MULTRET, 0, 0, None);
            if status != 0 {
                let error = lua_tostring(lua, -1);
                let error = CStr::from_ptr(error).to_string_lossy().into_owned();

                Err(LuaErr::Eval(error))?
            }
        }
        Ok(())
    }

    /// Registers the methods in the array to the given variable name.
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

    /// Gets the argument from indexing a lua table.
    /// This argument is always the second one.
    pub fn get_index_arg_string(&self) -> Result<String, LuaErr> {
        unsafe {
            let lua = self.0;
            let buf = luaL_checklstring(lua, 2, ::std::ptr::null_mut());
            if buf.is_null() {
                return Err(LuaErr::ArgumentInvalid)
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

/// These are methods that are straight Rust-version copies of their equivalents
/// defined in the Awesome library.
///
/// They should not be used directly and instead you should use [Lua](./Lua).
pub mod luaA {
    use lua_sys::*;
    use libc;
    use std::ffi::CString;
    use std::collections::LinkedList;
    use std::sync::Mutex;
    use ::object::Property;
    use ::object::class::{Class, Object, AllocatorF, CheckerF, CollectorF,
                          PropF};
    // This weird line is so that I can use luaA namespace explicitly here.
    use super::luaA;

    // Global button class definitions
    pub static mut button_class: *mut Class = 0 as *mut _;

    pub struct ClassWrapper(*mut Class);

    unsafe impl Send for ClassWrapper {}
    unsafe impl Sync for ClassWrapper {}

    impl ClassWrapper {
        pub fn new(class: *mut Class) -> Self {
            ClassWrapper(class)
        }
    }

    lazy_static! {
        pub static ref classes: Mutex<LinkedList<ClassWrapper>> =
            Mutex::new(LinkedList::new());
    }

    // TODO move this somewhere else...
    #[repr(C)]
    pub struct area_t {
        x: i16,
        y: i16,
        width: u16,
        height: u16
    }
    pub unsafe fn openlib(lua: *mut lua_State, name: *const libc::c_char,
                          methods: &[luaL_Reg], meta: &[luaL_Reg]) {
        luaL_newmetatable(lua, name);
        lua_pushvalue(lua, -1); // dup meta table
        lua_setfield(lua, -2, c_str!("__index")); // metatable.__index = metatable

        luaA::registerlib(lua, ::std::ptr::null_mut(), meta);
        luaA::registerlib(lua, name, methods);
        lua_pushvalue(lua, -1); // dup self as meta table
        lua_setmetatable(lua, -2); // set self as meta table
        lua_pop(lua, 2);
    }

    pub unsafe fn registerlib(lua: *mut lua_State, libname: *const libc::c_char,
                              l: &[luaL_Reg]) {
        if ! libname.is_null() {
            lua_newtable(lua);
            luaL_setfuncs(lua, l.as_ptr(), 0);
            lua_pushvalue(lua, -1);
            lua_setglobal(lua, libname);
        } else {
            luaL_setfuncs(lua, l.as_ptr(), 0);
        }
    }

    pub unsafe fn typeerror(lua: *mut lua_State, narg: libc::c_int,
                            tname: *const libc::c_char) -> libc::c_int {
        let msg = lua_pushfstring(lua, c_str!("%s expected, got %s"),
                                  tname, lua_typename(lua, narg));
        luaL_traceback(lua, lua, ::std::ptr::null_mut(), 2);
        lua_concat(lua, 2);
        return luaL_argerror(lua, narg, msg);
    }

    pub unsafe fn rangeerror(lua: *mut lua_State, narg: libc::c_int,
                             min: libc::c_double, max: libc::c_double)
                             -> libc::c_int {
        let msg = lua_pushfstring(lua, c_str!("value in [%f, %f] expected, got %f"),
                                  min, max, lua_tonumber(lua, narg) as libc::c_double);
        luaL_traceback(lua, lua, ::std::ptr::null_mut(), 2);
        lua_concat(lua, 2);
        return luaL_argerror(lua, narg, msg);
    }

    pub unsafe fn getuservalue(lua: *mut lua_State, idx: libc::c_int) {
        lua_getuservalue(lua, idx);
    }

    pub unsafe fn setuservalue(lua: *mut lua_State, idx: libc::c_int) {
        lua_setuservalue(lua, idx);
    }

    pub unsafe fn rawlen(lua: *mut lua_State, idx: libc::c_int) -> libc::size_t {
        return lua_rawlen(lua, idx) as libc::size_t ;
    }

    pub unsafe fn checkfunction(lua: *mut lua_State, idx: libc::c_int) {
        let lua_t = lua_type(lua, idx);
        let is_function = lua_t == LUA_TFUNCTION as i32;
        if ! is_function {
            luaA::typeerror(lua, idx, c_str!("function"));
        }
    }

    pub unsafe fn checkboolean(lua: *mut lua_State, n: libc::c_int) -> libc::c_int {
        if lua_type(lua, n) != LUA_TBOOLEAN as i32 {
            luaA::typeerror(lua, n, c_str!("boolean"));
        }
        return lua_toboolean(lua, n);
    }

    pub unsafe fn getopt_number(lua: *mut lua_State, idx: libc::c_int,
                                name: *const libc::c_char, mut def: lua_Number)
                                -> lua_Number {
        lua_getfield(lua, idx, name);
        let is_nil = lua_type(lua, -1) == LUA_TNIL as i32;
        let is_num = lua_isnumber(lua, -1) != 0;
        if is_nil || is_num {
            def = luaL_optnumber(lua, -1, def);
        }
        lua_pop(lua, 1);
        return def;
    }

    pub unsafe fn checknumber_range(lua: *mut lua_State, n: libc::c_int,
                                    min: lua_Number, max: lua_Number)
                                    -> lua_Number {
        let result = lua_tonumber(lua, n);
        if result < min || result > max {
            luaA::rangeerror(lua, n, min, max);
        }
        return result;
    }

    pub unsafe fn optnumber_range(lua: *mut lua_State, narg: libc::c_int,
                                  def: lua_Number, min: lua_Number,
                                  max: lua_Number) -> lua_Number {
        let lua_t = lua_type(lua, narg);
        let is_none_or_nil = lua_t == LUA_TNIL as i32 || lua_t == LUA_TNONE;
        if is_none_or_nil {
            return def;
        }
        return luaA::checknumber_range(lua, narg, min, max);
    }

    pub unsafe fn getopt_number_range(lua: *mut lua_State, idx: libc::c_int,
                                      name: *const libc::c_char,
                                      mut def: lua_Number, min: lua_Number,
                                      max: lua_Number) -> lua_Number {
        lua_getfield(lua, idx, name);
        let is_nil = lua_type(lua, -1) == LUA_TNIL as i32;
        let is_number = lua_isnumber(lua, -1) != 0;
        if is_nil || is_number {
            def = luaA::optnumber_range(lua, -1, def, min, max);
        }
        lua_pop(lua, 1);
        return def;
    }

    pub unsafe fn checkinteger(lua: *mut lua_State, n: libc::c_int) -> libc::c_int {
        let d = lua_tonumber(lua, n);
        let need_to_round = d != (d as libc::c_int as lua_Number);
        if need_to_round {
            luaA::typeerror(lua, n, c_str!("integer"));
        }
        return d as libc::c_int;
    }

    pub unsafe fn optinteger(lua: *mut lua_State, narg: libc::c_int, def: lua_Integer)
                             -> lua_Integer {
        return ::utils::luaL_opt(lua, |lua, n| luaA::checkinteger(lua, n) as lua_Integer, narg, def);
    }

    pub unsafe fn getopt_integer(lua: *mut lua_State, idx: libc::c_int,
                                 name: *const libc::c_char,
                                 mut def: lua_Integer) -> libc::c_int {
        lua_getfield(lua, idx, name);
        let lua_t = lua_type(lua, -1);
        let is_nil = lua_t == LUA_TNIL as i32;
        let is_number = lua_t == LUA_TNUMBER as i32;
        if is_nil || is_number {
            def = luaA::optinteger(lua, -1, def);
        }
        lua_pop(lua, 1);
        return def as libc::c_int;
    }

    pub unsafe fn checkinteger_range(lua: *mut lua_State, n: libc::c_int,
                                     min: lua_Number, max: lua_Number)
                                     -> libc::c_int {
        let result = luaA::checkinteger(lua, n) as lua_Number;
        if result < min || result > max {
            luaA::rangeerror(lua, n, min, max);
        }
        return result as libc::c_int;
    }

    pub unsafe fn optinteger_range(lua: *mut lua_State, narg: libc::c_int,
                                   def: lua_Integer, min: lua_Number,
                                   max: lua_Number) -> lua_Integer {
        let lua_t = lua_type(lua, narg);
        let is_none_or_nil = lua_t == LUA_TNIL as i32 || lua_t == LUA_TNONE;
        if is_none_or_nil {
            return def;
        }
        return luaA::checkinteger_range(lua, narg, min, max) as lua_Integer;
    }

    pub unsafe fn getopt_integer_range(lua: *mut lua_State, idx: libc::c_int,
                                       name: *const libc::c_char,
                                       mut def: lua_Integer,
                                       min: lua_Number, max: lua_Number)
                                       -> libc::c_int {
        lua_getfield(lua, idx, name);
        let lua_t = lua_type(lua, -1);
        let is_nil = lua_t == LUA_TNIL as i32;
        let is_number = lua_t == LUA_TNUMBER as i32;
        if is_nil || is_number {
            def = luaA::optinteger_range(lua, -1, def, min, max);
        }
        lua_pop(lua, 1);
        return def as libc::c_int;
    }

    pub unsafe fn pusharea(lua: *mut lua_State, geo: area_t) -> libc::c_int {
        lua_createtable(lua, 0, 4);
        lua_pushinteger(lua, geo.x as lua_Integer);
        lua_setfield(lua, -2, c_str!("x"));
        lua_pushinteger(lua, geo.y as lua_Integer);
        lua_setfield(lua, -2, c_str!("y"));
        lua_pushinteger(lua, geo.width as lua_Integer);
        lua_setfield(lua, -2, c_str!("width"));
        lua_pushinteger(lua, geo.height as lua_Integer);
        lua_setfield(lua, -2, c_str!("height"));
        return 1;
    }

    pub unsafe fn register(lua: *mut lua_State, idx: libc::c_int,
                           ptr: *mut libc::c_int) -> libc::c_int {
        lua_pushvalue(lua, idx);
        if *ptr != LUA_REFNIL {
            luaL_unref(lua, LUA_REGISTRYINDEX, *ptr);
        }
        *ptr = luaL_ref(lua, LUA_REGISTRYINDEX);
        return 0;
    }

    pub unsafe fn unregister(lua: *mut lua_State, ptr: *mut libc::c_int) {
        luaL_unref(lua, LUA_REGISTRYINDEX, *ptr);
        *ptr = LUA_REFNIL;
    }

    pub unsafe fn registerfct(lua: *mut lua_State, idx: libc::c_int,
                              fct: *mut libc::c_int) -> libc::c_int {
        luaA::checkfunction(lua, idx);
        return luaA::register(lua, idx, fct);
    }

    pub unsafe fn default_index(lua: *mut lua_State) -> libc::c_int {
        return luaA::class_index_miss_property(lua, ::std::ptr::null_mut());
    }

    pub unsafe fn default_newindex(lua: *mut lua_State) -> libc::c_int {
        return luaA::class_newindex_miss_property(lua, ::std::ptr::null_mut());
    }

    pub unsafe fn class_index_miss_property(lua: *mut lua_State,
                                            object: *mut Object)
                                            -> libc::c_int {
        use object::{global_signals, signal_object_emit};
        let GLOBALSIGNALS = global_signals.lock().unwrap();
        signal_object_emit(lua, &*GLOBALSIGNALS, "debug::index::miss", 2);
        return 0
    }

    pub unsafe fn class_newindex_miss_property(lua: *mut lua_State,
                                               object: *mut Object)
                                               -> libc::c_int {
        use object::{global_signals, signal_object_emit};
        let GLOBALSIGNALS = global_signals.lock().unwrap();
        signal_object_emit(lua, &*GLOBALSIGNALS, "debug::newindex::miss", 3);
        return 0
    }

    pub unsafe fn object_push(lua: *mut lua_State, ptr: *mut libc::c_void) -> libc::c_int {
        luaA::object_registry_push(lua);
        lua_pushlightuserdata(lua, ptr);
        lua_rawget(lua, -2);
        ::lua::lua_remove(lua, -2);
        return 1;
    }

    pub unsafe fn object_registry_push(lua: *mut lua_State) {
        lua_pushstring(lua, c_str!("awesome.object.registry"));
        lua_rawget(lua, LUA_REGISTRYINDEX);
    }

    pub unsafe fn dofunction(lua: *mut lua_State, nargs: libc::c_int,
                             nret: libc::c_int) -> libc::c_int {
        /* Move function before arguments */
        ::lua::lua_insert(lua, - nargs - 1);
        /* Push error handling function */
        lua_pushcfunction(lua, Some(luaA::dofunction_error));
        /* Move error handling function before args and functions */
        ::lua::lua_insert(lua, - nargs -2);
        let error_func_pos = lua_gettop(lua) - nargs -1;
        if lua_pcallk(lua, nargs, nret, - nargs -2, 0, None) != 0{
            eprintln!("{:?}", lua_tostring(lua, -1));
            /* Remove error function and error string */
            lua_pop(lua, 2);
            return 0;
        }
        /* Remove error function */
        ::lua::lua_remove(lua, error_func_pos);
        return 1;
    }

    pub unsafe extern fn dofunction_error(lua: *mut lua_State) -> libc::c_int {
        // TODO
        //if lualib_dofunction_on_error {
        //    return  lualib_dofunction_on_error(lua);
        //}
        return 0;
    }

    pub unsafe fn checktable(lua: *mut lua_State, idx: libc::c_int) {
        let istable = lua_type(lua, idx) == LUA_TTABLE as i32;
        if istable {
            luaA::typeerror(lua, idx, c_str!("table"));
        }
    }

    pub unsafe fn class_property_get(lua: *mut lua_State, class: *mut Class,
                                     fieldidx: libc::c_int) -> *mut Property {
        /* Lookup the property using token */
        let attr = CString::from_raw(
            luaL_checklstring(lua, fieldidx,
                              ::std::ptr::null_mut()) as *mut _)
            .into_string().expect("Could not convert CString to string");

        /* Look for the property in the class; if not found, go in the parent class. */
        let mut cur_class = class;
        while ! class.is_null() {
            if let Some(prop) = (*class).properties.iter_mut()
                .find(|prop| prop.name == attr) {
                return prop as *mut _;
            }
            cur_class = (*cur_class).parent;
        }
        return ::std::ptr::null_mut();
    }

    pub unsafe fn class_new(lua: *mut lua_State, class: *mut Class)
                            -> libc::c_int {
        /* Check we have a table that should contains some properties */
        luaA::checktable(lua, 2);

        /* Create a new object */
        let mut object_ptr = ((*class).allocator)(lua);

        /* Push the first key before iterating */
        lua_pushnil(lua);
        /* Iterate over the property keys */
        while lua_next(lua, 2) != 0 {
            /* Check that the key is a string.
             * We cannot call tostring blindly or Lua will convert a key that is a
             * number TO A STRING, confusing lua_next() */
            let is_string = lua_type(lua, -2) == LUA_TSTRING as i32;
            if is_string {
                let prop = luaA::class_property_get(lua, class, -2);

                if !prop.is_null() && (*prop).new.is_some() {
                    (*prop).new.unwrap()(lua, object_ptr);
                }
            }
            /* Remove value */
            lua_pop(lua, 1);
        }
        0
    }

    pub unsafe fn class_get(lua: *mut lua_State, idx: libc::c_int)
                            -> *mut Class {
        let ty = lua_type(lua, idx);
        if ty == LUA_TUSERDATA as i32 && lua_getmetatable(lua, idx) != 0 {
            /* Use the metatable has key to get the class from registry */
            lua_rawget(lua, LUA_REGISTRYINDEX);
            let class = lua_touserdata(lua, -1) as *mut Class;
            lua_pop(lua, 1);
            return class;
        }
        return ::std::ptr::null_mut();
    }

    pub unsafe extern fn class_newindex_invalid(lua: *mut lua_State)
                                                -> libc::c_int {
        return luaL_error(lua, c_str!("attempt to index an object that \
                                       was already garbage collected"))
    }

    pub unsafe extern fn class_index_invalid(lua: *mut lua_State)
                                             -> libc::c_int {
        let attr = CString::from_raw(
            luaL_checklstring(lua, 2, ::std::ptr::null_mut()) as _)
            .into_string().unwrap();
        if &*attr == "valid" {
            lua_pushboolean(lua, 0);
            return 1;
        }
        return luaA::class_newindex_invalid(lua);

    }

    pub unsafe extern fn class_gc(lua: *mut lua_State) -> libc::c_int {
        let item = lua_touserdata(lua, 1) as *mut Object;
        (*item).signals.clear();
        /* Get the object class */
        let class = luaA::class_get(lua, 1);
        (*class).instances -= 1;
        /* Call the collector function of the class, and all its parent classes */
        let mut cur_class = class;
        while ! cur_class.is_null() {
            if let Some(collector) = (*class).collector {
                collector(item);
            }
            cur_class = (*cur_class).parent
        }
        /* Unset its metatable so that e.g. luaA_toudata() will no longer accept
         * this object. This is needed since other __gc methods can still use this.
         * We also make sure that `item.valid == false`.
         */
        lua_newtable(lua);
        lua_pushcfunction(lua, Some(luaA::class_index_invalid));
        lua_setfield(lua, -2, c_str!("__index"));
        lua_pushcfunction(lua, Some(luaA::class_newindex_invalid));
        lua_setfield(lua, -2, c_str!("__newindex"));
        lua_setmetatable(lua, 1);
        return 0;
    }

    pub unsafe fn object_setup(lua: *mut lua_State) {
        /* Push identification string */
        lua_pushstring(lua, c_str!("awesome.object.registry"));
        /* Create an empty table */
        lua_newtable(lua);
        /* Create an empty metatable */
        lua_newtable(lua);
        /* Set this empty table as the registry metatable.
         * It's used to store the number of reference on stored objects. */
        lua_setmetatable(lua, -2);
        /* Register table inside registry */
        lua_rawset(lua, LUA_REGISTRYINDEX);
    }

    pub unsafe fn class_setup(lua: *mut lua_State, class: *mut Class,
                              name: *const libc::c_char,
                              parent: *mut Class,
                              allocator: AllocatorF,
                              collector: CollectorF,
                              checker: CheckerF,
                              index_miss_property: PropF,
                              newindex_miss_property: PropF,
                              methods: &[luaL_Reg],
                              meta: &[luaL_Reg]) {
        /* Create the object metatable */
        lua_newtable(lua);
        /* Register it with class pointer as key in the registry
        * class-pointer -> metatable */
        lua_pushlightuserdata(lua, class as _);
        /* Duplicate the object metatable */
        lua_pushvalue(lua, -2);
        lua_rawset(lua, LUA_REGISTRYINDEX);
        /* Now register class pointer with metatable as key in the registry
        * metatable -> class-pointer */
        lua_pushvalue(lua, -1);
        lua_pushlightuserdata(lua, class as _);
        lua_rawset(lua, LUA_REGISTRYINDEX);

        /* Duplicate objects metatable */
        lua_pushvalue(lua, -1);
        /* Set garbage collector in the metatable */
        lua_pushcfunction(lua, Some(luaA::class_gc));
        lua_setfield(lua, -2, c_str!("__gc"));

        lua_setfield(lua, -2, c_str!("__index")); /* metatable.__index = metatable 1 */

        luaA::registerlib(lua, ::std::ptr::null_mut(), meta);                 /* 1 */
        luaA::registerlib(lua, name, methods);                                /* 2 */
        lua_pushvalue(lua, -1);           /* dup self as metatable              3 */
        lua_setmetatable(lua, -2);        /* set self as metatable              2 */
        lua_pop(lua, 2);

        (*class).collector = Some(collector);
        (*class).allocator = allocator;
        (*class).name = CString::from_raw(name as _).into_string().unwrap();
        (*class).index_miss_prop = index_miss_property;
        (*class).newindex_miss_prop = newindex_miss_property;
        (*class).checker = Some(checker);
        (*class).parent = parent;
        (*class).tostring = None;
        (*class).instances = 0;
        (*class).index_miss_handler = LUA_REFNIL;
        (*class).newindex_miss_handler = LUA_REFNIL;

        luaA::classes.lock().unwrap().push_back(ClassWrapper::new(class));
    }

    pub unsafe extern fn object_tostring(lua: *mut lua_State) -> libc::c_int {
        let lua_class = luaA::class_get(lua, 1);
        let object = luaA::checkudata(lua, 1, lua_class);
        let mut offset = 0;
        let mut cur_class = lua_class;
        while ! cur_class.is_null() {
            if offset != 0 {
                lua_pushstring(lua, c_str!("/"));
                ::lua::lua_insert(lua, -{offset += 1; offset});
            }
            let name = CString::new((*cur_class).name.clone()).unwrap();
            lua_pushstring(lua, name.as_ptr());
            ::lua::lua_insert(lua, -{offset += 1; offset});

            if let Some(tostring) = (*cur_class).tostring {
                let (mut k, mut n) = (0, 0);
                lua_pushstring(lua, c_str!("("));
                n = 2 + tostring(lua, object as _);
                lua_pushstring(lua, c_str!(")"));

                for k in 0..n {
                    ::lua::lua_insert(lua, -offset);
                }
                offset += n;
            }

            cur_class = (*cur_class).parent;
        }
        lua_pushfstring(lua, c_str!(": %p"), object);

        lua_concat(lua, offset + 1);
        1
    }

    pub unsafe fn toudata(lua: *mut lua_State, ud: libc::c_int,
                          class: *mut Class) -> *mut libc::c_void {
        let p = lua_touserdata(lua, ud);
        /* does it have a metatable? */
        if ! p.is_null() && lua_getmetatable(lua, ud) != 0 {
            /* Get the lua_class_t that matches this metatable */
            lua_rawget(lua, LUA_REGISTRYINDEX);
            let mut metatable_class = lua_touserdata(lua, -1) as *mut Class;

            /* remove lightuserdata (lua_class pointer) */
            lua_pop(lua, 1);

            /* Now, check that the class given in argument is the same as the
             * metatable's object, or one of its parent (inheritance) */
            while ! metatable_class.is_null() {
                if metatable_class == class {
                    return p;
                }
                metatable_class = (*metatable_class).parent;
            }
        }
        return ::std::ptr::null_mut();

    }

    pub unsafe fn checkudata(lua: *mut lua_State, ud: libc::c_int,
                             class: *mut Class) -> *mut libc::c_void {
        let p = luaA::toudata(lua, ud, class);
        if p.is_null() {
            let name = CString::new((*class).name.clone()).unwrap();
            luaA::typeerror(lua, ud, name.as_ptr());
        } else if let Some(checker) = (*class).checker {
            checker(p as _);
            luaL_error(lua, c_str!("invalid object"));
        }
        return p;

    }
}

pub unsafe fn lua_remove(lua: *mut lua_State, idx: ::libc::c_int) {
    lua_rotate(lua, idx, -1);
    lua_pop(lua, 1);
}

pub unsafe fn lua_insert(lua: *mut lua_State, idx: ::libc::c_int) {
    lua_rotate(lua, idx, 1);
}
