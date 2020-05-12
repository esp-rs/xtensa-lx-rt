//! Xtensa LX6 Exception handling (Specialized for esp32 LX6 configuration)
//!
//! First level interrupts and exceptions save full processor state to the user stack.
//! This includes the coprocessor registers contrary to the esp-idf where these are lazily saved.
//! (Kernel mode option is currently not used.)
//!
//! WindowUnder/Overflow and AllocA use default Xtensa implementation.
//!
//! LoadSToreError and Unaligned are not (yet) implemented: so all accesses to IRAM must
//! be word sized and aligned.
//!
//! Syscall 0 is not (yet) implemented: it doesn't seem to be used in rust.

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

// impl From<u32> for Exccause {
//     fn from(v: u32) -> Exccause {
//         match v {
//             0 => Exccause::Illegal,
//             1 => Exccause::Syscall,
//             2 => Exccause::InstrError,
//             3 => Exccause::LoadStoreError,
//             4 => Exccause::Illegal,
//             5 => Exccause::Illegal,
//             6 => Exccause::Illegal,
//             7 => Exccause::Illegal,
//         }
//     }
// }

global_asm!(
    "
    .set XT_STK_EXIT,            0
    .set XT_STK_PC,              4 
    .set XT_STK_PS,              8 
    .set XT_STK_A0,             12 
    .equ XT_STK_A1,             16 
    .set XT_STK_A2,             20 
    .set XT_STK_A3,             24 
    .set XT_STK_A4,             28 
    .set XT_STK_A5,             32 
    .set XT_STK_A6,             36 
    .set XT_STK_A7,             40 
    .set XT_STK_A8,             44 
    .set XT_STK_A9,             48 
    .set XT_STK_A10,            52 
    .set XT_STK_A11,            56 
    .set XT_STK_A12,            60 
    .set XT_STK_A13,            64 
    .set XT_STK_A14,            68 
    .set XT_STK_A15,            72 
    .set XT_STK_SAR,            76 
    .set XT_STK_EXCCAUSE,       80
    .set XT_STK_EXCVADDR,       84
    .set XT_STK_LBEG,           88
    .set XT_STK_LEND,           92
    .set XT_STK_LCOUNT,         96
    
    .set XT_STK_TMP0,          100
    .set XT_STK_TMP1,          104
    .set XT_STK_TMP2,          108

    .set XT_STK_THREADPTR,     112 // freely usable 32-bit register intended for TLS
    .set XT_STK_SCOMPARE1,     116 // register for s32ci instruction
    .set XT_STK_BR,            120 // part of boolean 

    .set XT_STK_ACCLO,         124 // 
    .set XT_STK_ACCHI,         128
    .set XT_STK_M0,            132
    .set XT_STK_M1,            136
    .set XT_STK_M2,            140
    .set XT_STK_M3,            144

    .set XT_STK_F64R_LO,       148
    .set XT_STK_F64R_HI,       152
    .set XT_STK_F64S,          156

    .set XT_STK_FCR,           160
    .set XT_STK_FSR,           164
    .set XT_STK_F0,            168
    .set XT_STK_F1,            172
    .set XT_STK_F2,            176
    .set XT_STK_F3,            180
    .set XT_STK_F4,            184
    .set XT_STK_F5,            188
    .set XT_STK_F6,            192
    .set XT_STK_F7,            196
    .set XT_STK_F8,            200
    .set XT_STK_F9,            204
    .set XT_STK_F10,           208
    .set XT_STK_F11,           212
    .set XT_STK_F12,           216
    .set XT_STK_F13,           220
    .set XT_STK_F14,           224
    .set XT_STK_F15,           228

    
    .set XT_STK_FRMSZ,         256  // allow for some extra space and nice round number for addmi

    .set PS_INTLEVEL_MASK, 0x0000000f
    .set PS_INTLEVEL_EXCM, 3	
    .set PS_UM, 0x00000020
    .set PS_WOE, 0x00040000

    .set PS_EXC_MODE, PS_INTLEVEL_EXCM | PS_UM | PS_WOE
    
    "
);

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn _lowint1() {
    loop {
        continue;
    }
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn _rust_user_exc() {
    let mut cause: u32;
    asm!("rsr.exccause a2" : "={a2}"(cause) );
    if cause == ExceptionCause::LevelOneInterrupt as u32 {
        _lowint1();
        return;
    }
    panic!("Ruh Roh")
}

#[naked]
#[no_mangle]
#[link_section = ".rwtext"]
unsafe extern "C" fn _Exception() {
    panic!("Exception");
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
#[link_section = ".Level2InterruptVector.text"]
pub unsafe extern "C" fn _Level2InterruptVector() {
    asm!("rfi 2");
}

#[naked]
#[no_mangle]
#[link_section = ".Level3InterruptVector.text"]
pub unsafe extern "C" fn _Level3InterruptVector() {
    asm!("rfi 3");
}

#[naked]
#[no_mangle]
#[link_section = ".Level4InterruptVector.text"]
pub unsafe extern "C" fn _Level4InterruptVector() {
    asm!("rfi 4");
}

#[naked]
#[no_mangle]
#[link_section = ".Level5InterruptVector.text"]
pub unsafe extern "C" fn _Level5InterruptVector() {
    asm!("rfi 5");
}

#[naked]
#[no_mangle]
#[link_section = ".DebugExceptionVector.text"]
pub unsafe extern "C" fn _DebugExceptionVector() {
    asm!("rfi 6");
}

#[naked]
#[no_mangle]
#[link_section = ".NMIExceptionVector.text"]
pub unsafe extern "C" fn _NMIExceptionVector() {
    asm!("rfi 7");
}

#[naked]
#[no_mangle]
#[link_section = ".KernelExceptionVector.text"]
pub unsafe extern "C" fn _KernelExceptionVector() {
    asm!("call0 _Exception");
}

// TODO this doesn't work, we cannot call into Rust functions here: see idf asm

#[naked]
#[no_mangle]
#[link_section = ".rwtext"]
unsafe extern "C" fn _UserExceptionHandler() {
    panic!("_Level2InterruptVector");
}

#[no_mangle]
#[link_section = ".rwtext"]
pub extern "C" fn __default_other_exception(cause: ExceptionCause) {
    panic!("Exception {:?}", cause)
}

extern "Rust" {
    // This symbol will be provided by the user via `#[exception]`
    fn __other_exception(cause: ExceptionCause);
    // This symbol will be provided by the user via `#[exception(double)]`
    fn __double_exception(cause: ExceptionCause);
}

/// Save processor state to stack.
///
/// *Must only be called with call0.*
/// *For spill all window registers to work WOE must be enabled on entry
///
/// Saves all registers except PC, PS, A0, A1
///
/// Inputs:
///     A0 is the return address
///     A1 is the stack pointers
///     Exceptions are disabled (PS.EXCM = 1)
///
/// Output:
///     A0 is the return address
///     A1 is the stack pointer
///     A3, A9 are used as scratch registers
///     EPC1 is changed
#[naked]
#[no_mangle]
#[link_section = ".rwtext"]
unsafe extern "C" fn save_context() {
    asm!(
        "
        s32i    a2,  sp, +XT_STK_A2
        s32i    a3,  sp, +XT_STK_A3
        s32i    a4,  sp, +XT_STK_A4
        s32i    a5,  sp, +XT_STK_A5
        s32i    a6,  sp, +XT_STK_A6
        s32i    a7,  sp, +XT_STK_A7
        s32i    a8,  sp, +XT_STK_A8
        s32i    a9,  sp, +XT_STK_A9
        s32i    a10, sp, +XT_STK_A10
        s32i    a11, sp, +XT_STK_A11
        s32i    a12, sp, +XT_STK_A12
        s32i    a13, sp, +XT_STK_A13
        s32i    a14, sp, +XT_STK_A14
        s32i    a15, sp, +XT_STK_A15
 
        rsr     a3,  SAR
        s32i    a3,  sp, +XT_STK_SAR

        // Loop Option
        rsr     a3,  LBEG
        s32i    a3,  sp, +XT_STK_LBEG
        rsr     a3,  LEND
        s32i    a3,  sp, +XT_STK_LEND
        rsr     a3,  LCOUNT
        s32i    a3,  sp, +XT_STK_LCOUNT

        // Thread Pointer Option
        rur     a3, threadptr
        s32i    a3, sp, +XT_STK_THREADPTR

        // Conditional Store Option
        rsr     a3, scompare1
        s32i    a3, sp, +XT_STK_SCOMPARE1
        
        // Boolean Option
        rsr     a3, br
        s32i    a3, sp, +XT_STK_BR

        // MAC16 Option
        rsr     a3, acclo
        s32i    a3, sp, +XT_STK_ACCLO
        rsr     a3, acchi
        s32i    a3, sp, +XT_STK_ACCHI
        rsr     a3, m0
        s32i    a3, sp, +XT_STK_M0
        rsr     a3, m1
        s32i    a3, sp, +XT_STK_M1
        rsr     a3, m2
        s32i    a3, sp, +XT_STK_M2
        rsr     a3, m3
        s32i    a3, sp, +XT_STK_M3

        // Double Precision Accelerator Option
        .byte 0xa0, 0x3e, 0xe3              // rur     a3, f64r_lo TODO: 20200510 not yet supported by llvm
        s32i    a3, sp, +XT_STK_F64R_LO
        .byte 0xb0, 0x3e, 0xe3              // rur     a3, f64r_hi TODO: 20200510 not yet supported by llvm
        s32i    a3, sp, +XT_STK_F64R_HI
        .byte 0xc0, 0x3e, 0xe3              // rur     a3, f64s    TODO: 20200510 not yet supported by llvm
        s32i    a3, sp, +XT_STK_F64S

        // Coprocessor Option
        .byte 0x80, 0x3e, 0xe3              // rur     a3, fcr     TODO: 20200510 not yet supported by llvm
        s32i    a3, sp, +XT_STK_FCR
        .byte 0x90, 0x3e, 0xe3              // rur     a3, fsr     TODO: 20200510 not yet supported by llvm
        s32i    a3, sp, +XT_STK_FSR
        ssi     f0, sp, +XT_STK_F0
        ssi     f1, sp, +XT_STK_F1
        ssi     f2, sp, +XT_STK_F2
        ssi     f3, sp, +XT_STK_F3
        ssi     f4, sp, +XT_STK_F4
        ssi     f5, sp, +XT_STK_F5
        ssi     f6, sp, +XT_STK_F6
        ssi     f7, sp, +XT_STK_F7
        ssi     f8, sp, +XT_STK_F8
        ssi     f9, sp, +XT_STK_F9
        ssi     f10, sp, +XT_STK_F10
        ssi     f11, sp, +XT_STK_F11
        ssi     f12, sp, +XT_STK_F12
        ssi     f13, sp, +XT_STK_F13
        ssi     f14, sp, +XT_STK_F14
        ssi     f15, sp, +XT_STK_F15


        // Spill all windows (up to 64) to the stack
        // Uses the overflow exception: doing a noop write to the high registers 
        // will trigger if needed. WOE needs to be enabled before this routine.
        
        mov     a9, a0                      // store return address
        addmi   sp,  sp, +XT_STK_FRMSZ       // go back to spill register region

        and a12, a12, a12
        rotw 3
        and a12, a12, a12
        rotw 3
        and a12, a12, a12
        rotw 3
        and a12, a12, a12
        rotw 3
        and a12, a12, a12
        rotw 4

        addmi   sp,  sp, -XT_STK_FRMSZ     // return the current stack pointer
        mov     a0, a9                     // retrieve return address

        ret
    "
    )
}

#[naked]
#[no_mangle]
#[link_section = ".rwtext"]
unsafe extern "C" fn restore_context() {
    asm!(
        "
 
        l32i    a3,  sp, +XT_STK_SAR
        wsr     a3,  SAR

        // Loop Option
        l32i    a3,  sp, +XT_STK_LBEG
        wsr     a3,  LBEG
        l32i    a3,  sp, +XT_STK_LEND
        wsr     a3,  LEND
        l32i    a3,  sp, +XT_STK_LCOUNT
        wsr     a3,  LCOUNT

        // Thread Pointer Option
        l32i    a3, sp, +XT_STK_THREADPTR
        wur     a3, threadptr

        // Conditional Store Option
        l32i    a3, sp, +XT_STK_SCOMPARE1
        wsr     a3, scompare1
        
        // Boolean Option
        l32i    a3, sp, +XT_STK_BR
        wsr     a3, br

        // MAC16 Option
        l32i    a3, sp, +XT_STK_ACCLO
        wsr     a3, acclo
        l32i    a3, sp, +XT_STK_ACCHI
        wsr     a3, acchi
        l32i    a3, sp, +XT_STK_M0
        wsr     a3, m0
        l32i    a3, sp, +XT_STK_M1
        wsr     a3, m1
        l32i    a3, sp, +XT_STK_M2
        wsr     a3, m2
        l32i    a3, sp, +XT_STK_M3
        wsr     a3, m3

        // Double Precision Accelerator Option
        l32i    a3, sp, +XT_STK_F64R_LO
        .byte 0x30, 0xea, 0xf3               // wur     a3, f64r_lo TODO: 20200510 not yet supported by llvm
        l32i    a3, sp, +XT_STK_F64R_HI
        .byte 0x30, 0xeb, 0xf3               // wur     a3, f64r_hi TODO: 20200510 not yet supported by llvm
        l32i    a3, sp, +XT_STK_F64S
        .byte 0x30, 0xec, 0xf3               // wur     a3, f64s TODO: 20200510 not yet supported by llvm

        // Coprocessor Option
        l32i    a3, sp, +XT_STK_FCR
        .byte 0x30, 0xe8, 0xf3               // wur     a3, fcr TODO: 20200510 not yet supported by llvm
        l32i    a3, sp, +XT_STK_FSR
        .byte 0x30, 0xe9, 0xf3               // wur     a3, fsr TODO: 20200510 not yet supported by llvm
        lsi     f0, sp, +XT_STK_F0
        lsi     f1, sp, +XT_STK_F1
        lsi     f2, sp, +XT_STK_F2
        lsi     f3, sp, +XT_STK_F3
        lsi     f4, sp, +XT_STK_F4
        lsi     f5, sp, +XT_STK_F5
        lsi     f6, sp, +XT_STK_F6
        lsi     f7, sp, +XT_STK_F7
        lsi     f8, sp, +XT_STK_F8
        lsi     f9, sp, +XT_STK_F9
        lsi     f10, sp, +XT_STK_F10
        lsi     f11, sp, +XT_STK_F11
        lsi     f12, sp, +XT_STK_F12
        lsi     f13, sp, +XT_STK_F13
        lsi     f14, sp, +XT_STK_F14
        lsi     f15, sp, +XT_STK_F15

        // general registers
        l32i    a2,  sp, +XT_STK_A2
        l32i    a3,  sp, +XT_STK_A3
        l32i    a4,  sp, +XT_STK_A4
        l32i    a5,  sp, +XT_STK_A5
        l32i    a6,  sp, +XT_STK_A6
        l32i    a7,  sp, +XT_STK_A7
        l32i    a8,  sp, +XT_STK_A8
        l32i    a9,  sp, +XT_STK_A9
        l32i    a10, sp, +XT_STK_A10
        l32i    a11, sp, +XT_STK_A11
        l32i    a12, sp, +XT_STK_A12
        l32i    a13, sp, +XT_STK_A13
        l32i    a14, sp, +XT_STK_A14
        l32i    a15, sp, +XT_STK_A15

        ret
    "
    )
}

/// Handle Other Exceptions by storing full context and then calling regular function
///
/// # Input:
///    * A0 stored in EXCSAVE1
#[naked]
#[no_mangle]
#[link_section = ".rwtext"]
unsafe extern "C" fn _OtherException() {
    asm!(
        "
        mov     a0, a1                     // save a1/sp
        addmi   sp, sp, -XT_STK_FRMSZ      // only allow multiple of 256

        s32i    a0, sp, +XT_STK_A1
        s32e    a0, sp, -12                // for debug backtrace 
        rsr     a0, PS                     // save interruptee's PS
        s32i    a0, sp, +XT_STK_PS
        rsr     a0, EPC1                   // save interruptee's PC 
        s32i    a0, sp, +XT_STK_PC
        s32e    a0, sp, -16                // for debug backtrace 

        rsr     a0, EXCCAUSE
        s32i    a0, sp, +XT_STK_EXCCAUSE
        rsr     a0, EXCVADDR
        s32i    a0, sp, +XT_STK_EXCVADDR

        // _save interuptees a0
        rsr     a0, EXCSAVE1               // save interruptee's a0 
        s32i    a0, sp, +XT_STK_A0

        // Set up PS for C, reenable hi-pri interrupts, and clear EXCM. 
        // needs to be done before save_context, which uses window overflow to store all state
        movi    a0, (PS_INTLEVEL_EXCM | PS_UM | PS_WOE)
        wsr     a0, PS
        rsync                              // wait for WSR.PS to complete 

        call0   save_context

        //    Create pseudo base save area. At this point, sp is still pointing to the
        //    allocated and filled exception stack frame.
        
        l32i    a3, sp, +XT_STK_A0        // Copy pre-exception a0 (return address) 
        s32e    a3, sp, -16
        l32i    a3, sp, +XT_STK_A1        // Copy pre-exception a1 (stack pointer) 
        s32e    a3, sp, -12
        l32i    a0, sp, +XT_STK_PC        // return address for debug backtrace 

        rsr     a6, EXCCAUSE              // recover exc cause, a6 will be a2 in the 
                                          // called routine
        
        call4   __other_exception           // call handler <= actual call!

        // TODO: handle properly
        xor     a8,a8,a8                  // clear all interrupts
        addi    a8,a8,-1
        .byte 0x80, 0xe3, 0x13            // wsr     a8, INT_CLEAR 
                                          // TODO: 20200511, not supported


        // Restore context and return 
        call0   restore_context

        l32i    a0, sp, +XT_STK_PS        // retrieve interruptee's PS 
        wsr     a0, PS
        l32i    a0, sp, +XT_STK_PC        // retrieve interruptee's PC 
        wsr     a0, EPC1
        l32i    a0, sp, +XT_STK_A0        // retrieve interruptee's A0 
        l32i    sp, sp, +XT_STK_A1        // remove exception frame 
        rsync                             // ensure PS and EPC written 
    
        .byte 0x00, 0x30, 0x00            // rfe   // PS.EXCM is cleared 
                                          // TODO: 20200509, not supported: 
        "
    )
}

#[naked]
#[no_mangle]
#[link_section = ".rwtext"]
unsafe extern "C" fn _SysCallException() {
    panic!("_Level2InterruptVector");
}

#[naked]
#[no_mangle]
#[link_section = ".rwtext"]
unsafe extern "C" fn _CoprocessorException() {
    panic!("_Level2InterruptVector");
}

#[naked]
#[no_mangle]
#[link_section = ".UserExceptionVector.text"]
pub unsafe extern "C" fn _UserExceptionVector() {
    asm!(
        "
        wsr a0, EXCSAVE1 // preserve a0
        rsr a0, EXCCAUSE // get exception cause

 //       beqi a0, 1, .SysCallException
 //       beqi a0, 3, .AlignmentException
        beqi a0, 5, .AllocAException
   //     bgeui a0, 32, .CoprocessorException
        

        // call0's actually used as jump, return address will be set manually in exception handler

        call0 _OtherException
        

        .AllocAException:
        call0 _AllocAException

        .CoprocessorException:
        call0 _CoprocessorException

        .SysCallException:
        call0 _SysCallException

        .AlignmentException:
//        call0 _AlignmentException
//        addi    a0, a0, 1
//        j       .LS_exit
        "
    );
}

/*
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
*/

#[naked]
#[no_mangle]
#[link_section = ".DoubleExceptionVector.text"]
pub unsafe extern "C" fn _DoubleExceptionVector() {
    asm!(
        "
    rsr.exccause a2
    rsr.depc a3
    rsr.epc1 a4
    1:
    j 1b
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
