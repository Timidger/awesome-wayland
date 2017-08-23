//! Callbacks for the `client` object in the Lua libraries

use ::luaA::{self, CLIENT_CLASS, SCREEN_CLASS, area_t, object_push};
use ::globalconf::GLOBAL_CONF;
use ::lua::Lua;
use libc::{c_int, c_void};
use ::object::class::{Class, Object};
use ::object::window::WindowState;
use ::callbacks::drawable::DrawableState;
use ::callbacks::screen::ScreenState;
use std::ffi::{CString};

LUA_OBJECT_FUNCS!(luaA::CLIENT_CLASS, Class, client_new);
LUA_CLASS_FUNCS!(luaA::CLIENT_CLASS,
                 client_class_add_signal,
                 client_class_connect_signal,
                 client_class_disconnect_signal,
                 client_class_emit_signal,
                 client_class_instances,
                 client_set_index_miss_handler,
                 client_set_newindex_miss_handler);

#[repr(C)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct ClientState {
    pub window: WindowState,
    /// Window we use for input focus and no-input clients
    pub nofocus_window: (),
    /// Client logical screen
    pub screen: *mut c_void,
    /// Client name
    pub name: String,
    pub alt_name: String,
    pub icon_name: String,
    pub alt_icon_name: String,
    /// WM_CLASS stuff
    pub class: String,
    pub instance: String,
    /// Window geometry
    pub geometry: area_t,
    /// Old window geometry currently configured in X11
    pub x11_client_geometry: area_t,
    pub x11_frame_geometry: area_t,
    /// Got a configure request and have to call
    /// client_send_configure() if its ignored?
    pub got_configure_request: bool,
    /// Startup ID
    pub startup_id: String,
    /// True if the client is sticky
    pub sticky: bool,
    /// Has urgency hint
    pub urgent: bool,
    /// True if the client is hidden
    pub hidden: bool,
    /// True if the client is minimized
    pub minimized: bool,
    /// True if the client is fullscreen
    pub fullscreen: bool,
    /// True if the client is maximized horizontally
    pub maximized_horizontal: bool,
    /// True if the client is maximized vertically
    pub maximized_vertical: bool,
    /// True if the client is maximized both horizontally and vertically by the
    /// the user
    pub maximized: bool,
    /// True if the client is above others
    pub above: bool,
    /// True if the client is below others
    pub below: bool,
    /// True if the client is modal
    pub modal: bool,
    /// True if the client is on top
    pub ontop: bool,
    /// True if a client is banned to a position outside the viewport.
    /// Note that the geometry remains unchanged and that the window is
    /// still mapped.
    pub isbanned: bool,
    /// true if the client must be skipped from task bar client list
    pub skip_taskbar: bool,
    /// True if the client cannot have focus
    pub nofocus: bool,
    /// True if the client is focusable.  Overrides nofocus, and can be set
    /// from Lua.
    pub focusable: bool,
    pub focusable_set: bool,
    /// True if the client window has a _NET_WM_WINDOW_TYPE proeprty
    pub has_NET_WM_WINDOW_TYPE: bool,
    /// Window of the group leader
    // TODO WRONG TYPE, done to make it compile...
    pub group_window: i64,
    /// Window holding command needed to start it (session management related)
    // TODO WRONG TYPE, done to make it compile...
    pub leader_window: i64,
    /// Client's WM_PROTOCOLS property
    pub protocols: (),
    /// Key bindings
    pub keys: Vec<()>,
    /// Icons
    pub icons: Vec<()>,
    /// True if we ever got an icon from _NET_WM_ICON
    pub have_ewmh_icon: bool,
    /// Size hints
    pub size_hints: (),
    /// The visualtype that c->window uses
    pub visualtype: (),
    /// Do we honor the client's size hints?
    pub size_hints_honor: bool,
    /// Machine the client is running on.
    pub machine: String,
    /// Role of the client
    pub role: String,
    /// Client pid
    pub pid: u32,
    /// Window it is transient for
    pub transient_for: *mut ClientState,
    /// Value of WM_TRANSIENT_FOR
    pub transient_for_window: (),
    /// Titelbar information
    pub titlebar: Vec<TitleBar>
}

#[allow(dead_code)]
pub struct TitleBar {
    /// The size of this bar.
    size: u16,
    /// The drawable for this bar.
    drawable: *mut DrawableState
}

/// Wrapper around pushstring that converts a Rust string to a *const i8
unsafe fn lua_pushstring_wrapper(lua: *mut lua_State, input: String)
                          -> *const libc::c_char {
    let input = CString::new(input).unwrap();
    let res = lua_pushstring(lua,input.as_ptr());
    ::std::mem::forget(input);
    return res;
}

LUA_OBJECT_EXPORT_PROPERTIES!(ClientState, [
    client_get_class; class; lua_pushstring_wrapper,
    client_get_instance; instance; lua_pushstring_wrapper,
    client_get_role; role; lua_pushstring_wrapper,
    client_get_transient_for; transient_for; object_push,
    client_get_skip_taskbar; skip_taskbar; lua_pushboolean,
    client_get_leader_window; leader_window; lua_pushinteger,
    client_get_group_window; group_window; lua_pushinteger,
    client_get_hidden; hidden; lua_pushboolean,
    client_get_minimized; minimized; lua_pushboolean,
    client_get_fullscreen; fullscreen; lua_pushboolean,
    client_get_modal; modal; lua_pushboolean,
    client_get_ontop; ontop; lua_pushboolean,
    client_get_urgent; urgent; lua_pushboolean,
    client_get_above; above; lua_pushboolean,
    client_get_below; below; lua_pushboolean,
    client_get_sticky; sticky; lua_pushboolean,
    client_get_size_hints_honor; size_hints_honor; lua_pushboolean,
    client_get_maximized_horizontal; maximized_horizontal; lua_pushboolean,
    client_get_maximized_vertical; maximized_vertical; lua_pushboolean,
    client_get_maximized; maximized; lua_pushboolean,
    client_get_startup_id; startup_id; lua_pushstring_wrapper
]);

