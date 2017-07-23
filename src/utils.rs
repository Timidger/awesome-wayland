//! Utilites to make my life easier

/// A way to put C-literals in Rust code.
/// Does not accept byte strings.
///
/// #Usage
/// foo(c_str!("my string"));
#[macro_export]
macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const i8
    }
}

/// Registers a lua function in a LuaL_reg to call the given global identifier.
///
/// Do NOT use this macro directly, use the register_for_lua! instead.
#[macro_export]
macro_rules! register_lua {
    ($global_name:ident, $([ $( $inner:ident; $inner_lua_name:ident ),+ ])+) => {{
        use ::awesome_wayland::callbacks::Awesome;
        use ::libc::c_int;
        $($(unsafe extern "C" fn $inner(lua: *mut lua_State) -> c_int {
            let mut callback = $global_name.lock()
                .expect("Could not lock user defined callback object");
            callback.callbacks.$inner(::Lua::from_ptr(lua))
        })*),*
            [
                $($(register_lua!($inner, $inner_lua_name)),*),*,
                ::lua_sys::luaL_Reg {
                    name: ::std::ptr::null(),
                    func: None
                },
            ]
    }};
    ($name:ident, $lua_name:ident) => {
        ::lua_sys::luaL_Reg {
            name: c_str!(stringify!($lua_name)),
            func: Some($name)
        }
    }
}

/// Registers a struct that implements [Awesome](callbacks/trait.Awesome.html)
///
/// Note that errors for registering the method is up to the caller
///
/// Use this in your main method, after using [register_for_lua](register_for_lua)
#[macro_export]
macro_rules! register_awesome {
    ($callback_impl:ident, $global_name:ident) => {{
        let lua_reg = {
            register_lua!($global_name, [
                awesome_quit; quit,
                awesome_exec; exec,
                awesome_spawn; spawn,
                awesome_restart; restart,
                awesome_connect_signal; connect_signal,
                awesome_disconnect_signal; disconnect_signal,
                awesome_emit_signal; emit_signal,
                awesome_systray; systray,
                awesome_load_image; load_image,
                awesome_set_preferred_icon_size; set_preferred_icon_size,
                awesome_register_xproperty; register_xproperty,
                awesome_set_xproperty; set_xproperty,
                awesome_get_xproperty; get_xproperty,
                awesome___index; __index,
                awesome___newindex; __newindex,
                awesome_xkb_set_layout_group; xkb_set_layout_group,
                awesome_xkb_get_layout_group; xkb_get_layout_group,
                awesome_xkb_get_group_names; xkb_get_group_names,
                awesome_xrdb_get_value; xrdb_get_value,
                awesome_kill; kill,
                awesome_sync; sync
            ])
        };
        LUA.register_methods("awesome\0", &lua_reg)
    }}
}

/// Registers a struct that implements [Button](callbacks/trait.Button.html)
///
/// Note that errors for registering the method is up to the caller
///
/// Use this in your main method, after using [register_for_lua](register_for_lua)
#[macro_export]
macro_rules! register_button {
    ($callback_impl:ident, $global_name:ident) => {{
        use ::awesome_wayland::Button;
        use ::awesome_wayland::callbacks::{default, button};
        use ::awesome_wayland::callbacks::button::BUTTON_CLASS;
        let lua_reg = {
            register_lua!($global_name,  [
                // Class methods
                button_add_signal; add_signal,
                button_connect_signal; connect_signal,
                button_disconnect_signal; disconnect_signal,
                button_emit_signal; emit_signal,
                button_instances; instances,
                button_set_index_miss_handler; set_index_miss_handler,
                button_set_newindex_miss_handler; set_newindex_miss_handler,
                // Object methods meta
                button___tostring_meta; __tostring,
                button_connect_signal_meta; connect_signal,
                button_disconnect_signal_meta; button_disconnect_signal,
                // Class methods meta
                button___index_meta; __index,
                button___newindex_meta; __newindex,
                button___call; __call,
                /* Properties */
                button; button,
                modifiers; modifiers
            ])
        };
        // TODO Fill this in with the meta methods from above!
        let lua_meta = [
            ::lua_sys::luaL_Reg {
                name: ::std::ptr::null(),
                func: None
            }
        ];
        LUA.register_class(&BUTTON_CLASS, "button\0", None,
                           Some(button::button_new), None, None,
                           Some(default::index_miss_property),
                           Some(default::newindex_miss_property),
                           &lua_reg, &lua_meta)
    }}
}

