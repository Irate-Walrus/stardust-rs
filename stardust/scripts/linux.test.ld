SECTIONS
{
	. = 0x10000;

	.text : {
	    *(.text.prologue)   /* aligns the stack by 16-bytes and executes implant entrypoint*/
        *(.text.implant)    /* implant */

        /* Ensure memcpy, memmove, memset, memcmp, bcmp, strlen are placed in .text */
        *(.text.memcpy)
        *(.text.memmove)
        *(.text.memset)
        *(.text.memcmp)
        *(.text.bcmp)
        *(.text.strlen)

        /* Unique function names */
        *(.text._*)
        *(.text..*)
        *(.text.unlikely*)
        *(.text.rust_oom)

        *(.rodata*)
        *(.got)

        *(.text.epilogue)   /* get RIP/EIP/PC at end of implant */

        FILL( 0x00 )
        . = ALIGN(0x1000);

        /* Insert `_data_offset` symbol at linking so it can be used within code. */
        _data_offset = .;
        *(.data*)           /* static, global, static local read-write data */
        *(.bss*)            /* unintialized static, global, static local read-write data */
	}

  	/DISCARD/ :
	{
		*(.interp)
		*(.comment)
		*(.debug_frame)
    	*(.gcc_except_table*)
    	*(.eh_frame*)
	}
}
