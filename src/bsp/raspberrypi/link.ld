/* SPDX-License-Identifier: MIT OR Apache-2.0
 *
 * Copyright (c) 2018-2020 Andre Richter <andre.o.richter@gmail.com>
 */

SECTIONS
{
    /* Set current address to 32 MiB */
    . = 0x80000;

    __ro_start = .;
    .text :
    {
        *(.text._start) *(.text*)
    }

    .exception_vectors :
    {
        *(.exception_vectors*)
    }

    .rodata :
    {
        *(.rodata*)
    }
    . = ALIGN(65536); /* Fill up to 64 KiB */
    __ro_end = .;

    .data :
    {
        *(.data*)
    }

    /* Zero section in u64 chunks, align start and end to 8 bytes */
    .bss ALIGN(8):
    {
        __bss_start = .;
        *(.bss*);
        . = ALIGN(8);

        /* Fill for the bss == 0 case, so that __bss_start <= __bss_end_inclusive holds */
        . += 8;
        __bss_end_inclusive = . - 8;
    }

    /DISCARD/ : { *(.comment*) }
}
