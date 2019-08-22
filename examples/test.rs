#![no_std]
#![no_main]

use xtensa_lx6_rt as _;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn main() -> ! {
    loop {}
}
