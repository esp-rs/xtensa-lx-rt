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

    .set PS_INTLEVEL_EXCM, 3	
    .set PS_INTLEVEL_MASK, 0x0000000f
    .set PS_EXCM,          0x00000010
    .set PS_UM,            0x00000020
    .set PS_WOE,           0x00040000

    .set PS_EXC_MODE, PS_INTLEVEL_EXCM | PS_UM | PS_WOE
    
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
    rsr     a0, EXCSAVE1
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
        
        call4   __exception               // call handler <= actual call!

        j       .RestoreContext

        .Level1Interrupt:
        movi    a6, 1                     // put interrupt level in a6 = a2 in callee
        
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
