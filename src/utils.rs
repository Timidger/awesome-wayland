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

/// Registers a lua class' meta table.
/// E.g adds __tostring, connect_signal, etc.
/// For the lua class methods, us register_lua_class_meth
#[macro_export]
macro_rules! register_lua_class_meta {
    ($([ $( $inner:ident ),+ ])+) => {{
        // TODO Remove this when I'm less lazy.
        $($(unsafe extern "C" fn $inner(lua: *mut lua_State) -> i32 {0})*),*
        unsafe extern "C" fn __tostring(lua: *mut lua_State) -> i32 {0}
        unsafe extern "C" fn connect_signal(lua: *mut lua_State) -> i32 {0}
        unsafe extern "C" fn disconnect_signal(lua: *mut lua_State) -> i32 {0}
        unsafe extern "C" fn emit_signal(lua: *mut lua_State) -> i32 {0}
            [
                register_lua!(__tostring),
                register_lua!(connect_signal),
                register_lua!(disconnect_signal),
                register_lua!(emit_signal),
                $($(register_lua!($inner)),*),*,
                ::lua_sys::luaL_Reg {
                    name: ::std::ptr::null(),
                    func: None
                },
            ]
    }}
}


/// Registers a lua function in a LuaL_reg to call the given global identifier.
///
/// Do NOT use this macro directly, use the register_for_lua! instead.
#[macro_export]
macro_rules! register_lua {
    ($global_name:ident, $([ $( $inner:ident ),+ ])+) => {{
        use ::awesome_wayland::callbacks::Awesome;
        $($(unsafe extern "C" fn $inner(lua: *mut lua_State) -> i32 {
            let mut callback = $global_name.lock()
                .expect("Could not lock user defined callback object");
            callback.callbacks.$inner(::Lua::from_ptr(lua));
            0
        })*),*
            [
                $($(register_lua!($inner)),*),*,
                ::lua_sys::luaL_Reg {
                    name: ::std::ptr::null(),
                    func: None
                },
            ]
    }};
    ($name:ident) => {
        ::lua_sys::luaL_Reg {
            name: c_str!(stringify!($name)),
            func: Some($name)
        }
    }
}

/// Registers a struct that implements [Awesome](callbacks/trait.Awesome.html)
///
/// Note that errors for registering the method is up to the caller
///
/// Do NOT use this macro directly, use the register_for_lua! instead.
#[macro_export]
macro_rules! register_awesome {
    ($callback_impl:ident, $global_name:ident) => {{
        let lua_reg = {
            register_lua!($global_name, [
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
            ])
        };
        LUA.register_methods("awesome\0", &lua_reg)
    }}
}

/// Defines the methods associated with classes. These methods have default
/// implementations, but can be defined by the user if they so choose.
#[macro_export]
macro_rules! class_methods {
    () => {
        // TODO Give these the default impls
        /* LUA_CLASS_META methods */
        fn __index(&mut self, awesome: Lua) -> c_int {
            // TODO luaA_class_index
            0
        }
        fn __newindex(&mut self, awesome: Lua) -> c_int {
            // TODO luaA_class_newindex
            0
        }
    }
}

/// Defines the meta methods associated with classes. These methods have default
/// implementations, but can be defined by the user if they so choose.
macro_rules! class_methods_meta {
    ($prefix:ident) => {
        /* LUA_OBJECT_META methods */
        fn __tostring(&mut self, awesome: Lua) {
            // TODO implement
        }
        fn connect_signal(&mut self, awesome: Lua) {
            // TODO implement
        }
        fn disconnect_signal(&mut self, awesome: Lua) {
            // TODO implement
        }
        fn emit_signal(&mut self, awesome: Lua);
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
