#![no_std]
#![no_main]

use xtensa_lx6_rt as _;

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

// GPIO output enable reg
const GPIO_ENABLE_W1TS_REG: u32 = 0x3FF44024;

// gpio output set register
const GPIO_OUT_W1TS_REG: u32 = 0x3FF44008;
// gpio output clear register
const GPIO_OUT_W1TC_REG : u32 = 0x3FF4400C;



// const GPIO_ENABLE_REG: u32 = 0x3FF44020;

// const GPIO_OUT_REG: u32 = 0x3FF44004;


const BLINKY_GPIO: u32 = 2; // the GPIO hooked up to the onboard LED
const GPIO_FUNCX_OUT_BASE: u32 = 0x3FF44530;
const GPIO_FUNC2_OUT_SEL_CFG: u32 = GPIO_FUNCX_OUT_BASE + (BLINKY_GPIO * 4);

const IO_MUX_GPIO2_REG: u32 = 0x3FF49040;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {

    }
}

#[no_mangle]
fn main() -> ! {

    // set the pin as an output
    unsafe {
        core::ptr::write_volatile(GPIO_ENABLE_W1TS_REG as *mut _, 0x1 << BLINKY_GPIO);
        core::ptr::write_volatile(GPIO_FUNC2_OUT_SEL_CFG as *mut _, 0x100); // 0x100 makes this pin a simple gpio pin - see the technical reference
        
        // TODO select function and drive strength
        // let iomux_gpio2 = 0;
        // core::ptr::write_volatile(IO_MUX_GPIO2_REG as *mut _, BLINKY_GPIO); // GPIO2 function 1 is being a gpio port
    }
    loop {
        unsafe {
            core::ptr::write_volatile(GPIO_OUT_W1TS_REG as *mut _, 0x1 << BLINKY_GPIO);
            let dummy_var: u32 = 0;
            for _ in 0..32_00 {
                unsafe { core::ptr::read_volatile(&dummy_var) };
            }
            // core::ptr::write_volatile(GPIO_OUT_W1TC_REG as *mut _, 0x1 << BLINKY_GPIO);
            // for _ in 0..32_00 {
            //     unsafe { core::ptr::read_volatile(&dummy_var) };
            // }
        }
    }
}
