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

const AWESOME_LIB: &'static str = "shims/awesome.lua";
const BEAUTIFUL_LIB: &'static str = "shims/beautiful.lua";
const BUTTON_LIB: &'static str = "shims/button.lua";
const CLIENT_LIB: &'static str = "shims/client.lua";
const DRAWIN_LIB: &'static str = "shims/drawin.lua";
const KEYGRABBER_LIB: &'static str = "shims/keygrabber.lua";
const MOUSEGRABBER_LIB: &'static str = "shims/mousegrabber.lua";
const MOUSE_LIB: &'static str = "shims/mouse.lua";
const ROOT_LIB: &'static str = "shims/root.lua";
const SCREEN_LIB: &'static str = "shims/screen.lua";
const TAG_LIB: &'static str = "shims/tag.lua";

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
    // Properties
    default_impl!([
        name,
        transient_for,
        skip_taskbar,
        content,
        type_,
        class,
        instance,
        role,
        pid,
        leader_window,
        machine,
        icon_name,
        screen,
        hidden,
        minimized,
        fullscreen,
        modal,
        group_window,
        maximized,
        maximized_horizontal,
        maximized_vertical,
        icon,
        icon_sizes,
        ontop,
        above,
        below,
        sticky,
        size_hints_honor,
        urgent,
        size_hints,
        focusable,
        shape_bounding,
        shape_clip,
        shape_input,
        startup_id,
        client_shape_bounding,
        client_shape_clip,
        first_tag
    ]);
}
impl callbacks::Drawin for DummyStruct {
    default_impl!([
        add_signal,
        connect_signal,
        disconnect_signal,
        emit_signal,
        instances,
        set_index_miss_handler,
        set_newindex_miss_handler,
        __call,
        geometry,
        drawable,
        visible,
        ontop,
        cursor,
        x,
        y,
        width,
        height,
        type_,
        shape_bounding,
        shape_clip,
        shape_input
    ]);
}
impl callbacks::Keygrabber for DummyStruct {
    default_impl!([
        run,
        stop,
        isrunning,
        __index,
        __newindex
    ]);
}
impl callbacks::Mousegrabber for DummyStruct {
    default_impl!([
        run,
        stop,
        isrunning,
        __index,
        __newindex
    ]);
}
impl callbacks::Mouse for DummyStruct {
    default_impl!([
        __index,
        __newindex,
    coords,
        object_under_pointer,
        set_index_miss_handler,
        set_newindex_miss_handler
    ]);
}
impl callbacks::Root for DummyStruct {
    default_impl!([
        buttons,
        keys,
        cursor,
        fake_input,
        drawins,
        wallpaper,
        size,
        size_mm,
        tags,
        __index,
        __newindex
    ]);
}
impl callbacks::Screen for DummyStruct {
    default_impl!([
        add_signal,
        connect_signal,
        disconnect_signal,
        emit_signal,
        instances,
        set_index_miss_handler,
        set_newindex_miss_handler,
        count,
        __index,
        __newindex,
        __call,
        fake_add,
        fake_remove,
        fake_resize,
        swap,
        geometry,
        index,
        outputs,
        workarea
    ]);
}
impl callbacks::Tag for DummyStruct {
    default_impl!([
        add_signal,
        connect_signal,
        disconnect_signal,
        emit_signal,
        instances,
        set_index_miss_handler,
        set_newindex_miss_handler,
        __call,
        clients,
        name,
        selected,
        activated
    ]);
}

impl Default for DummyStruct {
    fn default() -> Self {
        DummyStruct
    }
}

register_for_lua!(DummyStruct, AWESOME);

fn main() {
    register_awesome!(DummyStruct, AWESOME).unwrap();
    // TODO Other registers

    // Add shims/ as a place for Lua to look for libraries.
    LUA.add_lib_lookup_path(&[";shims/?.lua".into()]);

    // Adds default awesome libs to path
    // NOTE that we added this AFTER the shims path, in order
    // for the shims path to have precedence.
    LUA.add_default_awesome_libs();

    // Load library shims.
    LUA.load_library("awesome", AWESOME_LIB.into()).unwrap();
    LUA.load_library("beautiful", BEAUTIFUL_LIB.into()).unwrap();
    LUA.load_library("button", BUTTON_LIB.into()).unwrap();
    LUA.load_library("client", CLIENT_LIB.into()).unwrap();
    LUA.load_library("drawin", DRAWIN_LIB.into()).unwrap();
    LUA.load_library("keygrabber", KEYGRABBER_LIB.into()).unwrap();
    LUA.load_library("mousegrabber", MOUSEGRABBER_LIB.into()).unwrap();
    LUA.load_library("mouse", MOUSE_LIB.into()).unwrap();
    LUA.load_library("root", ROOT_LIB.into()).unwrap();
    LUA.load_library("screen", SCREEN_LIB.into()).unwrap();
    LUA.load_library("tag", TAG_LIB.into()).unwrap();

    // Run the user init script
    //LUA.load_and_run("examples/rc.lua".into()).unwrap();
}
