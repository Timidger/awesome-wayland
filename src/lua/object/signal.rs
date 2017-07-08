// TODO I shouldn't use c_void here...<T>?
use libc::c_void;
use std::cmp::{Eq, PartialEq};

pub struct Signal {
    /// Unique ID for the signal
    id: u64,
    /// The functions to call for this signal
    sigfuncs: Vec<c_void>
}

impl PartialEq for Signal {
    fn eq(&self, other: &Signal) -> bool {
        self.id == other.id
    }
}

impl Eq for Signal {}
