#![no_std]
#![feature(asm)]
#![feature(naked_functions)]

use r0;

mod exceptions;

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
        static _init_start: u32;

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

    enable_ints(0); /* disable all interupts */

    asm!("wsr.vecbase $0" ::"r"(&_init_start) :: "volatile"); /* Move the vector table to our handlers */

    main()
}

pub unsafe fn enable_ints(mask: u32) -> u32 {
    asm!("mov a2, $0" :: "r"(mask) :: "volatile"); /* enabled (1 << 6) */
    asm!("movi a3, 0" :::: "volatile");
    asm!("xsr.intenable a3" :::: "volatile"); /* Disable all interrupts */
    asm!("rsync");
    asm!("or a2, a3, a2" :::: "volatile"); /* set bits in mask */
    asm!("wsr.intenable a2" :::: "volatile"); /* Re-enable ints */
    asm!("rsync");

    let prev: u32;
    asm!("mov a2, a3" : "={a2}"(prev) ::: "volatile"); /* return prev mask */

    prev
}
