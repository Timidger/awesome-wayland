//! The global configuration state.
//! Based off of globalconf.h in the C source.

use libc::{self, c_void};
use std::sync::Mutex;
use std::default::Default;
use ::callbacks::button::ButtonState;
use ::callbacks::client::ClientState;
use ::callbacks::screen::ScreenState;
use ::callbacks::drawin::DrawinState;
use ::callbacks::drawable::DrawableState;
use ::callbacks::tag::TagState;
use ::object::{WindowState, KeyState};
use xcb::ffi::*;
use ::xcb_util_sys::{xcb_key_symbols_t, xcb_key_symbols_alloc};
use ::xcb_util_xrm_sys::{xcb_xrm_database_t, xcb_xrm_database_from_default};
use ::xcb_cursor_sys::xcb_cursor_context_t;
use ::xkbcommon_sys::{xkb_context, xkb_state};
use ::libsn_sys::{SnDisplay, SnMonitorContext};
use ::cairo_xcb::{cairo_xcb_surface_create, _cairo_surface};
use glib_sys::GMainLoop;
use ::cairo::surface::Surface;

#[allow(non_camel_case_types)]
type void_ptr = *mut c_void;

// TODO Remove
#[allow(dead_code)]
/// Main configuration structure
#[repr(C)]
pub struct GlobalConf {
    /// XCB Connection ref
    pub connection: *mut xcb_connection_t,
    /// X Resources DB
    pub xrmdb: *mut xcb_xrm_database_t,
    /// Default screen number
    pub default_screen: i32,
    /// xcb-cursor context
    pub cursor_ctx: *mut xcb_cursor_context_t,
    /// Keys symbol table
    pub keysyms: *mut xcb_key_symbols_t,
    /// Logical screens
    pub screens: Vec<ScreenState>,
    /// The primary screen, access through screen_get_primary()
    pub primary_screen: *mut ScreenState,
    /// Root window key bindings
    pub keys: Vec<KeyState>,
    /// Root window mouse bindings
    pub buttons: Vec<ButtonState>,
    /// Atom for WM_Sn
    pub selection_atom: xcb_atom_t,
    /// Window owning the WM_Sn selection
    pub selection_owner_window: xcb_window_t,
    /// Do we have RandR 1.3 or newer?
    pub have_randr_13: bool,
    /// Do we have RandR 1.5 or newer?
    pub have_randr_15: bool,
    /// Do we have a RandR screen update pending?
    pub screen_need_refresh: bool,
    /// Check for XTest extension
    pub have_xtest: bool,
    /// Check for SHAPE extension
    pub have_shape: bool,
    /// Check for SHAPE extension with input shape support
    pub have_input_shape: bool,
    /// Check for XKB extension
    pub have_xkb: bool,
    pub event_base_shape: u8,
    pub event_base_xkb: u8,
    pub event_base_randr: u8,
    /// Clients list
    pub clients: Vec<ClientState>,
    /// Embedded windows
    pub embedded: Vec<XembedWindow>,
    /// Stack client history
    pub stack: Vec<ClientState>,
    /// All errors messages from loading config files
    pub startup_errors: Vec<String>,
    /// main loop that awesome is running on
    pub g_loop: *mut GMainLoop,
    /// The key grabber function
    pub keygrabber: i32,
    /// The mouse pointer grabber function
    pub mousegrabber: i32,
    /// The drawable that currently contains the pointer
    pub drawable_under_mouse: *mut DrawableState,
    pub focus: Focus,
    /// Drawins
    pub drawins: Vec<DrawinState>,
    /// The startup notification display struct
    pub sndisplay: *mut SnDisplay,
    /// Latest timestamp we got from the X server
    pub timestamp: xcb_timestamp_t,
    pub systray: Systray,
    /// The monitor of startup notifications
    pub snmonitor: *mut SnMonitorContext,
    /// The visual, used to draw
    pub visual: *mut xcb_visualtype_t,
    /// The screen's default visual
    pub default_visual: *mut xcb_visualtype_t,
    /// The screen's information
    pub screen: *mut xcb_screen_t,
    /// A graphic context.
    pub gc: xcb_gcontext_t,
    /// Our default depth
    pub default_depth: u8,
    /// Our default color map
    pub default_cmap: xcb_colormap_t,
    /// Do we have to reban clients?
    pub need_lazy_banning: bool,
    /// Tag list
    pub tags: Vec<TagState>,
    /// List of registered xproperties
    pub xproperties: Vec<xcb_property_t>,
    /// xkb context
    pub xkb_ctx: *mut xkb_context,
    /// xkb state of dead keys on keyboard
    pub xkb_state: *mut xkb_state,
    /// Do we have a pending reload?
    pub xkb_reload_keymap: bool,
    /// Do we have a pending map change?
    pub xkb_map_changed: bool,
    /// Do we have a pending group change?
    pub xkb_group_changed: bool,
    /// The preferred size of client icons for this screen
    pub preferred_icon_size: u32,
    /// Cached wallpaper information
    pub wallpaper: *mut _cairo_surface,
    /// List of enter/leave events to ignore
    pub ignore_enter_leave_events: Vec<SequencePair>,
    pub pending_enter_leave_begin: xcb_void_cookie_t,
    /// List of windows to be destroyed later
    pub destroy_later_windows: Vec<WindowState>,
    /// Pending event that still needs to be handled
    pub pending_event: *mut xcb_generic_event_t,
    /// The exit code that main() will return with
    pub exit_code: i32
}

