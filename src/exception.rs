//! Xtensa LX6 Exception handling (Specialized for esp32 LX6 configuration)
//!
//! First level interrupts and exceptions save full processor state to the user stack.
//! This includes the coprocessor registers contrary to the esp-idf where these are lazily saved.
//! (Kernel mode option is currently not used.)
//!
//! WindowUnder/Overflow and AllocA use default Xtensa implementation.
//!
//! LoadStoreError and Unaligned are not (yet) implemented: so all accesses to IRAM must
//! be word sized and aligned.
//!
//! Syscall 0 is not (yet) implemented: it doesn't seem to be used in rust.
//!
//! Double Exceptions can only occur during the early setup of the exception handler. Afterwards
//! PS.EXCM is set to 0 to be able to handle WindowUnderflow/Overflow and recursive exceptions will
//! happen instead.
//!
//! In various places call0 are used as long jump: j.l syntax is not supported and call0
//! can always be expanded to mov a0,label; call a0. Care must be taken since A0 is overwritten.
//!

mod assembly;

/// EXCCAUSE register values:
///
/// General Exception Causes
/// (values of EXCCAUSE special register set by general exceptions,
///  which vector to the user, kernel, or double-exception vectors).
#[allow(unused)]
#[derive(Debug)]
pub enum ExceptionCause {
    Illegal = 0,             // Illegal Instruction
    Syscall = 1,             // System Call (Syscall Instruction)
    InstrError = 2,          // Instruction Fetch Error
    LoadStoreError = 3,      // Load Store Error
    LevelOneInterrupt = 4,   // Level 1 Interrupt
    Alloca = 5,              // Stack Extension Assist (Movsp Instruction) For Alloca
    DivideByZero = 6,        // Integer Divide By Zero
    Speculation = 7,         // Use Of Failed Speculative Access (Not Implemented)
    Privileged = 8,          // Privileged Instruction
    Unaligned = 9,           // Unaligned Load Or Store
    Reserved10 = 10,         // Reserved
    Reserved11 = 11,         // Reserved
    InstrDataError = 12,     // Pif Data Error On Instruction Fetch (Rb-200x And Later)
    LoadStoreDataError = 13, // Pif Data Error On Load Or Store (Rb-200x And Later)
    InstrAddrError = 14,     // Pif Address Error On Instruction Fetch (Rb-200x And Later)
    LoadStoreAddrError = 15, // Pif Address Error On Load Or Store (Rb-200x And Later)
    ItlbMiss = 16,           // Itlb Miss (No Itlb Entry Matches, Hw Refill Also Missed)
    ItlbMultiHit = 17,       // Itlb Multihit (Multiple Itlb Entries Match)
    InstrRing = 18,          // Ring Privilege Violation On Instruction Fetch
    Reserved19 = 19,         // Size Restriction On Ifetch (Not Implemented)
    InstrProhibited = 20,    // Cache Attribute Does Not Allow Instruction Fetch
    Reserved21 = 21,         // Reserved
    Reserved22 = 22,         // Reserved
    Reserved23 = 23,         // Reserved
    DtlbMiss = 24,           // Dtlb Miss (No Dtlb Entry Matches, Hw Refill Also Missed)
    DtlbMultiHit = 25,       // Dtlb Multihit (Multiple Dtlb Entries Match)
    LoadStoreRing = 26,      // Ring Privilege Violation On Load Or Store
    Reserved27 = 27,         // Size Restriction On Load/Store (Not Implemented)
    LoadProhibited = 28,     // Cache Attribute Does Not Allow Load
    StoreProhibited = 29,    // Cache Attribute Does Not Allow Store
    Reserved30 = 30,         // Reserved
    Reserved31 = 31,         // Reserved
    Cp0Disabled = 32,        // Access To Coprocessor 0 When Disabled
    Cp1Disabled = 33,        // Access To Coprocessor 1 When Disabled
    Cp2Disabled = 34,        // Access To Coprocessor 2 When Disabled
    Cp3Disabled = 35,        // Access To Coprocessor 3 When Disabled
    Cp4Disabled = 36,        // Access To Coprocessor 4 When Disabled
    Cp5Disabled = 37,        // Access To Coprocessor 5 When Disabled
    Cp6Disabled = 38,        // Access To Coprocessor 6 When Disabled
    Cp7Disabled = 39,        // Access to Coprocessor 7 when disabled

