//! These callbacks should be registered in order to be used by the Lua libraries.
//! Only one struct can be used by each interface, but the interfaces can share
//! as many structs as they want. It's recommended you have one struct per
//! interface, though you can just use one struct if you wish.

mod awesome;
mod button;
mod client;
mod drawin;
mod keygrabber;
mod mousegrabber;
mod mouse;
mod root;
mod screen;
mod tag;

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

// TODO I don't think this is needed
pub trait Beautiful {}
