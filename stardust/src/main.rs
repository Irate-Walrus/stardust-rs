#![no_std]
#![no_main]
/* Required to use `core::intrinisics` */
#![allow(internal_features)]
#![feature(core_intrinsics)]
use core::intrinsics;

extern crate alloc;

#[macro_use]
extern crate djb2_macro;

//use alloc::string::String;
//use core::str;

pub mod stcore;

use stcore::*;

define_djb2_hash_fn!(rt_djb2_hash);

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
pub extern "C" fn stmain() {
    unsafe { stcore::initialize() };
    info!("Hello Stardust!");

    let instance = Instance::get().unwrap();

    info_addr!("Stardust Start Address", instance.base.ptr);
    info_int!("Stardust Length", instance.base.len);
    info_addr!("Stardust Instance", instance as *mut Instance);

    /*
        Your code here?
    */

    info!("Hitting Breakpoint!");
    unsafe {
        intrinsics::breakpoint();
    }
}
