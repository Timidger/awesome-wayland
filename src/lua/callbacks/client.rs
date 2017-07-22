//! Callbacks for the `client` object in the Lua libraries

use ::lua::Lua;
use libc::c_int;

pub trait Client {
    fn client_add_signal(&mut self, awesome: Lua);
    fn client_connect_signal(&mut self, awesome: Lua);
    fn client_disconnect_signal(&mut self, awesome: Lua);
    fn client_emit_signal(&mut self, awesome: Lua);
    fn client_instances(&mut self, awesome: Lua);
    fn client_set_index_miss_handler(&mut self, awesome: Lua);
    fn client_set_newindex_miss_handler(&mut self, awesome: Lua);
    fn client___call(&mut self, awesome: Lua);
    fn client___tostring_meta(&mut self, awesome: Lua) {
        // TODO implement
    }
    fn client_connect_signal_meta(&mut self, awesome: Lua) {
        // TODO implement
    }
    fn client_disconnect_signal_meta(&mut self, awesome: Lua) {
        // TODO implement
    }
    fn client___index_meta(&mut self, awesome: Lua) -> c_int {
        // TODO luaA_class_index
        0
    }
    fn client___newindex_meta(&mut self, awesome: Lua) -> c_int {
        // TODO luaA_class_newindex
        0
    }
    fn client___get(&mut self, awesome: Lua);
    fn client___index(&mut self, awsemoe: Lua);
    fn client___newindex(&mut self, awsemoe: Lua);
    /* Meta */
    fn client_keys(&mut self, awesome: Lua);
    fn client_isvisible(&mut self, awesome: Lua);
    fn client_geometry(&mut self, awesome: Lua);
    fn client_apply_size_hints(&mut self, awesome: Lua);
    fn client_tags(&mut self, awesome: Lua);
    fn client_kill(&mut self, awesome: Lua);
    fn client_swap(&mut self, awesome: Lua);
    fn client_raise(&mut self, awesome: Lua);
    fn client_lower(&mut self, awesome: Lua);
    fn client_unmanange(&mut self, awesome: Lua);
    fn client_titlebar_top(&mut self, awesome: Lua);
    fn client_titlebar_right(&mut self, awesome: Lua);
    fn client_titlebar_bottom(&mut self, awesome: Lua);
    fn client_titlebar_left(&mut self, awesome: Lua);
    fn client_get_icon(&mut self, awesome: Lua);
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
