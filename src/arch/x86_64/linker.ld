ENTRY(start)

SECTIONS {
    . = 1M;

    .boot :
    {
        /* ensure that the multiboot header is at the beginninh */
        *(.multiboot_header)
    }

    .text :
    {
        *(.text)
    }
}
