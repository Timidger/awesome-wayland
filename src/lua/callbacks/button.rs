//! Callbacks for the `button` object in the Lua libraries

use ::lua::Lua;
use super::default;
use libc::c_int;
use lua_sys::*;
use ::lua::{Class, Object, Signal};
use std::sync::{Mutex, MutexGuard};
use std::ffi::CString;

// TODO This is a class, need to setup the class properly...
// Can probably do that in the `register` macro.

lazy_static! {
    pub static ref BUTTON_CLASS: Mutex<Class> = Mutex::new(Class::default());
}

pub trait Button {
    /* Methods */
    fn button_add_signal(&mut self, lua: Lua) -> c_int;
    fn button_connect_signal(&mut self, lua: Lua) -> c_int {
        /* The state of the stack:
        0: whatever
        -1: The object (in this case button) (oud)
        -2: The name of the signal to bind to (e.g "new")
        -3: The function to bind to the signal (e.g function() end) (ud)
        */
        unsafe {
            // TODO Assert -1
            //assert!(lua_type(lua.0, -1))
            ::utils::print_stack(&lua, 10);
            assert!(lua_type(lua.0, -2) == LUA_TSTRING as i32);
            assert!(lua_type(lua.0, -3) == LUA_TFUNCTION as i32);
            // TODO Is supposed to be an &Object or Box<Object> or whatever
            let userdata = lua_touserdata(lua.0, -1);
            // get the environment table from the object

            //  START luaA_object_ref_item
            lua_getuservalue(lua.0, -1);
            // CALL luaA_object_incref
            //  END luaA_object_ref_item
            // USE THAT for:
            // signal_connect










            //lua_pop(lua.0, 1);
            let string = CString::new(lua.get_arg_as_string(-2).unwrap()).unwrap();
            lua_pushstring(lua.0, string.as_ptr());
            println!("STRING: {:?}", string);
            lua_callk(lua.0, 1, 1, 0, None);
            panic!();
            /*let idx = -2;
            let ex = lua.get_arg_as_string(idx).unwrap();
            println!("GOT {} for {}", ex, idx);
            let fnc = lua_topointer(lua.0, -1) as *const unsafe extern "C" fn(*mut lua_State) -> i32;
            println!("GOT POINTER: {:?}", fnc);
            let fnc =  lua.get_arg_as_cfunction(-1).unwrap();
            let string = CString::new(lua.get_arg_as_string(1).unwrap()).unwrap();
            lua_pushstring(lua.0, string.as_ptr());
            lua_pushcfunction(lua.0, Some(::std::mem::transmute(fnc)));*/
            lua_callk(lua.0, 1, 1, 0, None);
        }
        panic!();
        // TODO error handling
        let name = lua.get_arg_as_string(1)
            .expect("Was not provided a name for the signal to connect to");
        eprintln!("Connecting a button signal {:?}", name);
        unsafe {
            match lua.get_arg_as_cfunction(1) {
                Ok(func) => {
                    let mut button = BUTTON_CLASS.lock()
                        .expect("Could not lock button class");
                    let func = *(&func as *const _
                                as *const unsafe extern "C" fn(*mut lua_State) -> i32);
                    button.connect_signal(name.as_str(), Some(func))
                },
                err => {
                    eprintln!("Could not connect button signal: {:#?}", err);
                }
            }
        }
        0
    }
    fn button_disconnect_signal(&mut self, lua: Lua) -> c_int;
    fn button_emit_signal(&mut self, lua: Lua) -> c_int;
    fn button_instances(&mut self, lua: Lua) -> c_int;
    fn button_set_index_miss_handler(&mut self, lua: Lua) -> c_int;
    fn button_set_newindex_miss_handler(&mut self, lua: Lua) -> c_int;
    fn button___call(&mut self, lua: Lua) -> c_int {
        lua.new_class(&BUTTON_CLASS).unwrap_or_else(|err| {
            eprintln!("Calling button constructor returned error: {:#?}", err);
            0
        });
        1
    }
    /* Meta */
    fn button___tostring_meta(&mut self, lua: Lua) -> c_int {
        default::__tostring_meta(lua)
    }
    fn button_connect_signal_meta(&mut self, lua: Lua) -> c_int {
        default::connect_signal_meta(lua)
    }
    fn button_disconnect_signal_meta(&mut self, lua: Lua) -> c_int {
        default::disconnect_signal_meta(lua)
    }
    // TODO Give these the default impls
    /* LUA_CLASS_META methods */
    fn button___index_meta(&mut self, lua: Lua) -> c_int {
        default::__index_meta(lua)
    }
    fn button___newindex_meta(&mut self, lua: Lua) -> c_int {
        default::__newindex_meta(lua)
    }
    /* Properties  */
    properties!([
        button,
        modifiers
    ]);
}

