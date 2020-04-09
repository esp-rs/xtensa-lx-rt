

INCLUDE memory.x

ENTRY(Reset)

/* Define output sections */
SECTIONS {

  .text :
  {
    _stext = .;
    _text_start = ABSOLUTE(.);
    *(.literal .text .literal.* .text.*)
    _text_end = ABSOLUTE(.);
    _etext = .;
  } > ROTEXT

  .rodata :
  {
    . = ALIGN (8);
    _rodata_start = ABSOLUTE(.);
    *(.rodata .rodata.*)
    _rodata_end = ABSOLUTE(.);
  } > RODATA

  .data :
  {
    . = ALIGN (8);
    _data_start = ABSOLUTE(.);
    *(.data .data.*)
    _data_end = ABSOLUTE(.);
  } > RWDATA

  _sidata = LOADADDR(.data);

  .bss (NOLOAD) :
  {
    . = ALIGN (8);
    _bss_start = ABSOLUTE(.);
    *(.bss .bss.*)
    _bss_end = ABSOLUTE(.);
  } > RWDATA

  .heap_start (NOLOAD) :
  {
    . = ALIGN (8);
    _heap_start = ABSOLUTE(.);
  } > RWDATA

}
