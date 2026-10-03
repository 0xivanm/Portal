ENTRY(_start)


SECTIONS
{
    . = 0x10000000;

    .text : {
        KEEP(*(.text.init))
        *(.text*)
    }

    .ARM.exidx : {
        *(.ARM.exidx*)
    }

    .rodata : {
        *(.rodata*)
    }

    .data : {
        *(.data*)
    }

    .bss (NOLOAD) : {
        /* zeroing will write 4 byte words */
        . = ALIGN(4);
        _bssbegin = .;
        *(.bss*)
        *(COMMON)
        . = ALIGN(4);
        _bssend = .;
    }

    .stack (NOLOAD) : {
        . = ALIGN(8);
        _stackbegin = .;
        . += 0x2000;
        _stackend = .;
    }
}
