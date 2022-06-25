#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CStr;

use libc::c_void;

pub mod bindings;
pub mod console;

#[macro_use]
pub mod macros {
    #[macro_export]
    macro_rules! unsafe_cstr {
        ($e: expr) => {{
            union Transmute {
                src: &'static str,
                dst: &'static CStr,
            }

            const _TRANSMUTE_CHECK: [(); std::mem::size_of::<&'static str>()]
                = [(); std::mem::size_of::<&'static CStr>()];

            const RES: &'static CStr = unsafe {
                (Transmute { src: concat!($e, "\0") }).dst
            };

            RES
        }}
    }
}

pub const PROTOTYPE: &CStr = unsafe_cstr!("prototype");
pub const OBJECT: &CStr = unsafe_cstr!("Object");
pub const NULL: *mut c_void = 0 as *mut c_void;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
