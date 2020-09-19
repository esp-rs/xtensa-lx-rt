/* exception vector for the lx106, only requiring the basic exception support */

/* high level exception/interrupt routines, which can be override with Rust functions */
PROVIDE(__exception = __default_exception);
PROVIDE(__kernel_exception = __default_exception);
PROVIDE(__double_exception = __default_double_exception);
PROVIDE(__nmi_exception = __default_exception);
PROVIDE(__debug_exception = __default_exception);
PROVIDE(__alloc_exception = __default_exception);
PROVIDE(__level_1_interrupt = __default_interrupt);

/* low level exception/interrupt, which must be overridden using naked functions */
PROVIDE(__naked_user_exception = __default_naked_exception);
PROVIDE(__naked_kernel_exception = __default_naked_kernel_exception);
PROVIDE(__naked_double_exception = __default_naked_double_exception);
PROVIDE(__naked_nmi_exception = __default_naked_nmi_exception);
PROVIDE(__naked_debug_exception = __default_naked_debug_exception);
PROVIDE(__naked_alloc_exception = __default_naked_alloc_exception);

/* needed to force inclusion of the vectors */
EXTERN(__default_exception);
EXTERN(__default_double_exception);
EXTERN(__default_interrupt);

EXTERN(__default_naked_exception);
EXTERN(__default_naked_exception);
EXTERN(__default_naked_double_exception);
EXTERN(__default_naked_nmi_exception);
EXTERN(__default_naked_debug_exception);
EXTERN(__default_naked_alloc_exception);

/* Define output sections */
SECTIONS {

  .vectors :
  {
    . = 0x0;
    _init_start = ABSOLUTE(.);
    . = 0x10;
    KEEP(*(.DebugException.text));
    . = 0x20;
    KEEP(*(.NMIException.text));
    . = 0x40;
    KEEP(*(.KernelException.text));
    . = 0x50;
    KEEP(*(.UserException.text));
    . = 0x70;
    KEEP(*(.DoubleException.text));
    . = 0x80;

    _init_end = ABSOLUTE(.);
  } > vectors_seg
}