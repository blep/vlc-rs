#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

pub mod valist;

// The bindings are a committed source file, regenerated out of band with
// `cargo xtask bindgen`.
include!("../bindings.rs");

// `libc` does not expose vsnprintf and libvlc advises to use it to handle logs in the log
// callbacks. Expose it for convenience.
unsafe extern "C" {
    pub fn vsnprintf(
        s: *mut libc::c_char,
        n: usize,
        fmt: *const libc::c_char,
        ap: VaList,
    ) -> libc::c_int;
}