/// Registers a struct that implements [Client](callbacks/trait.Client.html)
///
/// Note that errors for registering the method is up to the caller
///
/// Use this in your main method, after using [register_for_lua](register_for_lua)
#[macro_export]
macro_rules! register_client {
    ($callback_impl:ident, $global_name:ident) => {{
        use ::awesome_wayland::callbacks::Client;
        let lua_reg = register_lua!($global_name,  [
            // Methods
            client_get; get,
            client___index; __index,
            client___newindex; __newindex,
            // Class methods
            client_add_signal; add_signal,
            client_connect_signal; connect_signal,
            client_disconnect_signal; disconnect_signal,
            client_emit_signal; emit_signal,
            client_instances; instances,
            client_set_index_miss_handler; set_index_miss_handler,
            client_set_newindex_miss_handler; set_newindex_miss_handler,
            // Object methods meta
            client___tostring_meta; __tostring_meta,
            client_connect_signal_meta; connect_signal_meta,
            client_disconnect_signal_meta; button_disconnect_signal_meta,
            // Class methods meta
            client___index_meta; __index_meta,
            client___newindex_meta; __newindex_meta,
            client___call; __call,
            // Meta
            client_keys; keys,
            client_isvisible; isvisible,
            client_geometry; geometry,
            client_apply_size_hints; apply_size_hints,
            client_tags; tags,
            client_kill; kill,
            client_swap; swap,
            client_raise; raise,
            client_lower; lower,
            client_unmanange; unmanange,
            client_titlebar_top; titlebar_top,
            client_titlebar_right; titlebar_right,
            client_titlebar_bottom; titlebar_bottom,
            client_titlebar_left; titlebar_left,
            client_get_icon; get_icon,
            // Properties
            client_name; name,
            client_transient_for; transient_for,
            client_skip_taskbar; skip_taskbar,
            client_content; content,
            client_type_; type_,
            client_class; class,
            client_instance; instance,
            client_role; role,
            client_pid; pid,
            client_leader_window; leader_window,
            client_machine; machine,
            client_icon_name; icon_name,
            client_screen; screen,
            client_hidden; hidden,
            client_minimized; minimized,
            client_fullscreen; fullscreen,
            client_modal; modal,
            client_group_window; group_window,
            client_maximized; maximized,
            client_maximized_horizontal; maximized_horizontal,
            client_maximized_vertical; maximized_vertical,
            client_icon; icon,
            client_icon_sizes; icon_sizes,
            client_ontop; ontop,
            client_above; above,
            client_below; below,
            client_sticky; sticky,
            client_size_hints_honor; size_hints_honor,
            client_urgent; urgent,
            client_size_hints; size_hints,
            client_focusable; focusable,
            client_shape_bounding; shape_bounding,
            client_shape_clip; shape_clip,
            client_shape_input; shape_input,
            client_startup_id; startup_id,
            client_client_shape_bounding; client_shape_bounding,
            client_client_shape_clip; client_shape_clip,
            client_first_tag; first_tag
        ]);

        LUA.register_methods("client\0", &lua_reg)
    }}
}

/// Registers a struct that implements [Drawin](callbacks/trait.Drawin.html)
///
/// Note that errors for registering the method is up to the caller
///
/// Use this in your main method, after using [register_for_lua](register_for_lua)
#[macro_export]
macro_rules! register_drawin {
    ($callback_impl:ident, $global_name:ident) => {{
        use ::awesome_wayland::callbacks::Drawin;
        let lua_reg = register_lua!($global_name,  [
            drawin_add_signal; add_signal,
            drawin_connect_signal; connect_signal,
            drawin_disconnect_signal; disconnect_signal,
            drawin_emit_signal; emit_signal,
            drawin_instances; instances,
            drawin_set_index_miss_handler; set_index_miss_handler,
            drawin_set_newindex_miss_handler; set_newindex_miss_handler,
            drawin___call; __call,
            drawin_geometry; geometry,
            drawin___tostring_meta; __tostring_meta,
            drawin_connect_signal_meta; connect_signal_meta,
            drawin_disconnect_signal_meta; disconnect_signal_meta,
            drawin___index_meta; __index_meta,
            drawin___newindex_meta; __newindex_meta,
            // Properties
            drawin_drawable; drawable,
            drawin_visible; visible,
            drawin_ontop; ontop,
            drawin_cursor; cursor,
            drawin_x; x,
            drawin_y; y,
            drawin_width; width,
            drawin_height; height,
            drawin_type_; type_,
            drawin_shape_bounding; shape_bounding,
            drawin_shape_clip; shape_clip,
            drawin_shape_input; shape_input
        ]);

        LUA.register_methods("drawin\0", &lua_reg)
    }}
}

