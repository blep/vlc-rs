//! Maintenance tasks for vlc-rs, invoked via `cargo xtask <task>`.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// Minimum libvlc version the bindings target. Keep in sync with
/// `libvlc-sys/build.rs`.
const MIN_LIBVLC_VERSION: &str = "3.0.0";

fn main() -> ExitCode {
    match std::env::args().nth(1).as_deref() {
        Some("bindgen") => {
            generate_bindings();
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("usage: cargo xtask <task>\n");
            eprintln!("tasks:");
            eprintln!("    bindgen    regenerate libvlc-sys/bindings.rs from the system libvlc headers");
            ExitCode::FAILURE
        }
    }
}

fn generate_bindings() {
    let sys_dir = libvlc_sys_dir();
    let header = sys_dir.join("wrapper.h");
    let output = sys_dir.join("bindings.rs");

    // Enforce the same minimum version the build script requires and pick up any
    // header search paths.
    let library = pkg_config::Config::new()
        .atleast_version(MIN_LIBVLC_VERSION)
        .cargo_metadata(false)
        .probe("libvlc")
        .unwrap_or_else(|e| panic!("libvlc >= {MIN_LIBVLC_VERSION} not found via pkg-config: {e}"));

    let mut bindings = bindgen::Builder::default()
        .header(header.to_str().expect("non-UTF-8 header path"))
        // For no_std
        .use_core()
        // Use libc
        .ctypes_prefix("libc")
        // Allowlist every (lib)vlc symbol.
        .allowlist_item("(lib|LIB)?(vlc|VLC)_.*")
        // libvlc only uses FILE behind a pointer. Map it
        // to libc's opaque, per-platform FILE rather than emitting glibc's
        // plain _IO_FILE, whose baked-in layout breaks non-Linux builds.
        .blocklist_type("_IO_.*")
        .blocklist_type("FILE")
        .raw_line("pub use libc::FILE;")
        // Block the whole va_list family and rewrite the parameters bindings to our own `VaList`,
        // which is ABI-correct on every target.
        .blocklist_type(".*va_list.*")
        .raw_line("pub use crate::valist::VaList;")
        // Emit a single extern block rather than one per function.
        .merge_extern_blocks(true);

    for path in &library.include_paths {
        bindings = bindings.clang_arg(format!("-I{}", path.display()));
    }

    let generated = bindings
        .generate()
        .expect("unable to generate bindings")
        .to_string();

    let generated = generated
        .replace("*mut __va_list_tag", "VaList")
        .replace(": va_list", ": VaList");
    assert!(
        !generated.contains("va_list"),
        "a va_list spelling we do not know about survived"
    );

    std::fs::write(&output, generated).expect("couldn't write bindings");
    println!("wrote {}", output.display());
}

/// Absolute path to the `libvlc-sys` crate directory, derived from this xtask's
/// location so it works regardless of the current working directory.
fn libvlc_sys_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask has no parent directory")
        .join("libvlc-sys")
}
