ENTRY(_start);

SECTIONS
{
    .text ALIGN(16):
    {
		. = 0x0;
		*(.text.prologue)       /* aligns the stack by 16-bytes and executes implant entrypoint */
		*(.text.implant)        /* implant */

        *(.text)

		*(.rodata*)             /* read-only (const) data */
        *(.rdata*)

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

        _data_offset = .;
        *(.global*)
        *(.data*)
        *(.bss*)
		_epilogue_offset = .;
		*(.text.epilogue)       /* get RIP/EIP/PC at end of implant */
    }

    /DISCARD/ :
    {
        *(.interp)
        *(.comment)
        *(.debug_frame)
        *(.pdata)
        *(.xdata)
    }
}