/// Registers a struct that implements [Keygrabber](callbacks/trait.Keygrabber.html)
///
/// Note that errors for registering the method is up to the caller
///
/// Use this in your main method, after using [register_for_lua](register_for_lua)
#[macro_export]
macro_rules! register_keygrabber {
    ($callback_impl:ident, $global_name:ident) => {{
        use ::awesome_wayland::callbacks::Keygrabber;
        let lua_reg = register_lua!($global_name,  [
            keygrabber_run; run,
            keygrabber_stop; stop,
            keygrabber_isrunning; isrunning,
            keygrabber___index; __index,
            keygrabber___newindex; __newindex
        ]);

        LUA.register_methods("keygrabber\0", &lua_reg)
    }}
}

/// Registers a struct that implements [Mousegrabber](callbacks/trait.Mousegrabber.html)
///
/// Note that errors for registering the method is up to the caller
///
/// Use this in your main method, after using [register_for_lua](register_for_lua)
#[macro_export]
macro_rules! register_mousegrabber {
    ($callback_impl:ident, $global_name:ident) => {{
        use ::awesome_wayland::callbacks::Mousegrabber;
        let lua_reg = register_lua!($global_name,  [
            mousegrabber_run; run,
            mousegrabber_stop; stop,
            mousegrabber_isrunning; isrunning,
            mousegrabber___index; __index,
            mousegrabber___newindex; __newindex
        ]);

        LUA.register_methods("mousegrabber\0", &lua_reg)
    }}
}

/// Registers a struct that implements [Mouse](callbacks/trait.Mouse.html)
///
/// Note that errors for registering the method is up to the caller
///
/// Use this in your main method, after using [register_for_lua](register_for_lua)
#[macro_export]
macro_rules! register_mouse {
    ($callback_impl:ident, $global_name:ident) => {{
        use ::awesome_wayland::callbacks::Mouse;
        let lua_reg = register_lua!($global_name,  [
            mouse___index; __index,
            mouse___newindex; __newindex,
            mouse_coords; coords,
            mouse_object_under_pointer; object_under_pointer,
            mouse_set_index_miss_handler; set_index_miss_handler,
            mouse_set_newindex_miss_handler; set_newindex_miss_handler
        ]);

        LUA.register_methods("mouse\0", &lua_reg)
    }}
}

/// Registers a struct that implements [Root](callbacks/trait.Root.html)
///
/// Note that errors for registering the method is up to the caller
///
/// Use this in your main method, after using [register_for_lua](register_for_lua)
#[macro_export]
macro_rules! register_root {
    ($callback_impl:ident, $global_name:ident) => {{
        use ::awesome_wayland::callbacks::Root;
        let lua_reg = register_lua!($global_name,  [
            root_buttons; buttons,
            root_keys; keys,
            root_cursor; cursor,
            root_fake_input; fake_input,
            root_drawins; drawins,
            root_wallpaper; wallpaper,
            root_size; size,
            root_size_mm; size_mm,
            root_tags; tags,
            root___index; __index,
            root___newindex; __newindex
        ]);

        LUA.register_methods("root\0", &lua_reg)
    }}
}

