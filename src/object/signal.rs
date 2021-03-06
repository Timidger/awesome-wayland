use libc::c_void;
use std::cmp::{Eq, PartialEq};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::sync::Mutex;
use std::ffi::CStr;

lazy_static! {
    pub static ref GLOBAL_SIGNALS: Mutex<Vec<Signal>> = Mutex::new(vec![]);
}

pub struct SignalFunc(*mut c_void);

unsafe impl Send for SignalFunc {}
unsafe impl Sync for SignalFunc {}

pub struct Signal {
    /// Unique ID for the signal
    pub id: u64,
    /// The functions to call for this signal
    pub sigfuncs: Vec<SignalFunc>
}

unsafe impl Send for Signal {}
unsafe impl Sync for Signal {}

impl PartialEq for Signal {
    fn eq(&self, other: &Signal) -> bool {
        self.id == other.id
    }
}

impl Eq for Signal {}

use lua_sys::*;
use libc;
use lua::{self, luaA};
pub unsafe fn signal_object_emit(lua: *mut lua_State, signals: &[Signal],
                          name: &str, nargs: libc::c_int) {
    let mut hasher = DefaultHasher::new();
    hasher.write(name.as_bytes());
    let id = hasher.finish();
    if let Some(sig) = signals.iter().find(|sig| sig.id == id) {
        let nbfunc = sig.sigfuncs.len() as i32;
        luaL_checkstack(lua, nbfunc + nargs + 1, c_str!("too much signal"));
        /* Push all functions and then execute, because this list can change
         * while executing funcs. */
        for func in &sig.sigfuncs {
            luaA::object_push(lua, func.0);
        }
        for i in 0..nbfunc {
            /* push all args */
            for _ in 0..nargs {
                lua_pushvalue(lua, - nargs - nbfunc + i);
            }
            /* push first function */
            lua_pushvalue(lua, - nargs - nbfunc + i);
            /* remove this first function */
            lua::lua_remove(lua, - nargs - nbfunc -1 + i);
            luaA::dofunction(lua, nargs, 0);
        }
    }
    lua_pop(lua, nargs);
}

pub unsafe fn signal_connect(signals: &mut Vec<Signal>, name: *const libc::c_char,
                             ptr: *mut libc::c_void) {
    let mut hasher = DefaultHasher::new();
    let c_name = CStr::from_ptr(name);
    let c_name_str = c_name.to_str().unwrap();
    hasher.write(c_name_str.as_bytes());
    ::std::mem::forget(c_name);
    ::std::mem::forget(c_name_str);
    let id = hasher.finish();
    if let Some(mut sig) = signals.iter_mut().find(|sig| sig.id == id) {
        sig.sigfuncs.push(SignalFunc(ptr));
        return;
    }
    let sig = Signal {
        id,
        sigfuncs: vec![]
    };
    signals.push(sig);
}

pub unsafe fn signal_disconnect(signals: &mut Vec<Signal>,
                                name: *const libc::c_char,
                                ptr: *mut libc::c_void) -> libc::c_int {
    let mut hasher = DefaultHasher::new();
    let c_name = CStr::from_ptr(name);
    let c_name_str = c_name.to_str().unwrap();
    hasher.write(c_name_str.as_bytes());
    ::std::mem::forget(c_name);
    ::std::mem::forget(c_name_str);
    let id = hasher.finish();
    if let Some(index) = signals.iter().position(|sig| sig.id == id) {
        for i in 0..signals[index].sigfuncs.len() {
            if signals[index].sigfuncs[i].0 == ptr {
                // TODO This might be a memory leak
                signals.remove(i);
                break;
            }
        }
        1
    } else {
        0
    }
}
