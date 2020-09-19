use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Put the linker script somewhere the linker can find it
    File::create(out.join("link.x"))
        .unwrap()
        .write_all(include_bytes!("xtensa.in.x"))
        .unwrap();

    let exception_source = match (cfg!(feature = "lx6"), cfg!(feature = "lx106")) {
        (true, false) => &include_bytes!("exception-lx6.x")[..],
        (false, true) => &include_bytes!("exception-lx106.x")[..],
        _ => panic!("Either the lx6 or lx106 feature has to be enabled")
    };

    File::create(out.join("exception.x"))
        .unwrap()
        .write_all(exception_source)
        .unwrap();

    println!("cargo:rustc-link-search={}", out.display());

    // Only re-run the build script when memory.x is changed,
    // instead of when any part of the source code changes.
    println!("cargo:rerun-if-changed=xtensa.in.x");
}
