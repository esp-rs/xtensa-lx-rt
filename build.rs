use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use core_isa_parser::{get_config, Chip, Value};
use minijinja::{context, Environment};

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Put the linker script somewhere the linker can find it
    File::create(out.join("link.x"))
        .unwrap()
        .write_all(include_bytes!("xtensa.in.x"))
        .unwrap();

    match (
        cfg!(feature = "esp32") || cfg!(feature = "esp32s2") || cfg!(feature = "esp32s3"),
        cfg!(feature = "esp8266"),
    ) {
        (true, false) => handle_esp32(),
        (false, true) => handle_esp8266(),
        _ => panic!("Either the esp32, esp32s2, esp32s3 or esp8266 feature must be enabled"),
    };

    println!("cargo:rustc-link-search={}", out.display());

    // Only re-run the build script when xtensa.in.x is changed,
    // instead of when any part of the source code changes.
    println!("cargo:rerun-if-changed=xtensa.in.x");
}

fn handle_esp8266() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let exception_source = &include_bytes!("exception-esp8266.x")[..];

    File::create(out.join("exception.x"))
        .unwrap()
        .write_all(exception_source)
        .unwrap();
}

fn handle_esp32() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let rustflags = env::var_os("CARGO_ENCODED_RUSTFLAGS")
        .unwrap()
        .into_string()
        .unwrap();

    let mut features_to_disable: HashSet<String> = HashSet::new();

    // Users can pass -Ctarget-feature to the compiler multiple times, so we have to handle that
    let target_flags = rustflags
        .split(0x1f as char)
        .filter(|s| s.starts_with("target-feature="))
        .map(|s| s.strip_prefix("target-feature="))
        .flatten();
    for tf in target_flags {
        tf.split(",")
            .map(|s| s.trim())
            .filter(|s| s.starts_with('-'))
            .map(|s| s.strip_prefix('-'))
            .flatten()
            .map(rustc_feature_to_xchal_have)
            .flatten()
            .for_each(|s| {
                features_to_disable.insert(s.to_owned());
            })
    }

    let chip = match (
        cfg!(feature = "esp32"),
        cfg!(feature = "esp32s2"),
        cfg!(feature = "esp32s3"),
    ) {
        (true, false, false) => Chip::Esp32,
        (false, true, false) => Chip::Esp32s2,
        (false, false, true) => Chip::Esp32s3,
        _ => panic!("Either the esp32, esp32s2, esp32s3 or esp8266 feature must be enabled"),
    };
    let isa_config = get_config(chip).expect("Unable to parse ISA config");

    inject_cfgs(&isa_config, &features_to_disable);
    inject_cpu_cfgs(&isa_config);
    generate_exception_x(&out, &isa_config);
    generate_interrupt_level_masks(&out, &isa_config);
}

fn generate_interrupt_level_masks(out: &PathBuf, isa_config: &HashMap<String, Value>) {
    let mut env = Environment::new();
    let exception_source_template = &include_str!("interrupt_level_masks.rs.jinja")[..];
    env.add_template("interrupt_level_masks.rs", exception_source_template)
        .unwrap();
    let template = env.get_template("interrupt_level_masks.rs").unwrap();
    let exception_source = template
        .render(context! {
            XCHAL_INTLEVEL1_MASK => isa_config.get("XCHAL_INTLEVEL1_MASK").unwrap().as_integer(),
            XCHAL_INTLEVEL2_MASK => isa_config.get("XCHAL_INTLEVEL2_MASK").unwrap().as_integer(),
            XCHAL_INTLEVEL3_MASK => isa_config.get("XCHAL_INTLEVEL3_MASK").unwrap().as_integer(),
            XCHAL_INTLEVEL4_MASK => isa_config.get("XCHAL_INTLEVEL4_MASK").unwrap().as_integer(),
            XCHAL_INTLEVEL5_MASK => isa_config.get("XCHAL_INTLEVEL5_MASK").unwrap().as_integer(),
            XCHAL_INTLEVEL6_MASK => isa_config.get("XCHAL_INTLEVEL6_MASK").unwrap().as_integer(),
            XCHAL_INTLEVEL7_MASK => isa_config.get("XCHAL_INTLEVEL7_MASK").unwrap().as_integer(),
        })
        .unwrap();
    File::create(out.join("interrupt_level_masks.rs"))
        .unwrap()
        .write_all(exception_source.as_bytes())
        .unwrap();
}

