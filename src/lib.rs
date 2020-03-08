#![no_std]
#![feature(asm)]

use r0;

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    extern "C" {

        // These symbols come from `link.x`
        static mut _bss_start: u32;
        static mut _bss_end: u32;

        static mut _data_start: u32;
        static mut _data_end: u32;
        static _sidata: u32;

    }

    extern "Rust" {
        // This symbol will be provided by the user via `#[entry]`
        fn main() -> !;

    // This symbol will be provided by the user via `#[pre_init]`
    // fn __pre_init();
    }

    // __pre_init();

    // Initialize RAM
    r0::zero_bss(&mut _bss_start, &mut _bss_end);
    r0::init_data(&mut _data_start, &mut _data_end, &_sidata);

    main()
}

/// Get the core cycle count
pub fn get_cycle_count() -> u32 {
    let x: u32;
    unsafe { asm!("rsr.ccount a2" : "={a2}"(x) ) };
    x
}

/// Get the id of the current core
pub fn get_core_id() -> u32 {
    let x: u32;
    unsafe { asm!("rsr.prid a2" : "={a2}"(x) ) };
    // 0xCDCD for the PRO core (core 0)
    // 0xABAB for the APP core (core 0)
    // esp-idf uses bit 13 to distinguish
    x & (1 << 13)
}
