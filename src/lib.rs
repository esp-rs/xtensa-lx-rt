#![no_std]
#![feature(asm)]
#![feature(global_asm)]
#![feature(naked_functions)]
#![feature(core_intrinsics)]

pub use proc_macros::entry;
pub use proc_macros::exception;
pub use proc_macros::interrupt;
pub use proc_macros::pre_init;

use r0;
pub use r0::init_data;
pub use r0::zero_bss;

use xtensa_lx6_rt_proc_macros as proc_macros;

pub mod exception;
pub mod interrupt;

#[macro_use]
mod macros;

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

    interrupt::disable();

    let vecbase = &_init_start as *const u32;
    set_vecbase(vecbase);

    main();
}

/// Move the vector base
#[inline]
pub unsafe fn set_vecbase(base: *const u32) {
    asm!("wsr.vecbase $0" ::"r"(base) :: "volatile");
}

/// Get the core cycle count
#[inline]
pub fn get_cycle_count() -> u32 {
    let x: u32;
    unsafe { asm!("rsr.ccount $0" : "=r"(x) ::: "volatile" ) };
    x
}

/// Get the core stack pointer
#[inline(always)]
pub fn get_stack_pointer() -> *const u32 {
    let x: *const u32;
    unsafe { asm!("mov $0,sp" : "=r"(x) ::: "volatile") };
    x
}

/// Set the core stack pointer
///
/// *This is highly unsafe!*
/// It should be used with care at e.g. program start or when building a task scheduler
///
/// `stack` pointer to the non-inclusive end of the stack (must be 16-byte aligned)
#[inline(always)]
pub unsafe fn set_stack_pointer(stack: *mut u32) {
    asm!("
        movi a0,0
        mov sp,$0
        " :: "r"(stack):"a0" ::: "volatile" );
}

/// Get the core current program counter
#[inline(always)]
pub fn get_program_counter() -> *const u32 {
    let x: *const u32;
    let _y: u32;
    unsafe {
        asm!("
            mov $1,a0
            call0 1f
            .align 4
            1: 
            mov $0,a0
            mov a0,$1
            " : "=r"(x),"=r"(_y)::"a0" : "volatile" )
    };
    x
}

/// cycle accurate delay using the cycle counter register
#[inline]
pub fn delay(clocks: u32) {
    let start = get_cycle_count();
    loop {
        if get_cycle_count().wrapping_sub(start) >= clocks {
            break;
        }
    }
}

/// Get the id of the current core
#[inline]
pub fn get_processor_id() -> u32 {
    let mut x: u32;
    unsafe { asm!("rsr.prid $0" : "=r"(x) ::: "volatile") };
    x
}

const XDM_OCD_DCR_SET: u32 = 0x10200C;
const DCR_ENABLEOCD: u32 = 0x01;

/// Returns true if a debugger is attached
#[inline]
pub fn is_debugger_attached() -> bool {
    let mut x: u32;
    unsafe { asm!("rer $0,$1" : "=r"(x): "r"(XDM_OCD_DCR_SET) :: "volatile" ) };
    (x & DCR_ENABLEOCD) != 0
}

/// Insert debug breakpoint
#[inline(always)]
pub fn debug_break() {
    unsafe { asm!("break 1,15"::::"volatile") };
}
