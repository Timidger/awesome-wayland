//! Utilites to make my life easier

/// A way to put C-literals in Rust code.
/// Does not accept byte strings.
///
/// #Usage
/// foo(c_str!("my string"));
macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const i8
    }
}

/// Registers a lua class' meta table.
/// E.g adds __tostring, connect_signal, etc.
/// For the lua class methods, us register_lua_class_meth
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
                luaL_Reg {
                    name: ::std::ptr::null(),
                    func: None
                },
            ]
    }}
}


/// Registers a lua function in a LuaL_reg to call the given global identifier.
///
/// Do NOT use this macro directly, use the register_* macros instead.
macro_rules! register_lua {
    ($global_name:ident, $([ $( $inner:ident ),+ ])+) => {{
        $($(unsafe extern "C" fn $inner(lua: *mut lua_State) -> i32 {
            let mut callback = $global_name.lock()
                .expect("Could not lock user defined callback object");
            callback.$inner(::lua::Lua::from_ptr(lua));
            0
        })*),*
            [
                $($(register_lua!($inner)),*),*,
                luaL_Reg {
                    name: ::std::ptr::null(),
                    func: None
                },
            ]
    }};
    ($name:ident) => {
        luaL_Reg {
            name: c_str!(stringify!($name)),
            func: Some($name)
        }
    }
}

/// Registers a struct that implements [Awesome](callbacks/trait.Awesome.html)
///
/// Note that errors for registering the method is up to the caller
#[macro_export]
macro_rules! register_awesome {
    ($callback_impl:ident, $global_name:ident, $lua:expr) => {{
        lazy_static! {
            static ref $global_name: ::std::sync::Mutex<$callback_impl> =
                ::std::sync::Mutex::new($callback_impl::new());
        }
        let lua_reg = register_lua!($global_name, [
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
            xkb_get_layout_groub,
            xkb_get_group_names,
            xrdb_get_value,
            kill,
            sync
        ]);
        $lua.register_methods("awesome", &lua_reg)
    }}
}

