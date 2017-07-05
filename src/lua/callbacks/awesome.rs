//! Callbacks for the Awesome object in the Lua libraries

use lua_sys::*;
use ::lua::Lua;

pub trait Awesome {
    fn new() -> Self;
    fn quit(&mut self, awesome: Lua);
    fn exec(&mut self, awesome: Lua);
    fn spawn(&mut self, awesome: Lua);
    fn restart(&mut self, awesome: Lua);
    fn connect_signal(&mut self, awesome: Lua);
    fn disconnect_signal(&mut self, awesome: Lua);
    fn emit_signal(&mut self, awesome: Lua);
    fn systray(&mut self, awesome: Lua);
    fn load_image(&mut self, awesome: Lua);
    fn set_preferred_icon_size(&mut self, awesome: Lua);
    fn register_xproperty(&mut self, awesome: Lua);
    fn set_xproperty(&mut self, awesome: Lua);
    fn get_xproperty(&mut self, awesome: Lua);
    fn __index(&mut self, awesome: Lua);
    fn __newindex(&mut self, awesome: Lua);
    fn xkb_set_layout_group(&mut self, awesome: Lua);
    fn xkb_get_layout_groub(&mut self, awesome: Lua);
    fn xkb_get_group_names(&mut self, awesome: Lua);
    fn xrdb_get_value(&mut self, awesome: Lua);
    fn kill(&mut self, awesome: Lua);
    fn sync(&mut self, awesome: Lua);
}

/// Registers a lua function in a LuaL_reg
///
/// Note that errors for registering the method is up to the caller
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

struct AwesomeImpl;

impl Awesome for AwesomeImpl {
    fn new() -> Self {AwesomeImpl}
    fn quit(&mut self, awesome: Lua) {}
    fn exec(&mut self, awesome: Lua) {}
    fn spawn(&mut self, awesome: Lua) {}
    fn restart(&mut self, awesome: Lua) {}
    fn connect_signal(&mut self, awesome: Lua) {}
    fn disconnect_signal(&mut self, awesome: Lua) {}
    fn emit_signal(&mut self, awesome: Lua) {}
    fn systray(&mut self, awesome: Lua) {}
    fn load_image(&mut self, awesome: Lua) {}
    fn set_preferred_icon_size(&mut self, awesome: Lua) {}
    fn register_xproperty(&mut self, awesome: Lua) {}
    fn set_xproperty(&mut self, awesome: Lua) {}
    fn get_xproperty(&mut self, awesome: Lua) {}
    fn __index(&mut self, awesome: Lua) {}
    fn __newindex(&mut self, awesome: Lua) {}
    fn xkb_set_layout_group(&mut self, awesome: Lua) {}
    fn xkb_get_layout_groub(&mut self, awesome: Lua) {}
    fn xkb_get_group_names(&mut self, awesome: Lua) {}
    fn xrdb_get_value(&mut self, awesome: Lua) {}
    fn kill(&mut self, awesome: Lua) {}
    fn sync(&mut self, awesome: Lua) {}
}

fn test_fn(a: AwesomeImpl, mut l: Lua) {
    let mut A = a;
    register_awesome!(AwesomeImpl, A, l);
    A.sync(l.clone());
    A.load_image(l.clone());
    A.quit(l.clone());
}