fn generate_exception_x(out: &PathBuf, isa_config: &HashMap<String, Value>) {
    let mut env = Environment::new();
    let exception_source_template = &include_str!("exception-esp32.x.jinja")[..];
    env.add_template("exception.x", exception_source_template)
        .unwrap();
    let template = env.get_template("exception.x").unwrap();
    let exception_source = template.render(
        context! {
            XCHAL_WINDOW_OF4_VECOFS => isa_config.get("XCHAL_WINDOW_OF4_VECOFS").unwrap().as_integer(),
            XCHAL_WINDOW_UF4_VECOFS => isa_config.get("XCHAL_WINDOW_UF4_VECOFS").unwrap().as_integer(),
            XCHAL_WINDOW_OF8_VECOFS => isa_config.get("XCHAL_WINDOW_OF8_VECOFS").unwrap().as_integer(),
            XCHAL_WINDOW_UF8_VECOFS => isa_config.get("XCHAL_WINDOW_UF8_VECOFS").unwrap().as_integer(),
            XCHAL_WINDOW_OF12_VECOFS => isa_config.get("XCHAL_WINDOW_OF12_VECOFS").unwrap().as_integer(),
            XCHAL_WINDOW_UF12_VECOFS => isa_config.get("XCHAL_WINDOW_UF12_VECOFS").unwrap().as_integer(),
            XCHAL_INTLEVEL2_VECOFS => isa_config.get("XCHAL_INTLEVEL2_VECOFS").unwrap().as_integer(),
            XCHAL_INTLEVEL3_VECOFS => isa_config.get("XCHAL_INTLEVEL3_VECOFS").unwrap().as_integer(),
            XCHAL_INTLEVEL4_VECOFS => isa_config.get("XCHAL_INTLEVEL4_VECOFS").unwrap().as_integer(),
            XCHAL_INTLEVEL5_VECOFS => isa_config.get("XCHAL_INTLEVEL5_VECOFS").unwrap().as_integer(),
            XCHAL_INTLEVEL6_VECOFS => isa_config.get("XCHAL_INTLEVEL6_VECOFS").unwrap().as_integer(),
            XCHAL_NMI_VECOFS => isa_config.get("XCHAL_NMI_VECOFS").unwrap().as_integer(),
            XCHAL_KERNEL_VECOFS => isa_config.get("XCHAL_KERNEL_VECOFS").unwrap().as_integer(),
            XCHAL_USER_VECOFS => isa_config.get("XCHAL_USER_VECOFS").unwrap().as_integer(),
            XCHAL_DOUBLEEXC_VECOFS => isa_config.get("XCHAL_DOUBLEEXC_VECOFS").unwrap().as_integer(),
        }
    ).unwrap();
    File::create(out.join("exception.x"))
        .unwrap()
        .write_all(exception_source.as_bytes())
        .unwrap();
}

fn inject_cfgs(isa_config: &HashMap<String, Value>, disabled_features: &HashSet<String>) {
    for (key, value) in isa_config {
        if key.starts_with("XCHAL_HAVE") && *value.as_integer().unwrap_or(&0) != 0 {
            if !disabled_features.contains(key) {
                println!("cargo:rustc-cfg={}", key);
            }
        }
    }
}

fn inject_cpu_cfgs(isa_config: &HashMap<String, Value>) {
    for (key, value) in isa_config {
        if key.starts_with("XCHAL_TIMER")
            || key.starts_with("XCHAL_PROFILING")
            || key.starts_with("XCHAL_NMI")
        {
            if let Some(_) = value.as_integer() {
                let mut s = String::from(key.trim_end_matches("_INTERRUPT"));
                let first = s.chars().position(|c| c == '_').unwrap() + 1;
                s.insert_str(first, "HAVE_");
                println!("cargo:rustc-cfg={}", s);
            }
        }
    }
    if let Some(value) = isa_config
        .get("XCHAL_INTTYPE_MASK_SOFTWARE")
        .map(|v| v.as_integer())
        .flatten()
    {
        for i in 0..value.count_ones() {
            println!("cargo:rustc-cfg=XCHAL_HAVE_SOFTWARE{}", i);
        }
    }
}

fn rustc_feature_to_xchal_have(s: &str) -> Option<&str> {
    // List of rustc features taken from here:
    // https://github.com/esp-rs/rust/blob/84ecb3f010525cb1b2e7d4da306099c2eaa3e6cd/compiler/rustc_codegen_ssa/src/target_features.rs#L278
    // unlikely to change
    Some(match s {
        "fp" => "XCHAL_HAVE_FP",
        "windowed" => "XCHAL_HAVE_WINDOWED",
        "bool" => "XCHAL_HAVE_BOOLEANS",
        "loop" => "XCHAL_HAVE_LOOPS",
        "sext" => "XCHAL_HAVE_SEXT",
        "nsa" => "XCHAL_HAVE_NSA",
        "mul32" => "XCHAL_HAVE_MUL32",
        "mul32high" => "XCHAL_HAVE_MUL32_HIGH",
        "div32" => "XCHAL_HAVE_DIV32",
        "mac16" => "XCHAL_HAVE_MAC16",
        "dfpaccel" => "XCHAL_HAVE_DFP",
        "s32c1i" => "XCHAL_HAVE_S32C1I",
        "threadptr" => "XCHAL_HAVE_THREADPTR",
        "extendedl32r" => "XCHAL_HAVE_ABSOLUTE_LITERALS",
        "debug" => "XCHAL_HAVE_DEBUG",
        "exception" => "XCHAL_HAVE_EXCEPTIONS",
        "highpriinterrupts" => "XCHAL_HAVE_HIGHPRI_INTERRUPTS",
        "coprocessor" => "XCHAL_HAVE_CP",
        "interrupt" => "XCHAL_HAVE_INTERRUPTS",
        "rvector" => "XCHAL_HAVE_VECTOR_SELECT",
        "prid" => "XCHAL_HAVE_PRID",
        "regprotect" => "XCHAL_HAVE_MIMIC_CACHEATTR",
        "miscsr" => return None,   // XCHAL_NUM_MISC_REGS
        "timerint" => return None, // XCHAL_NUM_TIMERS
        "atomctl" => return None,
        "memctl" => return None,
        _ => return None,
    })
}