pub struct ButtonType {
    signals: Vec<Signal>,
    modifiers: u16,
    // TODO This is hard-coded, dependent on platform,
    // also it's for xcb, which isn't a thing anymore
    xcb_button_t: u8
}

impl Object for ButtonType {
    fn signals(&self) -> &[Signal] {
        panic!()
        //self.signals.as_slice()
    }
}

// NOTE _this_ is the proper one.
pub unsafe extern "C" fn button_new(lua: *mut lua_State) -> c_int {
    // TODO Not try_lock
    {
        let mut button = BUTTON_CLASS.try_lock().unwrap();
        let user_ptr = lua_newuserdata(lua, ::std::mem::size_of::<ButtonType>());
        button.instances += 1;
        // Set the type of the data to be Button
        lua_pushlightuserdata(lua, &mut *button as *mut _ as *mut _);
    }
    lua_rawget(lua, LUA_REGISTRYINDEX);
    lua_setmetatable(lua, -2);
    // Back to table construction
    lua_newtable(lua);
    lua_newtable(lua);
    lua_setmetatable(lua, -2);
    lua_newtable(lua);
    lua_setfield(lua, -2, c_str!("data"));
    // Set uservale
    lua_setuservalue(lua, -2);
    lua_pushvalue(lua, -1);
    // Now emit the signal, e.g call the function bound to the "new" signal
    {
        let button = BUTTON_CLASS.try_lock().unwrap();
        if let Err(err) = button.push_signal(lua, "new") {
            eprintln!("Could not push new signal: {:#?}", err);
            return 0
        }
    }
    // TODO remove
    lua_newtable(lua);
    lua_pushinteger(lua, 0);
    lua_pushfstring(lua, c_str!("hello"));
    lua_settable(lua, -3);
    lua_callk(lua, 1, 1, 0, None);
    1
}

/*
// TODO hide behind a macro like C does w/ LUA_OBJECT_FUNCS
pub fn button_new(lua_: &Lua, mut button: &mut Class) -> Option<&'static mut Object> {
    unsafe {
        let lua = lua_.0;
        let user_ptr = lua_newuserdata(lua, ::std::mem::size_of::<ButtonType>());
        button.instances += 1;
        // Set the type of the data to be Button
        lua_pushlightuserdata(lua, &mut *button as *mut _ as *mut _);
        lua_rawget(lua, LUA_REGISTRYINDEX);
        lua_setmetatable(lua, -2);
        // Back to table construction
        lua_newtable(lua);
        lua_newtable(lua);
        lua_setmetatable(lua, -2);
        lua_newtable(lua);
        lua_setfield(lua, -2, c_str!("data"));
        // Set uservale
        lua_setuservalue(lua, -2);
        lua_pushvalue(lua, -1);
        // TODO luaA_class_emit_signal(L, &(lua_class), "new", 1);
        //lua_.class_emit_signal(button.signals.as_slice(), "new", 1);
        if let Ok(_) = button.push_signal(lua, "new") {
            let button: &mut Object = &mut *(user_ptr as *mut ButtonType);
            Some(button)
        } else {
            None
        }
    }
}
*/
