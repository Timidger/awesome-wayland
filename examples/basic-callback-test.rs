//! Attempts to load the shim files used in basic Awesome testing.
//! This is the most minimal stress test to ensure globals are defined
//! correctly by the backend.

#[macro_use] extern crate awesome_wayland;
#[macro_use] extern crate lazy_static;
extern crate lua_sys;
use lua_sys::*;

use awesome_wayland::{Lua};
use awesome_wayland::callbacks::Awesome;
use std::path::PathBuf;

const SHIMS: &[&'static str] = &["awesome.lua", "beautiful.lua", "button.lua",
                                 "client.lua", "drawin.lua", "keygrabber.lua",
                                 "mousegrabber.lua", "mouse.lua", "root.lua",
                                 "screen.lua", "tag.lua"];

// Contains no state, just here so we can register the libs.
struct DummyStruct;

#[allow(unused_variables)]
impl Awesome for DummyStruct {
    fn new() -> Self { DummyStruct }
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

fn main() {
    let mut lua = Lua::new();
    register_awesome!(DummyStruct, AWESOME, lua).unwrap();
    lua.load_and_run(PathBuf::from("examples/basic-callback-test.lua"))
        .unwrap_or_else(|_| {
            println!("Could not find lua file! Please run this from the root \
                      of the project directory");
            ::std::process::exit(1);
        });
}
