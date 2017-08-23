//! Definition for the Window class. This class is a super class to various
//! objects, including drawable.

use ::object::signal::Signal;
use libc::{c_void};
use ::callbacks::button::ButtonState;

// TODO Remove this lint
#[allow(unused_variables)]
#[repr(C)]
pub struct WindowState {
    pub signals: Vec<Signal>,
    /// The X window number
    pub window: i32,
    /// The frame window, might be XCB_NONE
    pub frame_window: (),
    /// Opacity
    pub opacity: f64,
    /// Struct
    pub strut: Struct,
    /// Button bindings
    pub buttons: Vec<*mut ButtonState>,
    /// Do we have pending border changes?
    pub border_need_update: bool,
    /// Border color
    pub border_color: (),
    /// The window type
    pub window_type: (),
    /// The border width callback
    pub border_width_callback: fn(*mut c_void, u16, u16)
}


#[repr(C)]
pub struct Struct {
    pub left: u16,
    pub right: u16,
    pub top: u16,
    pub bottom: u16,
    pub left_start_y: u16,
    pub left_end_y: u16,
    pub right_start_y: u16,
    pub right_end_y: u16,
    pub top_start_x: u16,
    pub top_end_x: u16,
    pub bottom_start_x: u16,
    pub bottom_end_x: u16
}
