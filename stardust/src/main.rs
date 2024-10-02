#![no_std]
#![no_main]
#![allow(unused_imports)]

use syscalls::{syscall, Sysno};

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};

pub mod allocator;
pub mod prelude;

use allocator::StardustAllocator;
use prelude::*;

/// Workaround for rustc bug: https://github.com/rust-lang/rust/issues/47493
///
/// It shouldn't even be possible to reach this function, thanks to panic=abort,
/// but libcore is compiled with unwinding enabled and that ends up making unreachable
/// references to this.
#[no_mangle]
extern "C" fn rust_eh_personality() {
    unreachable!("Unwinding not supported");
}

/// Workaround for rustc bug: https://github.com/rust-lang/rust/issues/47493
///
/// It shouldn't even be possible to reach this function, thanks to panic=abort,
/// but libcore is compiled with unwinding enabled and that ends up making unreachable
/// references to this.
#[no_mangle]
extern "C" fn _Unwind_Resume() -> ! {
    unreachable!("Unwinding not supported");
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
#[link_section = ".text"]
pub static ALLOCATOR: StardustAllocator = StardustAllocator;

const STDOUT: usize = 0x01;

#[link_section = ".text.implant"]
#[no_mangle]
pub extern "C" fn main() {
    print("[+] Hello Stardust\n");

    let start = rip_start();
    let end = rip_end();
    let length = end as usize - start as usize;

    print(&format!("[*] Stardust Start Address:\t{:p}\n", start));
    print(&format!("[*] Stardust End Address:\t{:p}\n", end));
    print(&format!("[*] Stardust Length:\t\t{}B\n", length));

    exit(0);
}

fn print(s: &str) {
    unsafe {
        let _ = syscall!(Sysno::write, STDOUT, s.as_ptr(), s.len());
    }
}

fn exit(code: u8) {
    unsafe {
        _ = syscall!(Sysno::exit, code);
    }
}
