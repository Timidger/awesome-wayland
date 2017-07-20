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
        awesome_connect_signal,
        awesome_disconnect_signal,
        awesome_emit_signal,
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
        button___tostring_meta,
        button_connect_signal,
        button_disconnect_signal,
        button_emit_signal,
        button___call,
        button_set_index_miss_handler,
        button_set_newindex_miss_handler,
        button_add_signal,
        button_instances,
        button,
        modifiers
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
    // TODO FIXME Fix this
    // You need to change how register_* and consequently how register_lua works,
    // by providing different C function names. You can have the same names bound to a global
    // in lua, but you'll need to have the C name be unique.
    // I suggest changing the array value to the register_lua macro:
    // [
    //     lua_func_name1; <global_name>_func_name (this is for C),
    //     lua_func_name2; <global_name>_func_name (this is for C),
    // ]
    register_button!(DummyStruct, AWESOME).unwrap();



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
