ENTRY(start)

SECTIONS {
    . = 1M;
    .boot :
    {
        /* make sure not to garbage collected by ld */
        KEEP(*(.multiboot_header))
    }
    
    .text :
    {
        *(.text .text.*)
        /* merge text subsections  */
    }

    .rodata : {
      *(.rodata .rodata.*)

        /* merge rodata subsections  */
    }

    .data.rel.ro : {
      *(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
    }
}

