//! Definitions for the equivalent of `lua_class-property`
use super::class::PropF;

pub struct Property {
    /// Name of the property
    name: String,
    /// Callback called when the property is found in object creation
    new: PropF,
    /// Callback called when the property is found in object __index
    index: PropF,
    /// Callback called when the property is found in object __newindex
    new_index: PropF
}
