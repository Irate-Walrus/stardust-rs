use core::arch::{asm, global_asm};

#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("x86_64.asm"));

extern "C" {
    static mut _data_offset: usize;
    static mut _got_offset: usize;
    static mut _epilogue_offset: usize;
}

/*
    So it turns out that if you were to use _rip_start() or _rip_end() directly `lld` will decide a Global Offset Table is needed...
    This would be painful to deal with, so lets just create those functions in Rust
*/

#[cfg(target_arch = "x86_64")]
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

#[cfg(target_arch = "x86_64")]
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

pub fn data_offset() -> usize {
    #[allow(unused_unsafe)]
    unsafe {
        let offset_addr = &raw mut _data_offset;
        offset_addr as usize
    }
}
// TODO: migrate entire contents of x86_64.asm into Rust.

pub fn got_offset() -> usize {
    #[allow(unused_unsafe)]
    unsafe {
        let offset_addr = &raw mut _got_offset;
        offset_addr as usize
    }
}

pub fn epilogue_offset() -> usize {
    #[allow(unused_unsafe)]
    unsafe {
        let offset_addr = &raw mut _epilogue_offset;
        offset_addr as usize
    }
}
