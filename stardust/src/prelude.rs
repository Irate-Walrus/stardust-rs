use core::arch::{asm, global_asm};

#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("x86_64.asm"));

extern "C" {
    static _data_offset: usize;
}

pub fn rip_start() -> *const usize {
    let addr: *const usize;

    unsafe {
        asm!(
            "call _rip_start",  // call the assembly function
            "mov {0}, rax",     // move the value in rax to addr
            out(reg) addr       // output to addr
        );
    }

    addr
}

pub fn rip_end() -> *const usize {
    let addr: *const usize;

    unsafe {
        asm!(
            "call _rip_end",    // call the assembly function
            "mov {0}, rax",     // move the value in rax to addr
            out(reg) addr       // output to addr
        );
    }

    addr
}

pub fn data_offset() -> usize {
    unsafe { _data_offset }
}

/*
#[no_mangle]
#[link_section = ".text.align"]
/// Aligns the stack by 16-bytes to avoid unwanted crashes when calling win32 (x86) functions and execute the true entrypoint
pub unsafe extern "C" fn _align() {
    // TODO: We've used the linker to align this by 16 (0x10) bytes, not sure this is necessary.
    asm!(
        "
        push rdi                    // backup rdi since we will be using this as our main register
        mov rdi, rsp                // save stack pointer to rdi
        and rsp, 0xfffffffffffffff0 // align stack with 16 bytes
        sub rsp, 0x20               // allocate some space for our C function
        call _init                  // call the C function
        mov rsp, rdi                // restore stack pointer
        pop rdi                     // restore rdi
        ret                         // return where we left
        "
    );
}

#[no_mangle]
#[link_section = ".text.align"]
/// Get RIP (x64)/EIP(x32)/PC to start of implant.
pub unsafe extern "C" fn _rip_start() {
    asm!(
        "
        call _rip_ptr_start
        ret
        "
    );
}

#[no_mangle]
#[link_section = ".text.align"]
/// Get return address of _rip_start() and put in rax register.
pub unsafe extern "C" fn _rip_ptr_start() {
    asm!(
        "
        mov	rax, [rsp]      // get the return address
        sub rax, 0x1b       // subtract the instructions size to get the base address
        ret                 // return
        "
    );
}

#[no_mangle]
#[link_section = ".text.end"]
/// Get RIP (x64)/EIP(x32)/PC at end of implant.
pub unsafe extern "C" fn _rip_end() {
    asm!(
        "
        call _rip_ptr_end
        ret
        "
    );
}

#[no_mangle]
#[link_section = ".text.end"]
/// Get return address of _rip_end() and put in rax register.
pub unsafe extern "C" fn _rip_ptr_end() {
    asm!(
        "
        mov	rax, [rsp]      // get the return address
        add rax, 0xb        // get end implant
        ret                 // return
        "
    );
}

#[no_mangle]
#[link_section = ".text.sym_end"]
/// Get return address of _rip_end() and put in rax register.
pub unsafe extern "C" fn _sym_end() {
    /*
    asm!(
        "
        db 'S', 'T', 'A', 'R', 'D', 'U', 'S', 'T', '-', 'E', 'N', 'D'
        "
    );
    */
}
*/
