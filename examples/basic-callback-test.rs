//! Attempts to load the shim files used in basic Awesome testing.
//! This is the most minimal stress test to ensure globals are defined
//! correctly by the backend.
#![allow(unused_variables)]

#[macro_use] extern crate awesome_wayland;
#[macro_use] extern crate lazy_static;
extern crate lua_sys;
use lua_sys::*;

use awesome_wayland::{Lua, LuaErr, Awesome};
use awesome_wayland::callbacks;
use std::path::PathBuf;
use std::default::Default;

const SHIMS: &[&'static str] = &["awesome.lua", "beautiful.lua", "button.lua",
                                 "client.lua", "drawin.lua", "keygrabber.lua",
                                 "mousegrabber.lua", "mouse.lua", "root.lua",
                                 "screen.lua", "tag.lua"];

// Contains no state, just here so we can register the libs.
pub struct DummyStruct;

#[allow(unused_variables)]
impl callbacks::Awesome for DummyStruct {
    fn quit(&mut self, awesome: Lua) {}
    fn exec(&mut self, awesome: Lua) {}
    fn spawn(&mut self, awesome: Lua) {}
    fn restart(&mut self, awesome: Lua) {}
    fn connect_signal(&mut self, awesome: Lua) {}
    fn disconnect_signal(&mut self, awesome: Lua) {}
    fn emit_signal(&mut self, awesome: Lua) {}
    fn systray(&mut self, awesome: Lua) {}
    fn load_image(&mut self, awesome: Lua) {}
    fn set_preferred_icon_size(&mut self, awesome: Lua) {}
    fn register_xproperty(&mut self, awesome: Lua) {}
    fn set_xproperty(&mut self, awesome: Lua) {}
    fn get_xproperty(&mut self, awesome: Lua) {}
    fn __index(&mut self, awesome: Lua) {}
    fn __newindex(&mut self, awesome: Lua) {}
    fn xkb_set_layout_group(&mut self, awesome: Lua) {}
    fn xkb_get_layout_group(&mut self, awesome: Lua) {}
    fn xkb_get_group_names(&mut self, awesome: Lua) {}
    fn xrdb_get_value(&mut self, awesome: Lua) {}
    fn kill(&mut self, awesome: Lua) {}
    fn sync(&mut self, awesome: Lua) {}
}

impl callbacks::Button for DummyStruct {
    fn __tostring(&mut self, awesome: Lua) {}
    fn connect_signal(&mut self, awesome: Lua) {}
    fn disconnect_signal(&mut self, awesome: Lua) {}
    fn emit_signal(&mut self, awesome: Lua) {}
    fn __call(&mut self, awesome: Lua) {}
    fn button(&mut self, awesome: Lua) {}
    fn modifiers(&mut self, awesome: Lua) {}
}
impl callbacks::Beautiful for DummyStruct {}
impl callbacks::Client for DummyStruct {}
impl callbacks::Drawin for DummyStruct {}
impl callbacks::Keygrabber for DummyStruct {}
impl callbacks::Mousegrabber for DummyStruct {}
impl callbacks::Mouse for DummyStruct {}
impl callbacks::Root for DummyStruct {}
impl callbacks::Screen for DummyStruct {}
impl callbacks::Tag for DummyStruct {}

impl Default for DummyStruct {
    fn default() -> Self {
        DummyStruct
    }
}

register_for_lua!(DummyStruct, AWESOME);

fn main() {
    register_awesome!(DummyStruct, AWESOME).unwrap();
    // TODO Other registers

    match LUA.load_and_run(PathBuf::from("examples/basic-callback-test.lua")) {
        Ok(_) => {},
        Err(LuaErr::Load(_)) => {
            println!("Could not find lua file! Please run this from the root \
                      of the project directory");
            ::std::process::exit(1);
        },
        err => err.unwrap()
    }
}
