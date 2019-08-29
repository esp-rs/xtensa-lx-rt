

INCLUDE memory.x


ENTRY(Reset)



/* Define output sections */
SECTIONS {

  /* The program code and other data goes into Instruction RAM */
  .iram0.text :
  {
    /* Code marked as runnning out of IRAM */

    _iram_text_start = ABSOLUTE(.);
    *(.iram1 .iram1.*)
    *libphy.a:(.literal .text .literal.* .text.*)
    *librtc.a:(.literal .text .literal.* .text.*)
    *libpp.a:(.literal .text .literal.* .text.*)
    *libhal.a:(.literal .text .literal.* .text.*)
    _iram_text_end = ABSOLUTE(.);
  } > iram_seg
  /*.iram.text :
  {
    . = ALIGN(16);
    KEEP(*(.entry.text))
    *(.text)
    *(.text*)
    KEEP (*(.init))
    KEEP (*(.fini))
    *(.rodata)
    *(.rodata*)

    . = ALIGN(4);
    _etext = .;
  } >iram_seg */

  /* Initialized data goes into Data RAM */
  _sidata = .;
  .data : AT(_sidata)
  {
    . = ALIGN(4);
    _sdata = .;
    *(.data)
    *(.data*)

    . = ALIGN(4);
    _edata = .;
  } >dram_seg

  /* Uninitialized data also goes into Data RAM */
  .bss :
  {
    . = ALIGN(4);
    _sbss = .;
    *(.bss)
    *(.bss*)
    *(COMMON)

    . = ALIGN(4);
    _ebss = .;
  } >dram_seg

  . = ALIGN(4);
  PROVIDE ( end = . );
  PROVIDE ( _end = . );
}
