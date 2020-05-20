global_asm!(
    "
    .set XT_STK_PC,              0 
    .set XT_STK_PS,              4 
    .set XT_STK_A0,              8 
    .equ XT_STK_A1,             12 
    .set XT_STK_A2,             16 
    .set XT_STK_A3,             20 
    .set XT_STK_A4,             24 
    .set XT_STK_A5,             28 
    .set XT_STK_A6,             32 
    .set XT_STK_A7,             36 
    .set XT_STK_A8,             40 
    .set XT_STK_A9,             44 
    .set XT_STK_A10,            48 
    .set XT_STK_A11,            52 
    .set XT_STK_A12,            56 
    .set XT_STK_A13,            60 
    .set XT_STK_A14,            64 
    .set XT_STK_A15,            68 
    .set XT_STK_SAR,            72 
    .set XT_STK_EXCCAUSE,       76
    .set XT_STK_EXCVADDR,       80
    .set XT_STK_LBEG,           84 // Registers for Loop Option
    .set XT_STK_LEND,           88
    .set XT_STK_LCOUNT,         92
    .set XT_STK_THREADPTR,      96 // freely usable 32-bit register intended for TLS
    .set XT_STK_SCOMPARE1,     100 // Register for s32ci instruction
    .set XT_STK_BR,            104 // Register for Boolean Option
    .set XT_STK_ACCLO,         108 // Registers for MAC16 option
    .set XT_STK_ACCHI,         112
    .set XT_STK_M0,            116
    .set XT_STK_M1,            120
    .set XT_STK_M2,            124
    .set XT_STK_M3,            128
    .set XT_STK_F64R_LO,       132 // Registers for double support option
    .set XT_STK_F64R_HI,       136
    .set XT_STK_F64S,          140
    .set XT_STK_FCR,           144 // Registers for floating point coprocessor
    .set XT_STK_FSR,           148
    .set XT_STK_F0,            152
    .set XT_STK_F1,            156
    .set XT_STK_F2,            160
    .set XT_STK_F3,            164
    .set XT_STK_F4,            168
    .set XT_STK_F5,            172
    .set XT_STK_F6,            176
    .set XT_STK_F7,            180
    .set XT_STK_F8,            184
    .set XT_STK_F9,            188
    .set XT_STK_F10,           192
    .set XT_STK_F11,           196
    .set XT_STK_F12,           200
    .set XT_STK_F13,           204
    .set XT_STK_F14,           208
    .set XT_STK_F15,           212

    .set XT_STK_BASESAVE,      240
    .set XT_STK_FRMSZ,         256  // needs to be multiple of 16 and at least 16 free 
                                    // (for base save region)
                                    // multiple of 256 allows use of addmi instruction



    .set PS_INTLEVEL_EXCM, 3	
    .set PS_INTLEVEL_MASK, 0x0000000f
    .set PS_EXCM,          0x00000010
    .set PS_UM,            0x00000020
    .set PS_WOE,           0x00040000
    "
);

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
        rur     a3, f64r_lo 
        s32i    a3, sp, +XT_STK_F64R_LO
        rur     a3, f64r_hi
        s32i    a3, sp, +XT_STK_F64R_HI
        rur     a3, f64s   
        s32i    a3, sp, +XT_STK_F64S

        // Coprocessor Option
        rur     a3, fcr
        s32i    a3, sp, +XT_STK_FCR
        rur     a3, fsr
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
        
        mov     a9, a0                   // store return address
        addmi   sp,  sp, +XT_STK_FRMSZ   // go back to spill register region

        SPILL_REGISTERS

        addmi   sp,  sp, -XT_STK_FRMSZ   // return the current stack pointer
        mov     a0, a9                   // retrieve return address

        ret
    "
    )
}

global_asm!(
    r#"
    .macro SPILL_REGISTERS
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
    .endm
    "#
);

