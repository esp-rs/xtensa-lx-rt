#![no_std]
#![feature(cfg_version)]
#![cfg_attr(not(version("1.59")), feature(asm))]
#![feature(asm_experimental_arch)]
#![cfg_attr(not(version("1.59")), feature(global_asm))]
#![feature(naked_functions)]
// required due to: https://github.com/rust-lang/rust/pull/87324
#![allow(named_asm_labels)]

use core::arch::asm;

pub use proc_macros::entry;
pub use proc_macros::exception;
pub use proc_macros::interrupt;
pub use proc_macros::pre_init;

use r0;
pub use r0::init_data;
pub use r0::zero_bss;

use xtensa_lx_rt_proc_macros as proc_macros;

pub mod exception;
pub mod interrupt;

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

        static mut _data_start: u32;
        static mut _data_end: u32;
        static _sidata: u32;

        static mut _init_start: u32;

    }

    extern "Rust" {
        // This symbol will be provided by the user via `#[entry]`
        fn main() -> !;

        // This symbol will be provided by the user via `#[pre_init]`
        fn __pre_init();

        fn __zero_bss() -> bool;

        fn __init_data() -> bool;
    }

    __pre_init();

    if __zero_bss() {
        r0::zero_bss(&mut _bss_start, &mut _bss_end);
    }

    if __init_data() {
        r0::init_data(&mut _data_start, &mut _data_end, &_sidata);
    }

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
    #[cfg(any(
        XCHAL_HAVE_TIMER0,
        XCHAL_HAVE_TIMER1,
        XCHAL_HAVE_TIMER2,
        XCHAL_HAVE_TIMER3
    ))]
    {
        let value = 0;
        cfg_asm!(
        {
            #[cfg(XCHAL_HAVE_TIMER0)]
            "wsr.ccompare0 {0}",
            #[cfg(XCHAL_HAVE_TIMER1)]
            "wsr.ccompare1 {0}",
            #[cfg(XCHAL_HAVE_TIMER2)]
            "wsr.ccompare2 {0}",
            #[cfg(XCHAL_HAVE_TIMER3)]
            "wsr.ccompare3 {0}",
            "isync",
        }, in(reg) value, options(nostack));
    }
}

#[doc(hidden)]
#[inline]
unsafe fn set_vecbase(base: *const u32) {
    asm!("wsr.vecbase {0}", in(reg) base, options(nostack));
}

#[doc(hidden)]
#[no_mangle]
#[rustfmt::skip]
pub extern "Rust" fn default_mem_hook() -> bool {
    true // default to zeroing bss & initializing data
}

#[macro_export]
macro_rules! cfg_asm {
    (@inner, [$($x:tt)*], [$($opts:tt)*], ) => {
        asm!($($x)* $($opts)*)
    };
    (@inner, [$($x:tt)*], [$($opts:tt)*], #[cfg($meta:meta)] $asm:literal, $($rest:tt)*) => {
        #[cfg($meta)]
        cfg_asm!(@inner, [$($x)* $asm,], [$($opts)*], $($rest)*);
        #[cfg(not($meta))]
        cfg_asm!(@inner, [$($x)*], [$($opts)*], $($rest)*)
    };
    (@inner, [$($x:tt)*], [$($opts:tt)*], $asm:literal, $($rest:tt)*) => {
        cfg_asm!(@inner, [$($x)* $asm,], [$($opts)*], $($rest)*)
    };
    ({$($asms:tt)*}, $($opts:tt)*) => {
        cfg_asm!(@inner, [], [$($opts)*], $($asms)*)
    };
}
