use core::arch::{asm, global_asm};

global_asm!(include_str!("i686.asm"));

#[inline(never)]
pub fn rip_start() -> *mut usize {
    let addr: *mut usize;

    unsafe {
        asm!(
            "call _rip_start",  // call the assembly function
            "mov {0}, eax",     // move the value in rax to addr
            out(reg) addr       // output to addr
        );
    }

    addr
}

#[inline(never)]
pub fn rip_end() -> *mut usize {
    let addr: *mut usize;

    unsafe {
        asm!(
            "call _rip_end",  // call the assembly function
            "mov {0}, eax",     // move the value in rax to addr
            out(reg) addr       // output to addr
        );
    }

    addr
}
