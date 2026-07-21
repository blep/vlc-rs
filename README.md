# vlc-rs [![Build Status](https://travis-ci.org/garkimasera/vlc-rs.svg?branch=master)](https://travis-ci.org/garkimasera/vlc-rs)

Rust bindings for libVLC media framework.

## Status

Many missing functions and wrappers.

## Use

Please add the following dependencies to your Cargo.toml.

```Toml
[dependencies]
vlc-rs = "0.3"
```

Or:

```Toml
[dependencies.vlc-rs]
git = "https://github.com/garkimasera/vlc-rs.git"
```

## Example

Play for 10 seconds from a media file.

```Rust
extern crate vlc;
use vlc::{Instance, Media, MediaPlayer};
use std::thread;

fn main() {
    // Create an instance
    let instance = Instance::new().unwrap();
    // Create a media from a file
    let md = Media::new_path(&instance, "path_to_a_media_file.ogg").unwrap();
    // Create a media player
    let mdp = MediaPlayer::new(&instance).unwrap();
    mdp.set_media(&md);

    // Start playing
    mdp.play().unwrap();

    // Wait for 10 seconds
    thread::sleep(::std::time::Duration::from_secs(10));
}
```

Other examples are in the examples directory.

## Building

### Windows

vlc-rs uses libvlc's SDK and it is a required dependency that must be available at runtime.
For that, you must either build VLC from source or grab one of the pre-built packages from [videolan.org](https://www.videolan.org/vlc/download-windows.html).

Once you've downloaded your chosen package, you should extract it some place such that its path contains no spaces, and add that directory to your `PATH` so `libvlc.dll` is found at startup.
For distribution of an executable program, you should probably copy over the necessary DLLs, as well as the `plugins` directory.

## License

MIT (Examples are licensed under CC0)
