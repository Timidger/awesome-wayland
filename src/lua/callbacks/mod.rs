//! The callbacks defined by the user that is usde when called
//! through the Lua library from user Lua code.

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
