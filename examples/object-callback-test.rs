//! Loads the button class/object up, just like the awesome library.
#![allow(unused_variables)]

#[macro_use] extern crate awesome_wayland;
#[macro_use] extern crate lazy_static;
extern crate lua_sys;
extern crate libc;

use std::path::PathBuf;
use std::default::Default;
use awesome_wayland::{Lua, luaA, LuaErr, Awesome};
use awesome_wayland::callbacks;
use awesome_wayland::callbacks::*;
use libc::c_int;
use lua_sys::lua_State;
use lua_sys::*;

// Contains no state, just here so we can register the libs.
pub struct DummyStruct;

/// Defines the the default impl of a callback to do nothing.
/// Save on a LOT of typing
macro_rules! default_impl {
    ($([ $( $inner:ident ),+ ])+) => {
        $($(fn $inner(&self, lua: &Lua) -> c_int {0})*),*
    };
}

#[allow(unused_variables)]
impl callbacks::Awesome for DummyStruct {
    default_impl!([
        awesome_quit,
        awesome_spawn,
        awesome_restart,
        awesome_systray,
        awesome_load_image,
        awesome_set_preferred_icon_size,
        awesome_register_xproperty,
        awesome_set_xproperty,
        awesome_get_xproperty,
        awesome_xkb_set_layout_group,
        awesome_xkb_get_layout_group,
        awesome_xkb_get_group_names,
        awesome_xrdb_get_value,
        awesome_kill,
        awesome_sync
    ]);
}

impl callbacks::Button for DummyStruct {
}

impl callbacks::Client for DummyStruct {
    default_impl!([
        client_add_signal,
        client_connect_signal,
        client_disconnect_signal,
        client_emit_signal,
        client_instances,
        client_set_index_miss_handler,
        client_set_newindex_miss_handler,
        client_get,
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
        drawin_add_signal,
        drawin_connect_signal,
        drawin_disconnect_signal,
        drawin_emit_signal,
        drawin_instances,
        drawin_set_index_miss_handler,
        drawin_set_newindex_miss_handler,
        drawin___call,
        drawin_geometry,
        drawin_drawable,
        drawin_visible,
        drawin_ontop,
        drawin_cursor,
        drawin_x,
        drawin_y,
        drawin_width,
        drawin_height,
        drawin_type_,
        drawin_shape_bounding,
        drawin_shape_clip,
        drawin_shape_input
    ]);
}
impl callbacks::Keygrabber for DummyStruct {
    default_impl!([
        keygrabber_run,
        keygrabber_stop,
        keygrabber_isrunning,
        keygrabber___index,
        keygrabber___newindex
    ]);
}
impl callbacks::Mousegrabber for DummyStruct {
    default_impl!([
        mousegrabber_run,
        mousegrabber_stop,
        mousegrabber_isrunning,
        mousegrabber___index,
        mousegrabber___newindex
    ]);
}
impl callbacks::Mouse for DummyStruct {
    default_impl!([
        mouse___index,
        mouse___newindex,
        mouse_coords,
        mouse_object_under_pointer,
        mouse_set_index_miss_handler,
        mouse_set_newindex_miss_handler
    ]);
}
impl callbacks::Root for DummyStruct {
    default_impl!([
        root_buttons,
        root_keys,
        root_cursor,
        root_fake_input,
        root_drawins,
        root_wallpaper,
        root_size,
        root_size_mm,
        root_tags,
        root___index,
        root___newindex
    ]);
}
impl callbacks::Screen for DummyStruct {
    default_impl!([
        screen_add_signal,
        screen_connect_signal,
        screen_disconnect_signal,
        screen_emit_signal,
        screen_instances,
        screen_set_index_miss_handler,
        screen_set_newindex_miss_handler,
        screen_count,
        screen___index,
        screen___newindex,
        screen___call,
        screen_fake_add,
        screen_fake_remove,
        screen_fake_resize,
        screen_swap,
        screen_geometry,
        screen_index,
        screen_outputs,
        screen_workarea
    ]);
}
impl callbacks::Tag for DummyStruct {
    default_impl!([
        tag_add_signal,
        tag_connect_signal,
        tag_disconnect_signal,
        tag_emit_signal,
        tag_instances,
        tag_set_index_miss_handler,
        tag_set_newindex_miss_handler,
        tag___call,
        tag_clients_meta,
        tag_name,
        tag_selected,
        tag_activated
    ]);
}

impl Default for DummyStruct {
    fn default() -> Self {
        DummyStruct
    }
}

register_for_lua!(DummyStruct, AWESOME);

fn main() {
    register_all!(DummyStruct, AWESOME);
    match LUA.load_and_run(PathBuf::from("examples/object-callback-test.lua")) {
        Ok(_) => {},
        Err(LuaErr::Load(_)) => {
            println!("Could not find lua file! Please run this from the root \
                      of the project directory");
            ::std::process::exit(1);
        },
        err => err.unwrap()
    }
}
