ENTRY(_start);

SECTIONS
{
	.text :	{
		. = 0x0;				/* this will force linker error if .text section is not at 0x0 */
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

    	/*
    	`FILL (pattern)` specifies the fill pattern for the current session.
        Any otherwise unspecified regions of memory within the section
        (for example, gaps left due to the required alignment of input sections)
        are filled with the two least significant bytes of the expression.
    	*/
    	FILL( 0x00 )            /* fills unspecified regions of memory with 0x00 */

    	/*
    	Return the location counter (.) aligned to the next `exp` boundary.
    	`exp` must be a be an expression whose value is a power of two.
    	ALIGN doesn't change the value of the location counter, unless you assign it with `=`.

    	This aligns page by 0x1000 or 4096 bytes, the default Normal Page Size on Windows & Linux x86_32 and x86_64.
    	So that the `.data` section gets its own page.
    	*/
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

	/DISCARD/ :
	{
		*(.eh_frame*)			/* without this the text section base cannot be 0x0 */
		*(.gcc_except*)			/* ditto */
		*(.interp)
		*(.comment)
		*(.debug_frame)
	}
}