    None = 255,
}

extern "Rust" {
    // This symbol will be provided by the user via `#[exception]`
    fn __exception(cause: ExceptionCause);
    // This symbol will be provided by the user via `#[exception(double)]`
    fn __double_exception(cause: ExceptionCause);

    // This symbol will be provided by the user via `#[interrupt(1)]`
    fn __level_1_interrupt(level: u32);
    // This symbol will be provided by the user via `#[interrupt(2)]`
    fn __level_2_interrupt(level: u32);
    // This symbol will be provided by the user via `#[interrupt(3)]`
    fn __level_3_interrupt(level: u32);
    // This symbol will be provided by the user via `#[interrupt(4)]`
    fn __level_4_interrupt(level: u32);
    // This symbol will be provided by the user via `#[interrupt(5)]`
    fn __level_5_interrupt(level: u32);
    // This symbol will be provided by the user via `#[interrupt(6)]`
    fn __level_6_interrupt(level: u32);
    // This symbol will be provided by the user via `#[interrupt(7)]`
    fn __level_7_interrupt(level: u32);
}

#[no_mangle]
#[link_section = ".rwtext"]
pub extern "C" fn __default_exception(cause: ExceptionCause) {
    panic!("Exception: {:?}", cause)
}

#[no_mangle]
#[link_section = ".rwtext"]
pub extern "C" fn __default_interrupt(level: u32) {
    panic!("Interrupt: {:?}", level)
}

#[no_mangle]
#[link_section = ".rwtext"]
pub extern "C" fn __default_double_exception(cause: ExceptionCause) {
    panic!("Double Exception: {:?}", cause)
}

// Raw vector handlers
//
// The interrupt handlers all use special return instructions.
// rust still generates a ret.w instruction, which will never be reached.
// generation of the ret.w can be prevented by using core::intrinsics::unreachable,
// but then a break 15,1 will be generated (which takes 3 bytes instead of 2) or a 'loop {}',
// but then a jump to own address will be generated which is also 3 bytes.
// No way found yet to prevent this generation altogether.

