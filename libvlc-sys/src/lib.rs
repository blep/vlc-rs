#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

pub mod valist;

// The bindings are a committed source file, regenerated out of band with
// `cargo xtask bindgen`.
include!("../bindings.rs");