impl Default for GlobalConf {
    #[allow(non_snake_case)]
    fn default() -> Self {
        unsafe {
            let mut default_screen = 0;
            let NULL =  0 as *mut libc::c_void;
            let connection = xcb_connect(NULL as _, &mut default_screen);
            GlobalConf {
                connection,
                xrmdb: xcb_xrm_database_from_default(connection as _),
                default_screen,
                cursor_ctx: NULL as _,
                keysyms: xcb_key_symbols_alloc(connection as _),
                screens: Vec::new(),
                primary_screen: NULL as _,
                keys: Vec::new(),
                buttons: Vec::new(),
                selection_atom: 0,
                selection_owner_window: 0,
                have_randr_13: false,
                have_randr_15: false,
                screen_need_refresh: false,
                have_xtest: false,
                have_shape: false,
                have_input_shape: false,
                have_xkb: false,
                event_base_shape: 0,
                event_base_xkb: 0,
                event_base_randr: 0,
                clients: Vec::new(),
                embedded: Vec::new(),
                stack: Vec::new(),
                startup_errors: Vec::new(),
                g_loop:  NULL as _,
                keygrabber: 0,
                mousegrabber: 0,
                drawable_under_mouse: NULL as _,
                focus: Focus::default(),
                drawins: Vec::new(),
                sndisplay:  NULL as _,
                timestamp: 0,
                systray: Systray::default(),
                snmonitor:  NULL as _,
                visual:  NULL as _,
                default_visual:  NULL as _,
                screen:  NULL as _,
                gc: 0,
                default_depth: 0,
                default_cmap: 0,
                need_lazy_banning: false,
                tags: Vec::new(),
                xproperties: Vec::new(),
                xkb_ctx:  NULL as _,
                xkb_state:  NULL as _,
                xkb_reload_keymap: false,
                xkb_map_changed: false,
                xkb_group_changed: false,
                preferred_icon_size: 0,
                wallpaper: NULL as _,
                ignore_enter_leave_events: Vec::new(),
                pending_enter_leave_begin: xcb_grab_server(connection),
                destroy_later_windows: Vec::new(),
                pending_event: NULL as _,
                exit_code: 0
            }
        }
    }
}

unsafe impl Send for GlobalConf {}
unsafe impl Sync for GlobalConf {}

lazy_static! {
    pub static ref GLOBAL_CONF: Mutex<GlobalConf> = Mutex::new(GlobalConf::default());
}

#[repr(C)]
pub struct Focus {
    /// Focused client
    pub client: *mut ClientState,
    /// Is there a focuse change pending?
    pub need_update: bool,
    /// When nothing has the input focus, this window actually is focused.
    pub window_no_focus: xcb_window_t
}

impl Default for Focus {
    fn default() -> Self {
        Focus {
            client: ::std::ptr::null_mut(),
            need_update: false,
            window_no_focus: 0
        }
    }
}

#[repr(C)]
pub struct Systray {
    pub window: xcb_window_t,
    /// ATOM for _NET_SYSTEM_TRAY_%d
    pub atom: xcb_atom_t,
    /// Do we own the systray selection
    pub registered: bool,
    /// Systray window parent
    pub parent: *mut DrawinState,
    /// Background color
    pub background_pixel: u32
}

impl Default for Systray {
    fn default() -> Self {
        Systray {
            window: 0,
            atom: 0,
            registered: false,
            parent: ::std::ptr::null_mut(),
            background_pixel: 0
        }
    }
}

#[repr(C)]
pub struct SequencePair {
    begin: xcb_void_cookie_t,
    end: xcb_void_cookie_t
}


// TODO move to xembed.rs
#[repr(C)]
pub struct XEmbedInfo {
    version: u64,
    flags: u64
}

#[repr(C)]
pub struct XembedWindow {
    win: *mut xcb_window_t,
    info: *mut XEmbedInfo
}
