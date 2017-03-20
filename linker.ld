ENTRY(start)

SECTIONS {
    . = 1M;

    /* Multiboot v2 header */
    magic = 0xe85250d6;
    checksum = 0x100000000 - magic - (header_end - header_start);

    .multiboot : {
        header_start = .;

        LONG(magic)                      /* Magic number */
        LONG(0)                          /* x86 protected mode */
        LONG(header_end - header_start)  /* Header length */
        LONG(checksum)                   /* Checksum */

        /* End tag */
        SHORT(0)
        SHORT(0)
        LONG(8)

        header_end = .;
    }

    .text : {
        *(.text)
    }

    .bss : {
        /* We reserve 8 KiB of memory for the stack */
        . += 8K;
        bss = .;
    }

    /DISCARD/ : {}
}