global_asm!(
    r#"
    .macro SAVE_CONTEXT level:req
    mov     a0, a1                     // save a1/sp
    addmi   sp, sp, -XT_STK_FRMSZ      // only allow multiple of 256

    s32i    a0, sp, +XT_STK_A1         // save interruptee's A1/SP
    s32e    a0, sp, -12                // for debug backtrace 

    .ifc \level,double
    rsr     a0, DEPC                   
    .else
    rsr     a0, EPC\level                   
    .endif
    s32i    a0, sp, +XT_STK_PC         // save interruptee's PC 
    s32e    a0, sp, -16                // for debug backtrace 

    .ifc \level,double
    rsr     a0, EXCSAVE7               // ok to reuse EXCSAVE7 for double exception as long as
                                       // double exception is not in first couple of instructions
                                       // of level 7 handler
    .else
    rsr     a0, EXCSAVE\level               
    .endif
    s32i    a0, sp, +XT_STK_A0         // save interruptee's A0 

    .ifc \level,1
    rsr     a0, PS                     
    s32i    a0, sp, +XT_STK_PS         // save interruptee's PS

    rsr     a0, EXCCAUSE
    s32i    a0, sp, +XT_STK_EXCCAUSE
    rsr     a0, EXCVADDR
    s32i    a0, sp, +XT_STK_EXCVADDR
    .endif

    .ifc \level,double
    rsr     a0, EXCCAUSE
    s32i    a0, sp, +XT_STK_EXCCAUSE
    rsr     a0, EXCVADDR
    s32i    a0, sp, +XT_STK_EXCVADDR
    .endif

    // clear EXCM so other exceptions like window overflow can be handled normally again.
    // If level 1 interrupt or exception then block level 1 interrupts (for higher level
    // interrupts, this is done automatically).
    rsr     a0, PS
    .ifc \level,1
    movi    a2, ~(PS_EXCM|PS_INTLEVEL_MASK)
    and     a0, a0, a2
    movi    a2, 1
    or      a0, a0, a2
    .else
    movi    a2, ~PS_EXCM
    and     a0, a0, a2
    .endif
    wsr     a0, PS
    rsync                              // wait for WSR.PS to complete 

    call0   save_context
    
    .endm
    "#
);

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
        wur     a3, f64r_lo
        l32i    a3, sp, +XT_STK_F64R_HI
        wur     a3, f64r_hi
        l32i    a3, sp, +XT_STK_F64S
        wur     a3, f64s

        // Coprocessor Option
        l32i    a3, sp, +XT_STK_FCR
        wur     a3, fcr
        l32i    a3, sp, +XT_STK_FSR
        wur     a3, fsr
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

global_asm!(
    r#"
    .macro RESTORE_CONTEXT level:req
    
    // Restore context and return 
    call0   restore_context

    .ifc \level,1
    l32i    a0, sp, +XT_STK_PS        // retrieve interruptee's PS 
    wsr     a0, PS
    l32i    a0, sp, +XT_STK_PC        // retrieve interruptee's PC 
    wsr     a0, EPC\level
    .endif 

    l32i    a0, sp, +XT_STK_A0        // retrieve interruptee's A0 
    l32i    sp, sp, +XT_STK_A1        // remove exception frame 
    rsync                             // ensure PS and EPC written 
    
    .endm
    "#
);

/// Handle Other Exceptions or Level 1 interrupt by storing full context and then
/// calling regular function
///
/// # Input:
///    * A0 stored in EXCSAVE1
#[naked]
#[no_mangle]
#[link_section = ".rwtext"]
unsafe extern "C" fn __default_naked_exception() {
    asm!(
        "
        SAVE_CONTEXT 1

        l32i    a6, sp, +XT_STK_EXCCAUSE  // put cause in a6 = a2 in callee
        beqi    a6, 4, .Level1Interrupt

        mov     a7, sp                    // put address of save frame in a7=a3 in callee
        call4   __exception               // call handler <= actual call!

        j       .RestoreContext

        .Level1Interrupt:
        movi    a6, 1                     // put interrupt level in a6 = a2 in callee
        mov     a7, sp                    // put address of save frame in a7=a3 in callee
        call4   __level_1_interrupt       // call handler <= actual call!

        .RestoreContext:
        RESTORE_CONTEXT 1
        
        .byte 0x00, 0x30, 0x00            // rfe   // PS.EXCM is cleared 
                                          // TODO: 20200509, not supported in llvm yet
        "
    )
}

