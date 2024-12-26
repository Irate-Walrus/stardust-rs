use core::ffi::c_void;
use core::ptr::null_mut;
use phnt::ffi::{
    NtProtectVirtualMemoryFn, NtTerminateProcessFn, RtlAllocateHeapFn, RtlCreateHeapFn,
    RtlDestroyHeapFn, RtlFreeHeapFn, RtlReAllocateHeapFn,
};

pub struct Ntdll {
    pub base_addr: *mut c_void,
    pub rtl_create_heap: RtlCreateHeapFn,
    pub rtl_allocate_heap: RtlAllocateHeapFn,
    pub rtl_free_heap: RtlFreeHeapFn,
    pub rtl_re_allocate_heap: RtlReAllocateHeapFn,
    pub rtl_destroy_heap: RtlDestroyHeapFn,
    pub nt_terminate_process: NtTerminateProcessFn,
    pub nt_protect_virtual_memory: NtProtectVirtualMemoryFn,
}

impl Ntdll {
    pub fn new() -> Self {
        Ntdll {
            base_addr: null_mut(),
            rtl_create_heap: unsafe { core::mem::transmute(null_mut::<c_void>()) },
            rtl_allocate_heap: unsafe { core::mem::transmute(null_mut::<c_void>()) },
            rtl_free_heap: unsafe { core::mem::transmute(null_mut::<c_void>()) },
            rtl_re_allocate_heap: unsafe { core::mem::transmute(null_mut::<c_void>()) },
            rtl_destroy_heap: unsafe { core::mem::transmute(null_mut::<c_void>()) },
            nt_terminate_process: unsafe { core::mem::transmute(null_mut::<c_void>()) },
            nt_protect_virtual_memory: unsafe { core::mem::transmute(null_mut::<c_void>()) },
        }
    }
}
