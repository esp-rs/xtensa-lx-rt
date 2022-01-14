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

    let exception_source = match (cfg!(feature = "esp32"), cfg!(feature = "esp8266")) {
        (true, false) => &include_bytes!("exception-esp32.x")[..],
        (false, true) => &include_bytes!("exception-esp8266.x")[..],
        _ => panic!("Either the esp32 or esp8266 feature must be enabled"),
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
