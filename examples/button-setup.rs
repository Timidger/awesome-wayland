//! Loads the button class/object up, just like the awesome library.
#![allow(unused_variables)]

#[macro_use] extern crate awesome_wayland;
#[macro_use] extern crate lazy_static;
extern crate lua_sys;
extern crate libc;
use lua_sys::*;
use libc::c_int;

use awesome_wayland::{Lua, luaA, LuaErr, Awesome};
use awesome_wayland::object::class::{Class, Object};
use awesome_wayland::callbacks;
use awesome_wayland::callbacks::*;
use std::path::PathBuf;
use std::default::Default;

fn main() {
    let LUA = Lua::new();
    unsafe {
        let lua = LUA.0;
        luaA::object_setup(lua);
        callbacks::button::button_class_setup(lua);
    }
    match LUA.load_and_run(PathBuf::from("examples/button-setup.lua")) {
        Ok(_) => {},
        Err(LuaErr::Load(_)) => {
            println!("Could not find lua file! Please run this from the root \
                      of the project directory");
            ::std::process::exit(1);
        },
        err => err.unwrap()
    }
}
