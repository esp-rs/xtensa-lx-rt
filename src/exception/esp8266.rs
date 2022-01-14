use core::arch::asm;

use super::ExceptionCause;

/// State of the CPU saved when entering exception or interrupt
///
/// Must be aligned with assembly frame format in assembly_lx106
#[repr(C)]
#[allow(non_snake_case)]
#[derive(Debug, Default)]
pub struct Context {
    PC: u32,
    PS: u32,

    A0: u32,
    A1: u32,
    A2: u32,
    A3: u32,
    A4: u32,
    A5: u32,
    A6: u32,
    A7: u32,
    A8: u32,
    A9: u32,
    A10: u32,
    A11: u32,
    A12: u32,
    A13: u32,
    A14: u32,
    A15: u32,
    SAR: u32,
    EXCCAUSE: u32,
    EXCVADDR: u32,
}

extern "Rust" {
    /// This symbol will be provided by the user via `#[exception]`
    fn __exception(cause: ExceptionCause);
    /// No attribute is supplied for this symbol as the double exception can hardly occur
    fn __double_exception(cause: ExceptionCause);

    /// This symbol will be provided by the user via `#[interrupt]`
    fn __level_1_interrupt(level: u32);
}

#[no_mangle]
#[link_section = ".rwtext"]
extern "C" fn __default_exception(cause: ExceptionCause, save_frame: &Context) {
    panic!("Exception: {:?}, {:08x?}", cause, save_frame)
}

#[no_mangle]
#[link_section = ".rwtext"]
extern "C" fn __default_double_exception(cause: ExceptionCause, save_frame: &Context) {
    panic!("Double Exception: {:?}, {:08x?}", cause, save_frame)
}
#[no_mangle]
#[link_section = ".rwtext"]
extern "C" fn __default_interrupt(_level: u32, _save_frame: &Context) {}

#[naked]
#[no_mangle]
#[link_section = ".DebugException.text"]
unsafe extern "C" fn _DebugExceptionVector() {
    asm!(
        "
        wsr a0, EXCSAVE1 // preserve a0
        call0 __naked_debug_exception     // used as long jump
        ",
        options(noreturn)
    );
}

#[naked]
#[no_mangle]
#[link_section = ".NMIException.text"]
unsafe extern "C" fn _NMIExceptionVector() {
    asm!(
        "
        wsr a0, EXCSAVE1 // preserve a0
        call0 __naked_nmi_exception     // used as long jump
        ",
        options(noreturn)
    );
}

#[naked]
#[no_mangle]
#[link_section = ".KernelException.text"]
unsafe extern "C" fn _KernelExceptionVector() {
    asm!(
        "
        wsr a0, EXCSAVE1 // preserve a0

        call0  __naked_kernel_exception
        ",
        options(noreturn)
    );
}

#[naked]
#[no_mangle]
#[link_section = ".UserException.text"]
unsafe extern "C" fn _UserExceptionVector() {
    asm!(
        "
        wsr a0, EXCSAVE1 // preserve a0

        call0 __naked_user_exception
        ",
        options(noreturn)
    );
}

#[naked]
#[no_mangle]
#[link_section = ".DoubleException.text"]
unsafe extern "C" fn _DoubleExceptionVector() {
    asm!(
        "
        wsr a0, EXCSAVE1                   // preserve a0 (EXCSAVE1 can be reused as long as there
                                           // is no double exception in the first exception until
                                           // EXCSAVE1 is stored to the stack.)
        call0 __naked_double_exception     // used as long jump
    ",
        options(noreturn)
    );
}
