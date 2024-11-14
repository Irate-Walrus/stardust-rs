use core::arch::{asm, global_asm};

global_asm!(include_str!("x86_64.asm"));

/*
    So it turns out that if you were to use _rip_start() or _rip_end() directly `lld` will decide a Global Offset Table is needed...
    This would be painful to deal with, so lets just create those functions in Rust
*/

#[inline(never)]
pub fn rip_start() -> *mut usize {
    let addr: *mut usize;

    unsafe {
        asm!(
            "call _rip_start",  // call the assembly function
            "mov {0}, rax",     // move the value in rax to addr
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
            "mov {0}, rax",     // move the value in rax to addr
            out(reg) addr       // output to addr
        );
    }

    addr
}
