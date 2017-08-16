//! These callbacks should be registered in order to be used by the Lua libraries.
//! Only one struct can be used by each interface, but the interfaces can share
//! as many structs as they want. It's recommended you have one struct per
//! interface, though you can just use one struct if you wish.

// Callback modules
pub mod awesome;
pub mod button;
pub mod client;
pub mod drawin;
pub mod keygrabber;
pub mod mousegrabber;
pub mod mouse;
pub mod root;
pub mod screen;
pub mod tag;

pub use self::awesome::Awesome;
pub use self::button::Button;
pub use self::client::Client;
pub use self::drawin::Drawin;
pub use self::keygrabber::Keygrabber;
pub use self::mousegrabber::Mousegrabber;
pub use self::mouse::Mouse;
pub use self::root::Root;
pub use self::screen::Screen;
pub use self::tag::Tag;
