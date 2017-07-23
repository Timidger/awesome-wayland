//! Callbacks for the `button` object in the Lua libraries

use ::lua::Lua;
use super::default;
use libc::c_int;
use lua_sys::*;
use ::lua::{Class, Object, Signal};
use std::sync::{Mutex, MutexGuard};

// TODO This is a class, need to setup the class properly...
// Can probably do that in the `register` macro.

lazy_static! {
    pub static ref BUTTON_CLASS: Mutex<Class> = Mutex::new(Class::default());
}

pub trait Button {
    /* Methods */
    fn button_add_signal(&mut self, lua: Lua) -> c_int;
    fn button_connect_signal(&mut self, lua: Lua) -> c_int;
    fn button_disconnect_signal(&mut self, lua: Lua) -> c_int;
    fn button_emit_signal(&mut self, lua: Lua) -> c_int;
    fn button_instances(&mut self, lua: Lua) -> c_int;
    fn button_set_index_miss_handler(&mut self, lua: Lua) -> c_int;
    fn button_set_newindex_miss_handler(&mut self, lua: Lua) -> c_int;
    fn button___call(&mut self, lua: Lua) -> c_int {
        lua.new_class(&BUTTON_CLASS).unwrap_or_else(|err| {
            eprintln!("Calling button constructor returned error: {:#?}", err);
            0
        })
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

// TODO hide behind a macro like C does w/ LUA_OBJECT_FUNCS
pub fn button_new(lua_: &Lua, mut button: &mut Class) -> &'static mut Object {
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
        let button: &mut Object = &mut *(user_ptr as *mut ButtonType);
        // TODO Do this properly
        lua_newtable(lua);
        lua_pushinteger(lua, 0);
        lua_pushfstring(lua, c_str!("Hello"));
        lua_settable(lua, -3);
        lua_pushvalue(lua, -1);
        button
    }
}
