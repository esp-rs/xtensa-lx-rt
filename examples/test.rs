#![no_std]
#![no_main]

use xtensa_lx6_rt as _;

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        // add some side effect to prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        atomic::compiler_fence(Ordering::SeqCst)
    }
}

#[no_mangle]
fn main() -> ! {
    loop {
        // add some side effect to prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        atomic::compiler_fence(Ordering::SeqCst)
    }
}
