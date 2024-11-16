use core::ffi::c_void;
use core::ptr::null_mut;
use phnt::ffi::{
    NtProtectVirtualMemoryType, NtTerminateProcessType, RtlAllocateHeapType, RtlCreateHeapType,
    RtlDestroyHeapType, RtlFreeHeapType, RtlReAllocateHeapType,
};

pub struct Ntdll {
    pub base_addr: *mut c_void,
    pub rtl_create_heap: RtlCreateHeapType,
    pub rtl_allocate_heap: RtlAllocateHeapType,
    pub rtl_free_heap: RtlFreeHeapType,
    pub rtl_re_allocate_heap: RtlReAllocateHeapType,
    pub rtl_destroy_heap: RtlDestroyHeapType,
    pub nt_terminate_process: NtTerminateProcessType,
    pub nt_protect_virtual_memory: NtProtectVirtualMemoryType,
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
