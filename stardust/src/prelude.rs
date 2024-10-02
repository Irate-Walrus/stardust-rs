use core::arch::{asm, global_asm};

#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("x86_64.asm"));

extern "C" {
    pub static mut _data_offset: usize;
}

/*
    So it turns out that if you were to use _rip_start() or _rip_end() directly `lld` will decide a `got` is needed...
    This would be painful to deal with, so lets just create those functions in Rust
*/

#[cfg(target_arch = "x86_64")]
#[link_section = ".text.prologue"]
#[inline(never)]
pub fn rip_start() -> *const usize {
    let addr: *const usize;

    unsafe {
        asm!(
            "call _rip_ptr_start",  // call the assembly function
            "mov {0}, rax",     // move the value in rax to addr
            out(reg) addr       // output to addr
        );
    }

    addr
}

#[cfg(target_arch = "x86_64")]
#[link_section = ".text.epilogue"]
#[inline(never)]
pub fn rip_end() -> *const usize {
    let addr: *const usize;

    unsafe {
        asm!(
            "call _rip_ptr_end",  // call the assembly function
            "mov {0}, rax",     // move the value in rax to addr
            out(reg) addr       // output to addr
        );
    }

    addr
}

// TODO: migrate entire contents of x86_64.asm into Rust.
