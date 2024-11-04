SECTIONS
{
	. = 0x10000;

	.text : {
		*(.text.prologue)       /* aligns the stack by 16-bytes and executes implant entrypoint*/
		*(.text.implant)        /* implant */

        /* Ensure memcpy, memmove, memset, memcmp, bcmp, strlen are placed in .text */
        *(.text.memcpy)
        *(.text.memmove)
        *(.text.memset)
        *(.text.memcmp)
        *(.text.bcmp)
        *(.text.strlen)

        *(.text._*)             /* rust auto-generated helpers/junk */
        *(.text..Lanon*)        /* rust auto-generated helpers/junk */
        *(.text.unlikely*)      /* unlikely executed functions, related to compiler optimization */
        *(.text.rust_oom)       /* rust auto-generated alloc helper */

		*(.rodata*)             /* read-only (const) data */

        FILL( 0x00 )
        . = ALIGN(0x1000);

        /* Insert `_data_offset` symbol at linking so it can be used within code. */
        _data_offset = .;
		*(.data*)               /* initialized static, global, static local read-write data */
        *(.bss*)                /* unintialized static, global, static local read-write data */
        _got_offset = .;
        *(.got*)				/* include global offset table so that _data_offset symbol can be used*/
		_epilogue_offset = .;
		*(.text.epilogue)       /* get RIP/EIP/PC at end of implant */
	}
}
