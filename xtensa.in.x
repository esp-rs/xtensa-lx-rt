

INCLUDE memory.x

ENTRY(Reset)

PROVIDE(__pre_init = DefaultPreInit); 

/* Define output sections */
SECTIONS {

  .text :
  {
    . = ALIGN (4);
    _stext = .;
    _text_start = ABSOLUTE(.);
    *(.literal .text .literal.* .text.*)
    _text_end = ABSOLUTE(.);
    _etext = .;
  } > ROTEXT

  .rodata :
  {
    . = ALIGN (4);
    _rodata_start = ABSOLUTE(.);
    *(.rodata .rodata.*)
    _rodata_end = ABSOLUTE(.);
  } > RODATA

  .data :
  {
    . = ALIGN (4);
    _data_start = ABSOLUTE(.);
    *(.data .data.*)
    _data_end = ABSOLUTE(.);
  } > RWDATA

  _data_start_loadaddr = LOADADDR(.data);

  .bss (NOLOAD) :
  {
    . = ALIGN (4);
    _bss_start = ABSOLUTE(.);
    *(.bss .bss.* COMMON)
    _bss_end = ABSOLUTE(.);
  } > RWDATA

  .noinit (NOLOAD) :
  {
    . = ALIGN(4);
    *(.noinit .noinit.*)
  } > RWDATA

  /* must be last segment using RWDATA */
  .heap_start (NOLOAD) :
  {
    . = ALIGN (4);
    _heap_start = ABSOLUTE(.);
  } > RWDATA

}
