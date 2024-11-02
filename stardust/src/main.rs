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

pub mod stcore;

use stcore::instance::init;
use stcore::instance::Instance;
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

    let mut local_inst = Instance::new();

    local_inst.base.ptr = rip_start();
    let stardust_len = rip_end() as usize - local_inst.base.ptr as usize;
    local_inst.base.len = stardust_len;

    info_addr!("Stardust Start Address", local_inst.base.ptr);
    info_addr!(
        "Stardust End Address",
        local_inst.base.ptr as usize + local_inst.base.len
    );
    info_int!("Stardust Length", local_inst.base.len);

    let data_offset = data_offset();
    info_addr!("Stardust Data Offset", data_offset);

    let data_addr = unsafe { local_inst.base.ptr.add(data_offset / size_of::<usize>()) };

    info_addr!("Stardust Data Address", data_addr);

    #[cfg(target_os = "linux")]
    {
        let got_offset = got_offset() - 1; // I don't know why this off-by-one error exists, but it does.
        info_addr!("Stardust GOT Offset", got_offset);

        let got_addr = unsafe { local_inst.base.ptr.add(got_offset / size_of::<usize>()) };
        info_addr!("Stardust GOT Address", got_addr);

        let got_len = epilogue_offset() - got_offset;
        info_int!("Stardust GOT Length", got_len);

        unsafe {
            os::linux::rw_page(local_inst.base.ptr);
            os::linux::patch_got_offsets(local_inst.base.ptr)
        }
    }

    let global_instance = init(local_inst);

    info_addr!("Stardust Instance", global_instance as *const Instance);

    // Test to ensure that memcpy from `compiler_builtins` is working
    let src = alloc::string::String::from("SSECCUS\t\t:ypcmem gnitseT");
    let dst: String = src.chars().rev().collect();
    info!(&dst);

    #[cfg(target_os = "linux")]
    {
        use stcore::os::linux::{find_fn_in_lib, find_lib};
        // Find libc to demonstrate calling std library
        global_instance.libc.base_addr = unsafe { find_lib(djb2_hash!(b"libc"), 4) };

        if let Some(libc_base_addr) = global_instance.libc.base_addr {
            info_addr!("Library Base Address", libc_base_addr as usize);

            if let Some(write_fn_addr) =
                unsafe { find_fn_in_lib(libc_base_addr, djb2_hash!(b"write")) }
            {
                global_instance.libc.write = Some(unsafe { core::mem::transmute(write_fn_addr) });
            }
        }

        if let Some(write_fn) = global_instance.libc.write {
            let msg = b"[*] Hello, world from write!\n";
            unsafe { write_fn(1, msg.as_ptr(), msg.len()) };
        }
    }

    info!("HITTING BREAKPOINT");
    unsafe {
        intrinsics::breakpoint();
    }
}
