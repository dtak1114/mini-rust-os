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
        *(.text)
    }

}

