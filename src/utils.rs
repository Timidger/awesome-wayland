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


/// Registers a lua function in a LuaL_reg
macro_rules! register_lua {
    ($([ $( $inner:ident ),+ ])+) => {{
        // TODO Remove this when I'm less lazy.
        $($(unsafe extern "C" fn $inner(lua: *mut lua_State) -> i32 {0})*),*
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
