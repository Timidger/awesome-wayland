//! These callbacks should be registered in order to be used by the Lua libraries.
//! Only one struct can be used by each interface, but the interfaces can share
//! as many structs as they want. It's recommended you have one struct per
//! interface, though you can just use one struct if you wish.

mod awesome;
pub use self::awesome::Awesome;
pub trait Beautiful {}
pub trait Button {}
pub trait Client {}
pub trait Drawin {}
pub trait Keygrabber {}
pub trait Mousegrabber {}
pub trait Mouse {}
pub trait Root {}
pub trait Screen {}
pub trait Tag {}
