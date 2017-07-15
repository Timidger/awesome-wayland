//! Callbacks for the `client` object in the Lua libraries

use ::lua::Lua;
use libc::c_int;

pub trait Drawin {
    /* Methods */
    class_methods!(drawin);
    fn __call(&mut self, lua: Lua);
    /* Meta */
    object_methods_meta!(drawin);
    class_methods_meta!();
    fn geometry(&mut self, lua: Lua);
    /* Properties */
    properties!([
        drawable,
        visible,
        ontop,
        cursor,
        x,
        y,
        width,
        height,
        type_,
        shape_bounding,
        shape_clip,
        shape_input
    ]);
}
