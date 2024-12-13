mod allocator;
pub mod kernel32;
mod loadlib;
pub mod ntdll;

pub use allocator::StWindowsAllocator;
pub use loadlib::{ldr_function, ldr_module};

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ffi::c_void;
use core::sync::atomic::Ordering;

use crate::{stcore::*, ALLOCATOR};

/// Log &str to stdout
pub fn log_str(s: &str) {
    if let Some(instance) = Instance::get() {
        let mut buf: Vec<u16> = s.encode_utf16().collect();
        buf.push(0); // Add null terminator
        unsafe {
            let mut bytes_written: u32 = 0;
            (instance.kernel32.write_file)(
                -11i32 as u32 as *mut c_void,
                s.as_ptr() as *const c_void,
                s.len() as u32,
                &mut bytes_written,
                core::ptr::null_mut(),
            );
        }
    }
}

#[cfg(target_arch = "x86")]
pub unsafe fn patch_got_offsets(ptr: *mut c_void) {
    let offset = got_offset() - 1; // I don't know why this off-by-one error exists, but it does.
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

    // Load the base address of kernel32.dll.
    local_inst.kernel32.base_addr = ldr_module(djb2_hash!(b"KERNEL32.DLL"));

    let output_debug_string_a_addr = ldr_function(
        local_inst.kernel32.base_addr,
        djb2_hash!(b"OutputDebugStringA"),
    );
    local_inst.kernel32.output_debug_string_a = core::mem::transmute(output_debug_string_a_addr);

    let output_debug_string_w_addr = ldr_function(
        local_inst.kernel32.base_addr,
        djb2_hash!(b"OutputDebugStringW"),
    );
    local_inst.kernel32.output_debug_string_w = core::mem::transmute(output_debug_string_w_addr);

    let write_file_addr = ldr_function(local_inst.kernel32.base_addr, djb2_hash!(b"WriteFile"));
    local_inst.kernel32.write_file = core::mem::transmute(write_file_addr);

    // Load the base address of ntdll.dll.
    local_inst.ntdll.base_addr = ldr_module(djb2_hash!(b"ntdll.dll"));

    // Resolve RtlCreateHeap
    let rtl_create_heap_addr =
        ldr_function(local_inst.ntdll.base_addr, djb2_hash!(b"RtlCreateHeap"));
    local_inst.ntdll.rtl_create_heap = core::mem::transmute(rtl_create_heap_addr);

    // Resolve RtlAllocateHeap
    let rtl_allocate_heap_addr =
        ldr_function(local_inst.ntdll.base_addr, djb2_hash!(b"RtlAllocateHeap"));
    local_inst.ntdll.rtl_allocate_heap = core::mem::transmute(rtl_allocate_heap_addr);

    // Resolve RtlFreeHeap
    let rtl_free_heap_addr = ldr_function(local_inst.ntdll.base_addr, djb2_hash!(b"RtlFreeHeap"));
    local_inst.ntdll.rtl_free_heap = core::mem::transmute(rtl_free_heap_addr);

    // Resolve RtlReAllocateHeap
    let rtl_reallocate_heap_addr =
        ldr_function(local_inst.ntdll.base_addr, djb2_hash!(b"RtlReAllocateHeap"));
    local_inst.ntdll.rtl_re_allocate_heap = core::mem::transmute(rtl_reallocate_heap_addr);

    // Resolve RtlDestroyHeap
    let rtl_destroy_heap_addr =
        ldr_function(local_inst.ntdll.base_addr, djb2_hash!(b"RtlDestroyHeap"));
    local_inst.ntdll.rtl_destroy_heap = core::mem::transmute(rtl_destroy_heap_addr);

    // Resolve NtTerminateProcess
    let nt_terminate_process_addr = ldr_function(
        local_inst.ntdll.base_addr,
        djb2_hash!(b"NtTerminateProcess"),
    );
    local_inst.ntdll.nt_terminate_process = core::mem::transmute(nt_terminate_process_addr);

    // Resolve NtProtectVirtualMemory
    let nt_terminate_process_addr = ldr_function(
        local_inst.ntdll.base_addr,
        djb2_hash!(b"NtProtectVirtualMemory"),
    );
    local_inst.ntdll.nt_protect_virtual_memory = core::mem::transmute(nt_terminate_process_addr);

    let mut ptr = local_inst.base.ptr.byte_add(data_offset()) as *mut c_void;

    let mut size = size_of::<usize>() as _;
    let mut protect = 0x0;
    (local_inst.ntdll.nt_protect_virtual_memory)(
        usize::MAX as *mut c_void, // HANDLE-1 OR NtCurrentProcess()
        &mut ptr,
        &mut size, // will round up to full page
        0x04,      //PAGE_READWRITE
        &mut protect,
    );

    #[cfg(target_arch = "x86")]
    os::windows::patch_got_offsets(local_inst.base.ptr);

    // Get the address of the instance on the stack
    let instance_stack_addr = &mut local_inst as *mut Instance;

    // Store instance stack pointer as the global instance \(0.0)/
    INSTANCE.store(instance_stack_addr, Ordering::Release);

    // Initialize Allocator with the "global" stack instance
    ALLOCATOR.initialize();

    // Allocate instance on heap using initialized allocator
    let new_ptr = Box::into_raw(Box::new(local_inst));
    INSTANCE.store(new_ptr, Ordering::Release);
}
