#![no_std]
#![feature(asm)]
#![feature(global_asm)]
#![feature(naked_functions)]

pub use proc_macros::entry;
pub use proc_macros::exception;
pub use proc_macros::interrupt;
pub use proc_macros::pre_init;

use r0;
pub use r0::init_data;
pub use r0::zero_bss;

use xtensa_lx_rt_proc_macros as proc_macros;

pub mod exception;

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn DefaultPreInit() {}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    // These symbols come from `link.x`
    extern "C" {
        static mut _bss_start: u32;
        static mut _bss_end: u32;

        static mut _init_start: u32;
    }

    extern "Rust" {
        // This symbol will be provided by the user via `#[entry]`
        fn main() -> !;

        // This symbol will be provided by the user via `#[pre_init]`
        fn __pre_init();
    }

    __pre_init();

    // Initialize RAM
    r0::zero_bss(&mut _bss_start, &mut _bss_end);

    // Copy of data segment is done by bootloader

    // According to 4.4.6.2 of the xtensa isa, ccount and compare are undefined on reset,
    // set all values to zero to disable
    reset_internal_timers();

    // move vec table
    set_vecbase(&_init_start as *const u32);

    main();
}

/*
    We redefine these functions to avoid pulling in xtensa-lx as a dependency
*/

#[doc(hidden)]
#[inline]
unsafe fn reset_internal_timers() {
    #[cfg(feature = "lx6")]
    asm!("
        wsr.ccompare0 {0}
        wsr.ccompare1 {0}
        wsr.ccompare2 {0}
        isync
    ", out(reg) _, options(nostack));
    #[cfg(feature = "lx106")]
    asm!("
        wsr.ccompare0 {0}
        isync
    ", out(reg) _, options(no_stack));
}

#[doc(hidden)]
#[inline]
unsafe fn set_vecbase(base: *const u32) {
    asm!("wsr.vecbase {0}", in(reg) base, options(nostack));
}
