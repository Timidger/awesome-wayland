//! Definitions for the equivalent of `lua_class-property`
use super::class::PropF;

pub struct Property {
    /// Name of the property
    pub name: String,
    /// Callback called when the property is found in object creation
    pub new: PropF,
    /// Callback called when the property is found in object __index
    pub index: PropF,
    /// Callback called when the property is found in object __newindex
    pub new_index: PropF
}
