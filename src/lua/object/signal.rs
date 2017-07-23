// TODO I shouldn't use c_void here...<T>?
use libc::c_void;
use std::cmp::{Eq, PartialEq};

pub struct Signal {
    /// Unique ID for the signal
    pub id: u64,
    /// The functions to call for this signal
    pub sigfuncs: Vec<&'static c_void>
}

impl PartialEq for Signal {
    fn eq(&self, other: &Signal) -> bool {
        self.id == other.id
    }
}

impl Eq for Signal {}
