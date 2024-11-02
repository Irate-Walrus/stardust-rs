#![no_std]
#![no_main]
/* Required to use `core::intrinisics` */
#![allow(internal_features)]
#![feature(core_intrinsics)]
use core::intrinsics;

extern crate alloc;

#[macro_use]
extern crate djb2_macro;

use alloc::string::String;
use core::str;

use syscalls::{syscall, Sysno};

pub mod stcore;

use stcore::instance::instance;
use stcore::instance::Instance;
use stcore::os::linux::{find_fn_in_lib, find_lib};
use stcore::*;

/* These workarounds are required to compile if `alloc::format!` macro is used. */
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
    intrinsics::abort()
}

#[global_allocator]
pub static ALLOCATOR: StardustAllocator = StardustAllocator;

#[link_section = ".text.implant"]
#[no_mangle]
pub extern "C" fn main() {
    info!("Hello Stardust!");

    let stardust_start = rip_start();
    let stardust_end = rip_end();
    let stardust_len = stardust_end as usize - stardust_start as usize;

    info_addr!("Stardust Start Address", stardust_start);
    info_addr!("Stardust End Address", stardust_end);
    info_int!("Stardust Length", stardust_len);

    let data_offset = data_offset();
    info_addr!("Stardust Data Offset", data_offset);

    let data_addr = unsafe { stardust_start.add(data_offset / size_of::<usize>()) };
    info_addr!("Stardust Data Address", data_addr);

    let got_offset = got_offset() - 1; // I don't know why this off-by-one error exists, but it does.
    info_addr!("Stardust GOT Offset", got_offset);

    let got_addr = unsafe { stardust_start.add(got_offset / size_of::<usize>()) };
    info_addr!("Stardust GOT Address", got_addr);

    let got_len = epilogue_offset() - got_offset;
    info_int!("Stardust GOT Length", got_len);

    // Set data, bss, and got page to RW
    // really this only protects `size_of::<usize>()` but it'll flip the entire page
    // including `rip_end()`, so don't call that again
    unsafe {
        let _ = syscall!(Sysno::mprotect, data_addr, size_of::<usize>(), 0x1 | 0x2);
    }

    // Patch hardcoded memory addresses in the GLOBAL_OFFSET_TABLE
    // this has the side effect of changing the values of *_offset() to their actual addresses
    // but we can't call `rip_end()` after `mprotect` call anyway
    unsafe {
        let count = got_len / core::mem::size_of::<usize>();

        for i in 0..count {
            let value = got_addr.add(i);
            *value += stardust_start as usize;
        }
    }

    let instance = instance();
    let instance_ptr = instance as *const Instance;
    info_addr!("Stardust Instance", instance_ptr);

    // a test to ensure that memcpy from `compiler_builtins` is working
    let src = alloc::string::String::from("SSECCUS\t\t:ypcmem gnitseT");
    let dst: String = src.chars().rev().collect();
    info!(&dst);

    // Find libc to demonstrate calling std library
    instance.libc.base_addr = unsafe { find_lib(djb2_hash!(b"libc"), 4) };

    if let Some(libc_base_addr) = instance.libc.base_addr {
        info_addr!("Library Base Address", libc_base_addr as usize);

        if let Some(write_fn_addr) = unsafe { find_fn_in_lib(libc_base_addr, djb2_hash!(b"write")) }
        {
            instance.libc.write = Some(unsafe { core::mem::transmute(write_fn_addr) });
        }
    }

    if let Some(write_fn) = instance.libc.write {
        let msg = b"[*] Hello, world from write!\n";
        unsafe { write_fn(1, msg.as_ptr(), msg.len()) };
    }

    info!("HITTING BREAKPOINT");
    unsafe {
        intrinsics::breakpoint();
    }

    exit(0);
}

fn exit(code: u8) {
    unsafe {
        _ = syscall!(Sysno::exit, code);
    }
}
