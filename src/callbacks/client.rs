//! Callbacks for the `client` object in the Lua libraries

use super::default;
use ::lua::Lua;
use libc::c_int;

pub trait Client {
    fn client_add_signal(&self, lua: Lua) -> c_int;
    fn client_connect_signal(&self, lua: Lua) -> c_int;
    fn client_disconnect_signal(&self, lua: Lua) -> c_int;
    fn client_emit_signal(&self, lua: Lua) -> c_int;
    fn client_instances(&self, lua: Lua) -> c_int;
    fn client_set_index_miss_handler(&self, lua: Lua) -> c_int;
    fn client_set_newindex_miss_handler(&self, lua: Lua) -> c_int;
    fn client___call(&self, lua: Lua) -> c_int;
    fn client___tostring_meta(&self, lua: Lua) -> c_int {
        default::__tostring_meta(lua)
    }
    fn client_connect_signal_meta(&self, lua: Lua) -> c_int {
        default::connect_signal_meta(lua)
    }
    fn client_disconnect_signal_meta(&self, lua: Lua) -> c_int {
        default::disconnect_signal_meta(lua)
    }
    fn client___index_meta(&self, lua: Lua) -> c_int {
        default::__index_meta(lua)
    }
    fn client___newindex_meta(&self, lua: Lua) -> c_int {
        default::__newindex_meta(lua)
    }
    fn client_get(&self, lua: Lua) -> c_int;
    fn client___index(&self, awsemoe: Lua) -> c_int;
    fn client___newindex(&self, awsemoe: Lua) -> c_int;
    /* Meta */
    fn client_keys(&self, lua: Lua) -> c_int;
    fn client_isvisible(&self, lua: Lua) -> c_int;
    fn client_geometry(&self, lua: Lua) -> c_int;
    fn client_apply_size_hints(&self, lua: Lua) -> c_int;
    fn client_tags(&self, lua: Lua) -> c_int;
    fn client_kill(&self, lua: Lua) -> c_int;
    fn client_swap(&self, lua: Lua) -> c_int;
    fn client_raise(&self, lua: Lua) -> c_int;
    fn client_lower(&self, lua: Lua) -> c_int;
    fn client_unmanange(&self, lua: Lua) -> c_int;
    fn client_titlebar_top(&self, lua: Lua) -> c_int;
    fn client_titlebar_right(&self, lua: Lua) -> c_int;
    fn client_titlebar_bottom(&self, lua: Lua) -> c_int;
    fn client_titlebar_left(&self, lua: Lua) -> c_int;
    fn client_get_icon(&self, lua: Lua) -> c_int;
    /* Properties */
    properties!([
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