#[naked]
#[no_mangle]
#[link_section = ".KernelExceptionVector.text"]
pub unsafe extern "C" fn _KernelExceptionVector() {
    asm!(
        "
        wsr a0, EXCSAVE1 // preserve a0
        rsr a0, EXCCAUSE // get exception cause

        beqi a0, 5, .AllocAException
 
        call0 __naked_kernel_exception
        "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".UserExceptionVector.text"]
pub unsafe extern "C" fn _UserExceptionVector() {
    asm!(
        "
        wsr a0, EXCSAVE1 // preserve a0
        rsr a0, EXCCAUSE // get exception cause

        beqi a0, 5, .AllocAException
 
        call0 __naked_user_exception

        .AllocAException:
        call0  _AllocAException
        "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".DoubleExceptionVector.text"]
pub unsafe extern "C" fn _DoubleExceptionVector() {
    asm!(
        "
    wsr a0, EXCSAVE1                   // preserve a0 (EXCSAVE1 can be reused as long as there
                                       // is no double exception in the first exception until 
                                       // EXCSAVE1 is stored to the stack.)
    call0 __naked_double_exception     // used as long jump
    "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".Level2InterruptVector.text"]
pub unsafe extern "C" fn _Level2InterruptVector() {
    asm!(
        "
    wsr a0, EXCSAVE2 // preserve a0
    call0 __naked_level_2_interrupt     // used as long jump
    "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".Level3InterruptVector.text"]
pub unsafe extern "C" fn _Level3InterruptVector() {
    asm!(
        "
    wsr a0, EXCSAVE3 // preserve a0
    call0 __naked_level_3_interrupt     // used as long jump
    "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".Level4InterruptVector.text"]
pub unsafe extern "C" fn _Level4InterruptVector() {
    asm!(
        "
    wsr a0, EXCSAVE4 // preserve a0
    call0 __naked_level_4_interrupt     // used as long jump
    "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".Level5InterruptVector.text"]
pub unsafe extern "C" fn _Level5InterruptVector() {
    asm!(
        "
    wsr a0, EXCSAVE5 // preserve a0
    call0 __naked_level_5_interrupt     // used as long jump
    "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".DebugExceptionVector.text"]
pub unsafe extern "C" fn _Level6InterruptVector() {
    asm!(
        "
    wsr a0, EXCSAVE6 // preserve a0
    call0 __naked_level_6_interrupt     // used as long jump
    "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".NMIExceptionVector.text"]
pub unsafe extern "C" fn _Level7InterruptVector() {
    asm!(
        "
    wsr a0, EXCSAVE7 // preserve a0
    call0 __naked_level_7_interrupt     // used as long jump
    "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".WindowOverflow4.text"]
pub unsafe extern "C" fn _WindowOverflow4() {
    asm!(
        "
        s32e    a0, a5, -16
        s32e    a1, a5, -12
        s32e    a2, a5,  -8
        s32e    a3, a5,  -4
        rfwo
    "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".WindowUnderflow4.text"]
pub unsafe extern "C" fn _WindowUnderflow4() {
    asm!(
        "
        l32e    a1, a5, -12
        l32e    a0, a5, -16
        l32e    a2, a5,  -8
        l32e    a3, a5,  -4
        rfwu

        // inline the _AllocAException saves on the ret.w for WindowUnderflow4
        // this makes that it just fits, which is needed for the bbci instructions

        .align 4
        _AllocAException:
        rsr     a0, WINDOWBASE  // grab WINDOWBASE before rotw changes it 
        rotw    -1              // WINDOWBASE goes to a4, new a0-a3 are scratch 
        rsr     a2, PS
        extui   a3, a2, 8, 4    // XCHAL_PS_OWB_SHIFT, XCHAL_PS_OWB_BITS
        xor     a3, a3, a4      // bits changed from old to current windowbase 
        rsr     a4, EXCSAVE1    // restore original a0 (now in a4) 
        slli    a3, a3, 8       // XCHAL_PS_OWB_SHIFT
        xor     a2, a2, a3      // flip changed bits in old window base 
        wsr     a2, PS          // update PS.OWB to new window base 
        rsync

        bbci    a4, 31, _WindowUnderflow4
        rotw    -1              // original a0 goes to a8 
        bbci    a8, 30, _WindowUnderflow8
        rotw    -1
        j               _WindowUnderflow12

        "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".WindowOverflow8.text"]
pub unsafe extern "C" fn _WindowOverflow8() {
    asm!(
        "
        s32e    a0, a9, -16
        l32e    a0, a1, -12
                        
        s32e    a1, a9, -12
        s32e    a2, a9,  -8
        s32e    a3, a9,  -4
        s32e    a4, a0, -32
        s32e    a5, a0, -28
        s32e    a6, a0, -24
        s32e    a7, a0, -20
        rfwo
    "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".WindowUnderflow8.text"]
pub unsafe extern "C" fn _WindowUnderflow8() {
    asm!(
        "
        l32e    a0, a9, -16
        l32e    a1, a9, -12
        l32e    a2, a9,  -8
        l32e    a7, a1, -12
                        
        l32e    a3, a9,  -4
        l32e    a4, a7, -32
        l32e    a5, a7, -28
        l32e    a6, a7, -24
        l32e    a7, a7, -20
        rfwu
    "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".WindowOverflow12.text"]
pub unsafe extern "C" fn _WindowOverflow12() {
    asm!(
        "
        s32e    a0,  a13, -16
        l32e    a0,  a1,  -12
                            
        s32e    a1,  a13, -12
        s32e    a2,  a13,  -8
        s32e    a3,  a13,  -4
        s32e    a4,  a0,  -48
        s32e    a5,  a0,  -44
        s32e    a6,  a0,  -40
        s32e    a7,  a0,  -36
        s32e    a8,  a0,  -32
        s32e    a9,  a0,  -28
        s32e    a10, a0,  -24
        s32e    a11, a0,  -20
        rfwo
    "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".WindowUnderflow12.text"]
pub unsafe extern "C" fn _WindowUnderflow12() {
    asm!(
        "
        l32e    a0,  a13, -16
        l32e    a1,  a13, -12
        l32e    a2,  a13,  -8
        l32e    a11, a1,  -12
                            
        l32e    a3,  a13,  -4
        l32e    a4,  a11, -48
        l32e    a5,  a11, -44
        l32e    a6,  a11, -40
        l32e    a7,  a11, -36
        l32e    a8,  a11, -32
        l32e    a9,  a11, -28
        l32e    a10, a11, -24
        l32e    a11, a11, -20
        rfwu
    "
    );
}
