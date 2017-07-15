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

/// Defines the the default impl of a callback to do nothing.
/// Save on a LOT of typing
macro_rules! default_impl{
    ($([ $( $inner:ident ),+ ])+) => {
        $($(fn $inner(&mut self, lua: Lua) {})*),*
    };
}


#[allow(unused_variables)]
impl callbacks::Awesome for DummyStruct {
    default_impl!([
        quit,
        exec,
        spawn,
        restart,
        connect_signal,
        disconnect_signal,
        emit_signal,
        systray,
        load_image,
        set_preferred_icon_size,
        register_xproperty,
        set_xproperty,
        get_xproperty,
        __index,
        __newindex,
        xkb_set_layout_group,
        xkb_get_layout_group,
        xkb_get_group_names,
        xrdb_get_value,
        kill,
        sync
    ]);
}

impl callbacks::Button for DummyStruct {
    default_impl!([
        __tostring_meta,
        connect_signal,
        disconnect_signal,
        emit_signal,
        __call,
        button,
        modifiers,
        add_signal,
        instances,
        set_index_miss_handler,
        set_newindex_miss_handler
    ]);
}

// TODO Remove
impl callbacks::Beautiful for DummyStruct {}

impl callbacks::Client for DummyStruct {
    default_impl!([
        add_signal,
        connect_signal,
        disconnect_signal,
        emit_signal,
        instances,
        set_index_miss_handler,
        set_newindex_miss_handler,
        get,
        __index,
        __newindex,
        keys,
        isvisible,
        geometry,
        apply_size_hints,
        tags,
        kill,
        swap,
        raise,
        lower,
        unmanange,
        titlebar_top,
        titlebar_right,
        titlebar_bottom,
        titlebar_left,
        get_icon
    ]);
}
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
