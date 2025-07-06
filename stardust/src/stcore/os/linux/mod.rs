use alloc::boxed::Box;
use core::sync::atomic::Ordering;
use core::{ffi::c_void, intrinsics};
use syscalls::{syscall, Sysno};

mod allocator;
pub mod libc;
mod resolve;

pub use allocator::StLinuxAllocator;
pub use resolve::{resolve_function, resolve_module};

use crate::stcore::*;

/// Log &str to stdout
pub fn log_str(s: &str) {
    // If write has been resolved lets use that for logging
    if let Some(instance) = Instance::get() {
        if let Some(write_fn) = instance.libc.write {
            unsafe { write_fn(1, s.as_ptr(), s.len()) };
            return;
        }
    }
    // Otherwise, lets just use the write syscall
    unsafe { write(0x1, s.as_ptr(), s.len()) }
}

/// Write to file descriptor using the `write` syscall
pub unsafe fn write(fd: usize, buf: *const u8, count: usize) {
    let _ = syscall!(Sysno::write, fd, buf, count);
}

/// Set data, bss, and got page to RW
/// really this only protects `size_of::<usize>()` but it'll flip the entire page
/// including `rip_end()`, so don't call that again
pub unsafe fn rw_page(ptr: *mut c_void) {
    let offset = data_offset();
    let ptr = ptr.byte_add(offset);
    let _ = syscall!(Sysno::mprotect, ptr, size_of::<usize>(), 0x1 | 0x2);
}

/// Patch hardcoded memory addresses in the GLOBAL_OFFSET_TABLE
/// this has the side effect of changing the values of *_offset() to their actual addresses
/// but we can't call `rip_end()` after `mprotect` call anyway
pub unsafe fn patch_got_offsets(ptr: *mut c_void) {
    let offset = got_offset();
    let len = epilogue_offset() - offset;
    let got_addr = ptr.byte_add(offset) as *mut usize; // this cast is important, for the call to the usize `add()` later

    let count = len / core::mem::size_of::<usize>();

    for i in 0..count {
        let value = got_addr.add(i);
        *value += ptr as usize;
    }
}

pub unsafe fn initialize() {
    let mut local_inst = Instance::new();

    local_inst.base.ptr = rip_start();
    let stardust_len = rip_end() as usize - local_inst.base.ptr as usize;
    local_inst.base.len = stardust_len;

    unsafe {
        os::linux::rw_page(local_inst.base.ptr);
        os::linux::patch_got_offsets(local_inst.base.ptr)
    }

    // Find libc to demonstrate calling std library
    local_inst.libc.base_addr = unsafe { resolve_module(djb2_hash!(b"libc")) };

    if let Some(libc_base_addr) = local_inst.libc.base_addr {
        if let Some(write_fn_addr) =
            unsafe { resolve_function(libc_base_addr, djb2_hash!(b"write")) }
        {
            local_inst.libc.write = Some(unsafe { core::mem::transmute(write_fn_addr) });
        }
    }

    // Allocate instance on Heap
    let new_ptr = Box::into_raw(Box::new(local_inst));
    // Store heap instance pointer in global var
    INSTANCE.store(new_ptr, Ordering::Release);
}
