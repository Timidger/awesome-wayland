//! Loads the button class/object up, just like the awesome library.
#![allow(unused_variables)]

extern crate awesome_wayland;
extern crate lua_sys;
extern crate libc;

use awesome_wayland::{Lua, luaA, LuaErr};
use awesome_wayland::callbacks;
use std::path::PathBuf;

fn main() {
    let lua = Lua::new();
    unsafe {
        let lua = lua.0;
        luaA::object_setup(lua);
        callbacks::button::button_class_setup(lua);
    }
    match lua.load_and_run(PathBuf::from("examples/button-setup.lua")) {
        Ok(_) => {},
        Err(LuaErr::Load(_)) => {
            println!("Could not find lua file! Please run this from the root \
                      of the project directory");
            ::std::process::exit(1);
        },
        err => err.unwrap()
    }
}
