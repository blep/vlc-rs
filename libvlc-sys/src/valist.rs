//! Platform `va_list`, extracted and simplified from `core::ffi::va_list`.
//!
//! libvlc-sys rust bindings are common to all libvlc's supported platforms and are bindgen
//! generated once by the maintainers. Hence it needs a cross-platform va_list type.
//!
//! VaLists are platform specific, the C standard only specifies that they should be possible to
//! pass by value. Libvlc uses them in the logger callback exclusively, until the rust stdlib C
//! variadic API is stable, the type with the single goal of being forwarded to a C formatting
//! function. Which simplifies greatly the implementation.
//!
//! Once the c_variadic API is stabilized, this file should be removed.

/// [AArch64 Procedure Call Standard]:
/// https://github.com/ARM-software/abi-aa/blob/main/aapcs64/aapcs64.rst#id110
#[cfg(all(target_arch = "aarch64", not(target_vendor = "apple")))]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VaList {
    stack: *mut libc::c_void,
    gr_top: *mut libc::c_void,
    vr_top: *mut libc::c_void,
    gr_offs: i32,
    vr_offs: i32,
}

/// Everywhere else: either an opaque pointer already, or a pointer to the struct the array decayed
/// into. This is sound since VaLists are only passed around.
#[cfg(not(all(target_arch = "aarch64", not(target_vendor = "apple"))))]
pub type VaList = *mut libc::c_void;
