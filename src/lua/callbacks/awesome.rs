//! Callbacks for the Awesome object in the Lua libraries

use ::lua::Lua;

pub trait Awesome {
    fn quit(awesome: Lua);
    fn exec(awesome: Lua);
    fn spawn(awesome: Lua);
    fn restart(awesome: Lua);
    fn connect_signal(awesome: Lua);
    fn disconnect_signal(awesome: Lua);
    fn emit_signal(awesome: Lua);
    fn systray(awesome: Lua);
    fn load_image(awesome: Lua);
    fn set_preferred_icon_size(awesome: Lua);
    fn register_xproperty(awesome: Lua);
    fn set_xproperty(awesome: Lua);
    fn get_xproperty(awesome: Lua);
    fn __index(awesome: Lua);
    fn __newindex(awesome: Lua);
    fn xkb_set_layout_group(awesome: Lua);
    fn xkb_get_layout_groub(awesome: Lua);
    fn xkb_get_group_names(awesome: Lua);
    fn xrdb_get_value(awesome: Lua);
    fn kill(awesome: Lua);
    fn sync(awesome: Lua);
}
