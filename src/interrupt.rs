#[cfg(any(feature = "esp32", feature = "esp32s2", feature = "esp32s3"))]
include!(concat!(env!("OUT_DIR"), "/interrupt_level_masks.rs"));
