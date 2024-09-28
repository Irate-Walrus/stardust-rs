use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;
use syscalls::{syscall, Sysno};

/*
#[no_mangle]
#[link_section = ".data"]
static mut __rust_no_alloc_shim_is_unstable: u8 = 0;

extern "C" {
    #[cfg_attr(
        target_arch = "x86_64",
        link_name = "\x01.obj.__rust_no_alloc_shim_is_unstable"
    )]
    // again we have to declare this as a function to prevent another .refptr symbol from being generated
    fn __refptr__rust_no_alloc_shim_is_unstable();
}
*/

const MAP_ANONYMOUS: u8 = 0x20;
const MAP_PRIVATE: u8 = 0x02;
const PROT_READ: u8 = 0x01;
const PROT_WRITE: u8 = 0x02;

pub struct StardustAllocator;

unsafe impl GlobalAlloc for StardustAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // void *mmap(
        //      void *addr,
        //      size_t length,
        //      int prot,
        //      int flags,
        //      int fd,
        //      off_t offset
        // );
        let result = syscall!(
            Sysno::mmap,
            0x0,
            layout.size(),
            (PROT_READ | PROT_WRITE),
            (MAP_PRIVATE | MAP_ANONYMOUS),
            u64::MAX,
            0
        );
        result.unwrap() as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // int munmap(void *addr, size_t len);
        let result = syscall!(Sysno::munmap, ptr, layout.size());
        result.unwrap();
    }
}

#[no_mangle]
unsafe fn rust_oom() -> ! {
    asm!("ud2", options(noreturn));
}
