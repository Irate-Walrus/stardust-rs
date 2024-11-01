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
        push   esi               // save esi
        mov    esi, esp          // store current esp in esi
        and    esp, 0xFFFFFFF0   // align esp to 16 bytes
        sub    esp, 0x10         // allocate stack space for alignment
        call   main              // call the main function
        mov    esp, esi          // restore esp
        pop    esi               // restore esi
        ret

    // get rip to the start of the agent
    _rip_start:
        call   _rip_ptr_start
        ret

    // get the return address of _rip_str and put it into the eax register
    _rip_ptr_start:
        mov    eax, [esp]        // get the return address
        sub    eax, 0x17         // adjust to get base address
        ret                      // return to _rip_start

// end of implant code
.section ".text.epilogue"

    // get end of the implant
    _rip_end:
        call   _rip_ptr_end
        ret

    // get the return address of _rip_end and put it into the eax register
    _rip_ptr_end:
        mov    eax, [esp]        // get the return address
        add    eax, 0x8          // calculate implant end address
        ret                      // return to _rip_end