/// Handle Double Exceptions by storing full context and then calling regular function
///
/// # Input:
///    * A0 stored in ???
#[naked]
#[no_mangle]
#[link_section = ".rwtext"]
unsafe extern "C" fn __default_naked_double_exception() {
    asm!(
        "
        SAVE_CONTEXT double

        l32i    a6, sp, +XT_STK_EXCCAUSE  // put cause in a6 = a2 in callee
        mov     a7, sp                    // put address of save frame in a7=a3 in callee
        call4   __double_exception        // call handler <= actual call!

        RESTORE_CONTEXT double

        .byte 0x00, 0x32, 0x00            // rfde   
                                          // TODO: 20200509, not supported in llvm yet
        "
    )
}

global_asm!(
    r#"
    .macro HANDLE_INTERRUPT_LEVEL level

    SAVE_CONTEXT \level

    movi    a6, \level                     // put interrupt level in a6 = a2 in callee
    mov     a7, sp                         // put address of save frame in a7=a3 in callee
    call4   __level_\level\()_interrupt    // call handler <= actual call!

    RESTORE_CONTEXT \level
    
    rfi \level

    .endm
"#
);

/// Handle Level 2 Interrupt by storing full context and then calling regular function
///
/// # Input:
///    * A0 stored in EXCSAVE2
#[naked]
#[no_mangle]
#[link_section = ".rwtext"]
unsafe extern "C" fn __default_naked_level_2_interrupt() {
    asm!("HANDLE_INTERRUPT_LEVEL 2");
}

/// Handle Level 3 Interrupt by storing full context and then calling regular function
///
/// # Input:
///    * A0 stored in EXCSAVE3
#[naked]
#[no_mangle]
#[link_section = ".rwtext"]
unsafe extern "C" fn __default_naked_level_3_interrupt() {
    asm!("HANDLE_INTERRUPT_LEVEL 3");
}

/// Handle Level 4 Interrupt by storing full context and then calling regular function
///
/// # Input:
///    * A0 stored in EXCSAVE4
#[naked]
#[no_mangle]
#[link_section = ".rwtext"]
unsafe extern "C" fn __default_naked_level_4_interrupt() {
    asm!("HANDLE_INTERRUPT_LEVEL 4");
}

/// Handle Level 5 Interrupt by storing full context and then calling regular function
///
/// # Input:
///    * A0 stored in EXCSAVE5
#[naked]
#[no_mangle]
#[link_section = ".rwtext"]
unsafe extern "C" fn __default_naked_level_5_interrupt() {
    asm!("HANDLE_INTERRUPT_LEVEL 5");
}

/// Handle Level 6 (=Debug) Interrupt by storing full context and then calling regular function
///
/// # Input:
///    * A0 stored in EXCSAVE6
#[naked]
#[no_mangle]
#[link_section = ".rwtext"]
unsafe extern "C" fn __default_naked_level_6_interrupt() {
    asm!("HANDLE_INTERRUPT_LEVEL 6");
}

/// Handle Level 7 (=NMI) Interrupt by storing full context and then calling regular function
///
/// # Input:
///    * A0 stored in EXCSAVE7
#[naked]
#[no_mangle]
#[link_section = ".rwtext"]
unsafe extern "C" fn __default_naked_level_7_interrupt() {
    asm!("HANDLE_INTERRUPT_LEVEL 7");
}
