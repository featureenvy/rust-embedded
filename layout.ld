MEMORY
{
  FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 0x00040000
  RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 0x00008000
}

SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    LONG(ORIGIN(RAM) + LENGTH(RAM))
    KEEP(*(.reset))
  } > FLASH

  .text :
  {
    *(.text*)
  } > FLASH

  /DISCARD/ :
  {
    *(.ARM.exidx*)
    *(.note.gnu.build-id*)
  }
}
