use ::object::Signal;
use ::xcb_cursor_sys::{xcb_keycode_t, xcb_keysym_t};

#[repr(C)]
pub struct KeyState {
    pub signals: Vec<Signal>,
    pub modifiers: u16,
    pub keysym: xcb_keysym_t,
    pub keycode: xcb_keycode_t
}
