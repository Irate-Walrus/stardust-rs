//
// stardust-rs
//

// imports
.extern _init

// export
.globl _start
.globl _rip_ptr_start
.globl _rip_ptr_end

// entrypoint
.section ".text.prologue"
    //
    // shellcode entrypoint
    // aligns the stack by 16-bytes to avoid any unwanted
    // crashes while calling win32 functions and execute
    // the true C code entrypoint
    // TY Cracked5pider
    //
    _start:
        push  rsi
        mov   rsi, rsp
        and   rsp, 0xFFFFFFFFFFFFFFF0
        sub   rsp, 0x20
        call  _stmain
        mov   rsp, rsi
        pop   rsi
        ret

    // get rip to the start of the agent
    _rip_start:
        call _rip_ptr_start
        ret

    // get the return address of _rip_str and put it into the rax register
    _rip_ptr_start:
        mov rax, [rsp] // get the return address
        sub rax, 0x1b  // subtract the instructions size to get the base address
        ret            // return to _rip_start

// end of implant code
.section ".text.epilogue"

    // get end of the implant
    _rip_end:
        call _rip_ptr_end
        ret

    // get the return address of _rip_end and put it into the rax register
    _rip_ptr_end:
        mov rax, [rsp] // get the return address
        add rax, 0xa   // get implant end address
        ret            // return to _rip_end
