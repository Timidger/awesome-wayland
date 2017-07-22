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
        awesome_quit,
        awesome_exec,
        awesome_spawn,
        awesome_restart,
        awesome_connect_signal,
        awesome_disconnect_signal,
        awesome_emit_signal,
        awesome_systray,
        awesome_load_image,
        awesome_set_preferred_icon_size,
        awesome_register_xproperty,
        awesome_set_xproperty,
        awesome_get_xproperty,
        awesome___index,
        awesome___newindex,
        awesome_xkb_set_layout_group,
        awesome_xkb_get_layout_group,
        awesome_xkb_get_group_names,
        awesome_xrdb_get_value,
        awesome_kill,
        awesome_sync
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
        client_add_signal,
        client_connect_signal,
        client_disconnect_signal,
        client_emit_signal,
        client_instances,
        client_set_index_miss_handler,
        client_set_newindex_miss_handler,
        client___get,
        client___call,
        client___index,
        client___newindex,
        client_keys,
        client_isvisible,
        client_geometry,
        client_apply_size_hints,
        client_tags,
        client_kill,
        client_swap,
        client_raise,
        client_lower,
        client_unmanange,
        client_titlebar_top,
        client_titlebar_right,
        client_titlebar_bottom,
        client_titlebar_left,
        client_get_icon
    ]);
    // Properties
    default_impl!([
        client_name,
        client_transient_for,
        client_skip_taskbar,
        client_content,
        client_type_,
        client_class,
        client_instance,
        client_role,
        client_pid,
        client_leader_window,
        client_machine,
        client_icon_name,
        client_screen,
        client_hidden,
        client_minimized,
        client_fullscreen,
        client_modal,
        client_group_window,
        client_maximized,
        client_maximized_horizontal,
        client_maximized_vertical,
        client_icon,
        client_icon_sizes,
        client_ontop,
        client_above,
        client_below,
        client_sticky,
        client_size_hints_honor,
        client_urgent,
        client_size_hints,
        client_focusable,
        client_shape_bounding,
        client_shape_clip,
        client_shape_input,
        client_startup_id,
        client_client_shape_bounding,
        client_client_shape_clip,
        client_first_tag
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
    register_client!(DummyStruct, AWESOME).unwrap();
    //register_drawin!(DummyStruct, AWESOME).unwrap();
    //register_keygrabber!(DummyStruct, AWESOME).unwrap();
    //register_mousegrabber!(DummyStruct, AWESOME).unwrap();
    //register_mouse!(DummyStruct, AWESOME).unwrap();
    //register_root!(DummyStruct, AWESOME).unwrap();
    //register_screen!(DummyStruct, AWESOME).unwrap();
    //register_tag!(DummyStruct, AWESOME).unwrap();



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
