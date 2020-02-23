use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    // let window_vectors = format!("bin/{}.a", "window_vectors");
    // fs::copy(
    //         &window_vectors,
    //         out.join("libwindow_vectors.a"),
    //     ).unwrap();
    //     println!("cargo:rustc-link-lib=static=window_vectors");

    // Put the linker script somewhere the linker can find it
    File::create(out.join("link.x"))
        .unwrap()
        .write_all(include_bytes!("xtensa.in.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // Only re-run the build script when memory.x is changed,
    // instead of when any part of the source code changes.
    println!("cargo:rerun-if-changed=xtensa.in.x");
    // println!("cargo:rerun-if-changed={}", window_vectors);
}
