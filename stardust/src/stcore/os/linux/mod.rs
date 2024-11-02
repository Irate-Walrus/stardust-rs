use syscalls::{syscall, Sysno};

mod allocator;
mod loadlib;

pub use allocator::StardustAllocator;
pub use loadlib::{find_fn_in_lib, find_lib};

/// Log &str to stdout
pub fn log_str(s: &str) {
    unsafe { write(0x1, s.as_ptr(), s.len()) }
}

/// Write to file descriptor using the `write` syscall
pub unsafe fn write(fd: usize, buf: *const u8, count: usize) {
    let _ = syscall!(Sysno::write, fd, buf, count);
}
