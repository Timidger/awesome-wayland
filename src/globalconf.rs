//! The global configuration state.
//! Based off of globalconf.h in the C source.

use libc::{self, c_void};
use std::sync::Mutex;
use std::default::Default;
use ::callbacks::button::ButtonState;
use ::callbacks::client::ClientState;
use ::callbacks::screen::ScreenState;

#[allow(non_camel_case_types)]
type void_ptr = *mut c_void;

// NOTE We use () where we don't think we'll need this value, and it'll
// probably be removed earlier.

// NOTE We use void_ptr where we aren't sure what the type should be,
// mostly because it's some xcb specific thing and we aren't sure how much
// we are going to mock that quite yet.

// For filling this in, please see globalconf.h in the original awesome repo.

// TODO Remove
#[allow(dead_code)]
/// Main configuration structure
pub struct GlobalConf {
    /// XCB Connection ref
    pub connection: (),
    /// X Resources DB
    pub xrmdb: void_ptr,
    /// Default screen number
    pub default_screen: i32,
    /// xcb-cursor context
    pub cursor_ctx: (),
    /// Keys symbol table
    pub keysms: void_ptr,
    /// Logical screens
    pub screens: Vec<ScreenState>,
    /// The primary screen, access through screen_get_primary()
    pub primary_screen: Vec<void_ptr>,
    /// Root window key bindings
    pub keys: Vec<void_ptr>,
    /// Root window mouse bindings
    pub buttons: Vec<ButtonState>,
    /// Atom for WM_Sn
    pub selection_atom: (),
    /// Window owning the WM_Sn selection
    pub selection_owner_window: (),
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
    pub embedded: Vec<()>,
    /// Stack client history
    pub stack: Vec<ClientState>,
    /// All errors messages from loading config files
    pub startup_errors: Vec<String>,
    /// main loop that awesome is running on
    pub g_loop: void_ptr,
    /// The key grabber function
    pub keygrabber: i32,
    /// The mouse pointer grabber function
    pub mousegrabber: i32,
    /// The drawable that currently contains the pointer
    pub drawable_under_mouse: (),
    /// Drawins
    // TODO Replace wiht DrawinState when it's done
    pub drawins: Vec<()>,
    /// The startup notification display struct
    pub sndisplay: void_ptr,
    /// Latest timestamp we got from the X server
    pub timestamp: (),
    /// The monitor of startup notifications
    pub snmonitor: void_ptr,
    /// The visual, used to draw
    pub visual: void_ptr,
    /// The screen's default visual
    pub default_visual: void_ptr,
    /// The screen's information
    pub screen: void_ptr,
    /// A graphic context.
    pub gc: (),
    /// Our default depth
    pub default_depth: u8,
    /// Our default color map
    pub default_cmap: (),
    /// Do we have to reban clients?
    pub need_lazy_banning: bool,
    /// Tag list
    // TODO Replace with TagState when it's done
    pub tags: Vec<()>,
    /// List of registered xproperties
    pub xproperties: Vec<()>,
    /// xkb context
    pub xkb_ctx: void_ptr,
    /// xkb state of dead keys on keyboard
    pub xkb_state: void_ptr,
    /// Do we have a pending reload?
    pub xkb_reload_keymap: bool,
    /// Do we have a pending map change?
    pub xkb_map_changed: bool,
    /// Do we have a pending group change?
    pub xkb_group_changed: bool,
    /// The preferred size of client icons for this screen
    pub preferred_icon_size: u32,
    /// Cached wallpaper information
    // TODO Replace with cairo surface pointer
    pub wallpaper: void_ptr,
    /// List of enter/leave events to ignore
    pub ignore_enter_leave_events: Vec<()>,
    pub pending_enter_leave_begin: (),
    /// List of windows to be destroyed later
    pub destroy_later_windows: Vec<()>,
    /// Pending event that still needs to be handled
    pub pending_event: void_ptr,
    /// The exit code that main() will return with
    pub exit_code: i32
}

impl Default for GlobalConf {
    #[allow(non_snake_case)]
    fn default() -> Self {
        let NULL =  0 as *mut libc::c_void;
        GlobalConf {
            connection: (),
            xrmdb:  NULL as _,
            default_screen: 0,
            cursor_ctx: (),
            keysms:  NULL as _,
            screens: Vec::new(),
            primary_screen: Vec::new(),
            keys: Vec::new(),
            buttons: Vec::new(),
            selection_atom: (),
            selection_owner_window: (),
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
            drawable_under_mouse: (),
            drawins: Vec::new(),
            sndisplay:  NULL as _,
            timestamp: (),
            snmonitor:  NULL as _,
            visual:  NULL as _,
            default_visual:  NULL as _,
            screen:  NULL as _,
            gc: (),
            default_depth: 0,
            default_cmap: (),
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
            pending_enter_leave_begin: (),
            destroy_later_windows: Vec::new(),
            pending_event: NULL as _,
            exit_code: 0
        }
    }
}

unsafe impl Send for GlobalConf {}
unsafe impl Sync for GlobalConf {}

lazy_static! {
    pub static ref GLOBAL_CONF: Mutex<GlobalConf> = Mutex::new(GlobalConf::default());
}
