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
        $($(unsafe extern "C" fn $inner(lua: *mut lua_State) -> i32 {
            let mut callback = $global_name.lock()
                .expect("Could not lock user defined callback object");
            callback.callbacks.$inner(::Lua::from_ptr(lua));
            0
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
                quit; quit,
                exec; exec,
                spawn; spawn,
                restart; restart,
                awesome_connect_signal; connect_signal,
                awesome_disconnect_signal; disconnect_signal,
                awesome_emit_signal; emit_signal,
                systray; systray,
                load_image; load_image,
                set_preferred_icon_size; set_preferred_icon_size,
                register_xproperty; register_xproperty,
                set_xproperty; set_xproperty,
                get_xproperty; get_xproperty,
                __index; __index,
                __newindex; __newindex,
                xkb_set_layout_group; xkb_set_layout_group,
                xkb_get_layout_group; xkb_get_layout_group,
                xkb_get_group_names; xkb_get_group_names,
                xrdb_get_value; xrdb_get_value,
                kill; kill,
                sync; sync
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
                button___tostring_meta; __tostring_meta,
                button_connect_signal_meta; connect_signal_meta,
                button_disconnect_signal_meta; button_disconnect_signal_meta,
                // Class methods meta
                button___index_meta; __index_meta,
                button___newindex_meta; __newindex_meta,
                button___call; __call,
                /* Properties */
                button; button,
                modifiers; modifiers
            ])
        };

        LUA.register_methods("button\0", &lua_reg)
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
            client___get; __get,
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

/// Defines the methods associated with classes. These methods have default
/// implementations, but can be defined by the user if they so choose.
///
/// Equiv to the C macro LUA_CLASS_METHODS(prefix)
#[macro_export]
// TODO Remove, these names will clash and this will need to be written out manually...
macro_rules! class_methods {
    ($prefix:ident) => {
        // TODO Add default impls that call:
        // TODO ${prefix}_<method_name>
        // or perhaps ${prefix}::<method_name>
        fn add_signal(&mut self, awesome: Lua);
        fn connect_signal(&mut self, awesome: Lua);
        fn disconnect_signal(&mut self, awesome: Lua);
        fn emit_signal(&mut self, awesome: Lua);
        fn instances(&mut self, awesome: Lua);
        fn set_index_miss_handler(&mut self, awesome: Lua);
        fn set_newindex_miss_handler(&mut self, awesome: Lua);
    }
}

/// Defines the meta methods associated with classes. These methods have default
/// implementations, but can be defined by the user if they so choose.
///
/// Equiv to the C macro LUA_CLASS_META
macro_rules! class_methods_meta {
    () => {
        // TODO Give these the default impls
        /* LUA_CLASS_META methods */
        fn __index_meta(&mut self, awesome: Lua) -> c_int {
            // TODO luaA_class_index
            0
        }
        fn __newindex_meta(&mut self, awesome: Lua) -> c_int {
            // TODO luaA_class_newindex
            0
        }

    }
}

/// Object meta methods with default implementation
///
/// Equiv to the C macro LUA_OBJECT_META(prefix)
macro_rules! object_methods_meta {
    ($prefix:ident) => {
        /* LUA_OBJECT_META methods */
        fn __tostring_meta(&mut self, awesome: Lua) {
            // TODO implement
        }
        fn connect_signal_meta(&mut self, awesome: Lua) {
            // TODO implement
        }
        fn disconnect_signal_meta(&mut self, awesome: Lua) {
            // TODO implement
        }
    }
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
        $($(fn $inner(&mut self, lua: Lua);)*),*
    };
}