/// Registers a struct that implements [Screen](callbacks/trait.Screen.html)
///
/// Note that errors for registering the method is up to the caller
///
/// Use this in your main method, after using [register_for_lua](register_for_lua)
#[macro_export]
macro_rules! register_screen {
    ($callback_impl:ident, $global_name:ident) => {{
        use ::awesome_wayland::callbacks::Screen;
        let lua_reg = register_lua!($global_name,  [
            screen_add_signal; add_signal,
            screen_connect_signal; connect_signal,
            screen_disconnect_signal; disconnect_signal,
            screen_emit_signal; emit_signal,
            screen_instances; instances,
            screen_set_index_miss_handler; set_index_miss_handler,
            screen_set_newindex_miss_handler; set_newindex_miss_handler,
            screen_count; count,
            screen___index; __index,
            screen___newindex; __newindex,
            screen___call; __call,
            screen_fake_add; fake_add,
            screen___tostring_meta; __tostring_meta,
            screen_connect_signal_meta; connect_signal_meta,
            screen_disconnect_signal_meta; disconnect_signal_meta,
            screen___index_meta; __index_meta,
            screen___newindex_meta; __newindex_meta,
            screen_fake_remove; fake_remove,
            screen_fake_resize; fake_resize,
            screen_swap; swap,
            // properties
            screen_geometry; geometry,
            screen_index; index,
            screen_outputs; outputs,
            screen_workarea; workarea
        ]);

        LUA.register_methods("screen\0", &lua_reg)
    }}
}

/// Registers a struct that implements [Tag](callbacks/trait.Tag.html)
///
/// Note that errors for registering the method is up to the caller
///
/// Use this in your main method, after using [register_for_lua](register_for_lua)
#[macro_export]
macro_rules! register_tag {
    ($callback_impl:ident, $global_name:ident) => {{
        use ::awesome_wayland::callbacks::Tag;
        let lua_reg = register_lua!($global_name,  [
            tag_add_signal; add_signal,
            tag_connect_signal; connect_signal,
            tag_disconnect_signal; disconnect_signal,
            tag_emit_signal; emit_signal,
            tag_instances; instances,
            tag_set_index_miss_handler; set_index_miss_handler,
            tag_set_newindex_miss_handler; set_newindex_miss_handler,
            tag___call; __call,
            tag___tostring_meta; __tostring_meta,
            tag_connect_signal_meta; connect_signal_meta,
            tag_disconnect_signal_meta; disconnect_signal_meta,
            tag___index_meta; __index_meta,
            tag___newindex_meta; __newindex_meta,
            tag_clients_meta; clients_meta,
            tag_name; name,
            tag_selected; selected,
            tag_activated; activated
        ]);

        LUA.register_methods("tag\0", &lua_reg)
    }}
}


/// Registers all of the callbacks to be for the passed in global.
/// This is a helpful convience macro so you don't have to write
/// out all those registers.
///
/// Note that this does absolutely no error handling what-so-ever.
/// If you want to handle the possibilty of the registerts failing
/// (which is unlikely, they should work) then use the individual register_*!
#[macro_export]
macro_rules! register_all {
    ($callback_impl:ident, $global_name:ident) => {{
        register_awesome!($callback_impl, $global_name).unwrap();
        register_button!($callback_impl, $global_name).unwrap();
        register_client!($callback_impl, $global_name).unwrap();
        register_drawin!($callback_impl, $global_name).unwrap();
        register_keygrabber!($callback_impl, $global_name).unwrap();
        register_mousegrabber!($callback_impl, $global_name).unwrap();
        register_mouse!($callback_impl, $global_name).unwrap();
        register_root!($callback_impl, $global_name).unwrap();
        register_screen!($callback_impl, $global_name).unwrap();
        register_tag!($callback_impl, $global_name).unwrap();
    }}
}

/// Registers a new instance of the passed-in user object as a global
/// singleton that will be used for all of the Lua callbacks.
///
/// This also registers a global named `LUA` that is an instance of
/// [Lua](../awesome_wayland/struct.Lua.html). This is used by both the user
/// and internally by the library, which is why it needs a pre-defined name.
#[macro_export]
macro_rules! register_for_lua {
    ($callback_impl:ident, $global_name:ident) => {
        use ::std::sync::{Mutex, Arc};
        use ::awesome_wayland::Class;
        lazy_static! {
            pub static ref $global_name: Mutex<Awesome<$callback_impl>> =
                Mutex::new(Awesome::new());
            pub static ref LUA: Arc<Lua> = Arc::new(Lua::new());
        }
    }
}

/// Defines properties for the method
/// Eventually, this will automatically set the correct values in lua
/// so that they can be used as accessors, e.g []
/// For now, it just defines them
macro_rules! properties {
    ($([ $( $inner:ident ),+ ])+) => {
        $($(fn $inner(&mut self, lua: Lua) -> c_int;)*),*
    };
}
