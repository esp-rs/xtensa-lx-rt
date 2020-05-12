
/* before memory.x to allow override */
ENTRY(Reset)

INCLUDE memory.x

/* after memory.x to allow override */
PROVIDE(__pre_init = DefaultPreInit); 
PROVIDE(__other_exception = __default_other_exception); 

/* needed to force inclusion of the vectors */
EXTERN(_UserExceptionVector);

/* Define output sections */
SECTIONS {

  .vectors :
  {
    . = 0x0;
    _init_start = ABSOLUTE(.);
    . = 0x0;
    KEEP(*(.WindowOverflow4.text));
    . = 0x40;
    KEEP(*(.WindowUnderflow4.text));
    . = 0x80;
    KEEP(*(.WindowOverflow8.text));
    . = 0xC0;
    KEEP(*(.WindowUnderflow8.text));
    . = 0x100;
    KEEP(*(.WindowOverflow12.text));
    . = 0x140;
    KEEP(*(.WindowUnderflow12.text));
    . = 0x180;
    KEEP(*(.Level2InterruptVector.text));
    . = 0x1c0;
    KEEP(*(.Level3InterruptVector.text));
    . = 0x200;
    KEEP(*(.Level4InterruptVector.text));
    . = 0x240;
    KEEP(*(.Level5InterruptVector.text));
    . = 0x280;
    KEEP(*(.DebugExceptionVector.text));
    . = 0x2c0;
    KEEP(*(.NMIExceptionVector.text));
    . = 0x300;
    KEEP(*(.KernelExceptionVector.text));
    . = 0x340;
    KEEP(*(.UserExceptionVector.text));
    . = 0x3C0;
    KEEP(*(.DoubleExceptionVector.text));
    . = 0x400;

    _init_end = ABSOLUTE(.);
  } > vectors_seg

  .text :
  {
    _stext = .;
    _text_start = ABSOLUTE(.);
    . = ALIGN (4);
    *(.literal .text .literal.* .text.*)
    _text_end = ABSOLUTE(.);
    _etext = .;
  } > ROTEXT

  .rodata :
  {
    _rodata_start = ABSOLUTE(.);
    . = ALIGN (4);
    *(.rodata .rodata.*)
    _rodata_end = ABSOLUTE(.);
  } > RODATA

  .data :
  {
    _data_start = ABSOLUTE(.);
    . = ALIGN (4);
    *(.data .data.*)
    _data_end = ABSOLUTE(.);
  } > RWDATA AT > RODATA

  .bss (NOLOAD) :
  {
    _bss_start = ABSOLUTE(.);
    . = ALIGN (4);
    *(.bss .bss.* COMMON)
    _bss_end = ABSOLUTE(.);
  } > RWDATA

  .noinit (NOLOAD) :
  {
    . = ALIGN(4);
    *(.noinit .noinit.*)
  } > RWDATA

 /* must be last segment using RWTEXT */
  .text_heap_start (NOLOAD) :
  {
    . = ALIGN (4);
    _text_heap_start = ABSOLUTE(.);
  } > RWTEXT

 /* must be last segment using RWDATA */
  .heap_start (NOLOAD) :
  {
    . = ALIGN (4);
    _heap_start = ABSOLUTE(.);
  } > RWDATA
}