

/// EXCCAUSE register values:
/// 
/// General Exception Causes
/// (values of EXCCAUSE special register set by general exceptions,
///  which vector to the user, kernel, or double-exception vectors).
#[allow(unused)]
pub enum Exccause {
    Illegal = 0,          /* Illegal Instruction */
    Syscall = 1,          /* System Call (Syscall Instruction) */
    InstrError = 2,      /* Instruction Fetch Error */
    LoadStoreError = 3, /* Load Store Error */
    LevelOneInterrupt = 4, /* Level 1 Interrupt */
    Alloca = 5,           /* Stack Extension Assist (Movsp Instruction) For Alloca */
    DivideByZero = 6,   /* Integer Divide By Zero */
    Speculation = 7,      /* Use Of Failed Speculative Access (Not Implemented) */
    Privileged = 8,       /* Privileged Instruction */
    Unaligned = 9,        /* Unaligned Load Or Store */
        /* Reserved	10..11 */
    InstrDataError = 12, /* Pif Data Error On Instruction Fetch (Rb-200x And Later) */
    LoadStoreDataError = 13, /* Pif Data Error On Load Or Store (Rb-200x And Later) */
    InstrAddrError = 14, /* Pif Address Error On Instruction Fetch (Rb-200x And Later) */
    LoadStoreAddrError = 15, /* Pif Address Error On Load Or Store (Rb-200x And Later) */
    ItlbMiss = 16, /* Itlb Miss (No Itlb Entry Matches, Hw Refill Also Missed) */
    ItlbMultiHit = 17, /* Itlb Multihit (Multiple Itlb Entries Match) */
    InstrRing = 18, /* Ring Privilege Violation On Instruction Fetch */
    /* Reserved				19 */	/* Size Restriction On Ifetch (Not Implemented) */
    InstrProhibited = 20, /* Cache Attribute Does Not Allow Instruction Fetch */
        /* Reserved				21..23 */
    DtlbMiss = 24, /* Dtlb Miss (No Dtlb Entry Matches, Hw Refill Also Missed) */
    DtlbMultiHit = 25, /* Dtlb Multihit (Multiple Dtlb Entries Match) */
    LoadStoreRing = 26, /* Ring Privilege Violation On Load Or Store */
        /* Reserved				27 */	/* Size Restriction On Load/Store (Not Implemented) */
    LoadProhibited = 28, /* Cache Attribute Does Not Allow Load */
    StoreProhibited = 29, /* Cache Attribute Does Not Allow Store */
             /* Reserved				30..31 */
    Cp0Disabled = 32, /* Access To Coprocessor 0 When Disabled */
    Cp2Disabled = 34, /* Access To Coprocessor 2 When Disabled */
    Cp3Disabled = 35, /* Access To Coprocessor 3 When Disabled */
    Cp1Disabled = 33, /* Access To Coprocessor 1 When Disabled */
    Cp4Disabled = 36, /* Access To Coprocessor 4 When Disabled */
    Cp5Disabled = 37, /* Access To Coprocessor 5 When Disabled */
    Cp6Disabled = 38, /* Access To Coprocessor 6 When Disabled */
    Cp7Disabled = 39, /* Access to Coprocessor 7 when disabled */
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
    if cause == Exccause::LevelOneInterrupt as u32 {
        _lowint1();
        return;
    }
    panic!("Ruh Roh")
}

/* Raw vector handlers */
// TODO window vectors
// macros this?

#[naked]
#[no_mangle]
#[link_section = ".Level2InterruptVector.text"]
pub unsafe extern "C" fn _Level2InterruptVector() -> ! {
    loop {}
}

#[naked]
#[no_mangle]
#[link_section = ".Level3InterruptVector.text"]
pub unsafe extern "C" fn _Level3InterruptVector() -> ! {
    loop {}
}

#[naked]
#[no_mangle]
#[link_section = ".Level4InterruptVector.text"]
pub unsafe extern "C" fn _Level4InterruptVector() -> ! {
    loop {}
}

#[naked]
#[no_mangle]
#[link_section = ".Level5InterruptVector.text"]
pub unsafe extern "C" fn _Level5InterruptVector() -> ! {
    loop {}
}

#[naked]
#[no_mangle]
#[link_section = ".DebugExceptionVector.text"]
pub unsafe extern "C" fn _DebugExceptionVector() -> ! {
    loop {} /* call0 is a jump, can't get here */
}

#[naked]
#[no_mangle]
#[link_section = ".NMIExceptionVector.text"]
pub unsafe extern "C" fn _NMIExceptionVector() -> ! {
    loop {} /* call0 is a jump, can't get here */
}

#[naked]
#[no_mangle]
#[link_section = ".KernelExceptionVector.text"]
pub unsafe extern "C" fn _KernelExceptionVector() -> ! {
    loop {} /* call0 is a jump, can't get here */
}

#[naked]
#[no_mangle]
#[link_section = ".UserExceptionVector.text"]
pub unsafe extern "C" fn _UserExceptionVector() -> ! {
    asm!("wsr.excsave1 a0"); /* preserve a0 */
    // TODO this doesn't work, we cannot call into Rust functions here: see idf asm
    asm!("call0 _rust_user_exc"); /* _UserExceptionVector must be < 64 bytes, jump to new method to ensure that */
    loop {} /* call0 is a jump, can't get here */
}

#[naked]
#[no_mangle]
#[link_section = ".DoubleExceptionVector.text"]
pub unsafe extern "C" fn _DoubleExceptionVector() -> ! {
    loop {} /* call0 is a jump, can't get here */
}