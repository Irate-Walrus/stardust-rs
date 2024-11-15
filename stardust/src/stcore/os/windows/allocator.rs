use core::{
    alloc::{GlobalAlloc, Layout},
    arch::asm,
    ffi::{c_ulong, c_void},
};

use crate::stcore::instance::Instance;

pub const HEAP_GROWABLE: c_ulong = 0x00000002;
pub const HEAP_ZERO_MEMORY: c_ulong = 0x00000008;

/// Struct representing a custom heap allocator using the NT Heap API.
pub struct StWindowsAllocator;

impl StWindowsAllocator {
    /// Retrieves the raw handle to the heap managed by this allocator.
    /// This function fetches the heap handle from the global instance.
    #[inline]
    fn handle(&self) -> *mut c_void {
        let instance = Instance::get().unwrap();
        instance.heap_handle as *mut c_void
    }

    /// Initializes the heap by calling `RtlCreateHeap` and storing the resulting handle.
    /// This function uses the global instance to set the heap handle.
    #[inline]
    pub fn initialize(&self) {
        let instance = Instance::get().unwrap();

        let raw_heap_handle = unsafe {
            (instance.ntdll.rtl_create_heap)(
                HEAP_GROWABLE,
                core::ptr::null_mut(),
                0,
                0,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            )
        };
        instance.heap_handle = raw_heap_handle;
    }
}

/// Implementation of the `GlobalAlloc` trait for ` StWindowsAllocator`,
/// using the NT Heap API for memory management.
unsafe impl GlobalAlloc for StWindowsAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let instance = Instance::get().unwrap();
        (instance.ntdll.rtl_allocate_heap)(self.handle(), 0, layout.size())
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let instance = Instance::get().unwrap();
        (instance.ntdll.rtl_allocate_heap)(self.handle(), HEAP_ZERO_MEMORY, layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let instance = Instance::get().unwrap();
        (instance.ntdll.rtl_free_heap)(self.handle(), 0, ptr);
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        let instance = Instance::get().unwrap();
        (instance.ntdll.rtl_re_allocate_heap)(self.handle(), 0, ptr, new_size)
    }
}

#[no_mangle]
unsafe fn rust_oom() -> ! {
    asm!("ud2", options(noreturn));
}
