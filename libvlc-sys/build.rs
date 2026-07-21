//! Build script for `libvlc-sys`, locates and links libvlc.

use std::env;

/// Minimum supported libvlc version.
const MIN_LIBVLC_VERSION: &str = "3.0.0";

/// Locate libvlc via pkg-config.
fn probe_libvlc() -> Result<pkg_config::Library, pkg_config::Error> {
    pkg_config::Config::new()
        .atleast_version(MIN_LIBVLC_VERSION)
        .probe("libvlc")
}

fn main() {
    // vsnprintf is inlined by the UCRT headers, so MSVC needs this to resolve
    // the symbol src/lib.rs declares. https://stackoverflow.com/a/34230122
    if env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default() == "msvc" {
        println!("cargo:rustc-link-lib=dylib=legacy_stdio_definitions");
    }

    // The bindings import from libvlc.dll directly via `raw-dylib`, so
    // Windows needs neither an import library nor a search path,
    // only the DLL at runtime.
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        return;
    }

    if let Err(err) = probe_libvlc() {
        panic!("libvlc (>= {}) not found: {:?}", MIN_LIBVLC_VERSION, err);
    }
}