LUA_OBJECT_EXPORT_OPTIONAL_PROPERTIES!(ClientState, [
    client_get_screen; screen; object_push; ptr::null_mut(),
    client_get_machine; machine; lua_pushstring_wrapper; "",
    client_get_pid; pid; lua_pushinteger; 0
]);

#[allow(non_snake_case)]
pub trait Client {

    fn client_add_signal(&self, lua: &Lua) -> c_int {
        unsafe {
            client_class_add_signal(lua.0)
        }
    }


    fn client_connect_signal(&self, lua: &Lua) -> c_int {
        unsafe {
            client_class_connect_signal(lua.0)
        }
    }


    fn client_disconnect_signal(&self, lua: &Lua) -> c_int {
        unsafe {
            client_class_disconnect_signal(lua.0)
        }
    }


    fn client_emit_signal(&self, lua: &Lua) -> c_int {
        unsafe {
            client_class_emit_signal(lua.0)
        }
    }


    fn client_instances(&self, lua: &Lua) -> c_int {
        unsafe {
            client_class_instances(lua.0)
        }
    }


    fn client_set_index_miss_handler(&self, lua: &Lua) -> c_int {
        unsafe {
            client_set_index_miss_handler(lua.0)
        }
    }


    fn client_set_newindex_miss_handler(&self, lua: &Lua) -> c_int {
        unsafe {
            client_set_newindex_miss_handler(lua.0)
        }
    }


    fn client___call(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::class_new(lua.0, &*CLIENT_CLASS)
        }
    }


    fn client___tostring_meta(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::object_tostring(lua.0)
        }
    }

    fn client_connect_signal_meta(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::object_connect_signal_simple(lua.0)
        }
    }

    fn client_disconnect_signal_meta(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::object_disconnect_signal_simple(lua.0)
        }
    }

    fn client_emit_signal_meta(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::object_emit_signal_simple(lua.0)
        }
    }

    fn client___index_meta(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::class_index(lua.0)
        }
    }

    fn client___newindex_meta(&self, lua: &Lua) -> c_int {
        unsafe {
            luaA::class_newindex(lua.0)
        }
    }

    fn client_get(&self, lua: &Lua) -> c_int {
        unsafe {
            client_get(lua.0)
        }
    }

    fn client___index(&self, lua: &Lua) -> c_int;

    fn client___newindex(&self, lua: &Lua) -> c_int;

    /* Meta */
    fn client_keys(&self, lua: &Lua) -> c_int;

    fn client_isvisible(&self, lua: &Lua) -> c_int;

    fn client_geometry(&self, lua: &Lua) -> c_int;

    fn client_apply_size_hints(&self, lua: &Lua) -> c_int;

    fn client_tags(&self, lua: &Lua) -> c_int;

    fn client_kill(&self, lua: &Lua) -> c_int;

    fn client_swap(&self, lua: &Lua) -> c_int;

    fn client_raise(&self, lua: &Lua) -> c_int;

    fn client_lower(&self, lua: &Lua) -> c_int;

    fn client_unmanange(&self, lua: &Lua) -> c_int;

    fn client_titlebar_top(&self, lua: &Lua) -> c_int;

    fn client_titlebar_right(&self, lua: &Lua) -> c_int;

    fn client_titlebar_bottom(&self, lua: &Lua) -> c_int;

    fn client_titlebar_left(&self, lua: &Lua) -> c_int;

    fn client_get_icon(&self, lua: &Lua) -> c_int;
}

pub unsafe fn client_get(lua: *mut lua_State) -> libc::c_int {
    let mut i = 1;
    let screen = if ::lua::lua_isnonornil(lua, 1) {
        checkscreen(lua, 1)
    } else { None };
    let stacked = if ::lua::lua_isnonornil(lua, 2) {
        luaA::checkboolean(lua, 2) != 0
    } else { false };
    lua_newtable(lua);
    let global_conf = GLOBAL_CONF.try_lock().unwrap();
    if stacked {
        for stack in &global_conf.stack {
            if screen.is_none() || stack.screen == screen.unwrap() as _ {
                luaA::object_push(lua, stack as *const _ as *mut ClientState as _);
                lua_rawseti(lua, -2, i);
                i += 1;
            }
        }
    } else {
        for client in &global_conf.clients {
            if screen.is_none() || client.screen == screen.unwrap() as _ {
                luaA::object_push(lua, client as *const _ as *mut ClientState as _);
                lua_rawseti(lua, -2, i);
                i += 1;
            }
        }
    }
    1
}

pub unsafe fn checkscreen(lua: *mut lua_State, sidx: libc::c_int)
                            -> Option<*mut ScreenState> {
    let mut global_conf = GLOBAL_CONF.try_lock().unwrap();
    if lua_isnumber(lua, sidx) != 0 {
        let screen = lua_tointeger(lua, sidx);
        if screen < 1 || screen as usize > global_conf.screens.len() {
            luaL_error(lua, c_str!("invalid screen number"));
        }
        return Some(&mut global_conf.screens[screen as usize - 1] as _);
    } else {
        let mut screen_class = SCREEN_CLASS.try_read().unwrap();
        let class_ptr = &mut screen_class as *mut _ as *mut Class;
        return Some(luaA::checkudata(lua, sidx, class_ptr) as _)
    }
}
