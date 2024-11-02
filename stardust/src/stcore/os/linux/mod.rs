use syscalls::{syscall, Sysno};

mod allocator;
pub mod libc;
mod loadlib;

pub use allocator::StardustAllocator;
pub use loadlib::{find_fn_in_lib, find_lib};

use crate::{data_offset, epilogue_offset, got_offset};

/// Log &str to stdout
pub fn log_str(s: &str) {
    unsafe { write(0x1, s.as_ptr(), s.len()) }
}

/// Write to file descriptor using the `write` syscall
pub unsafe fn write(fd: usize, buf: *const u8, count: usize) {
    let _ = syscall!(Sysno::write, fd, buf, count);
}

/// Set data, bss, and got page to RW
/// really this only protects `size_of::<usize>()` but it'll flip the entire page
/// including `rip_end()`, so don't call that again
pub unsafe fn rw_page(ptr: *const usize) {
    let offset = data_offset();
    let ptr = unsafe { ptr.add(offset / size_of::<usize>()) };
    let _ = syscall!(Sysno::mprotect, ptr, size_of::<usize>(), 0x1 | 0x2);
}

/// Patch hardcoded memory addresses in the GLOBAL_OFFSET_TABLE
/// this has the side effect of changing the values of *_offset() to their actual addresses
/// but we can't call `rip_end()` after `mprotect` call anyway
pub unsafe fn patch_got_offsets(ptr: *const usize) {
    let offset = got_offset() - 1; // I don't know why this off-by-one error exists, but it does.
    let len = epilogue_offset() - offset;
    let got_addr = ptr.add(offset / size_of::<usize>());

    let count = len / core::mem::size_of::<usize>();

    for i in 0..count {
        let value = got_addr.add(i) as *mut usize;
        *value += ptr as usize;
    }
}
