use core::ffi::c_void;
use phnt::ffi::{
    NtProtectVirtualMemoryFn, NtTerminateProcessFn, RtlAllocateHeapFn, RtlCreateHeapFn,
    RtlDestroyHeapFn, RtlFreeHeapFn, RtlReAllocateHeapFn,
};

pub struct Ntdll {
    pub base_addr: *mut c_void,
    pub rtl_create_heap: *mut c_void,
    pub rtl_allocate_heap: *mut c_void,
    pub rtl_free_heap: *mut c_void,
    pub rtl_re_allocate_heap: *mut c_void,
    pub rtl_destroy_heap: *mut c_void,
    pub nt_terminate_process: *mut c_void,
    pub nt_protect_virtual_memory: *mut c_void,
}

impl Ntdll {
    pub fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    #[inline(always)]
    pub unsafe fn rtl_create_heap(&self) -> RtlCreateHeapFn {
        core::mem::transmute(self.rtl_create_heap)
    }

    #[inline(always)]
    pub unsafe fn rtl_allocate_heap(&self) -> RtlAllocateHeapFn {
        core::mem::transmute(self.rtl_allocate_heap)
    }

    #[inline(always)]
    pub unsafe fn rtl_free_heap(&self) -> RtlFreeHeapFn {
        core::mem::transmute(self.rtl_free_heap)
    }

    #[inline(always)]
    pub unsafe fn rtl_re_allocate_heap(&self) -> RtlReAllocateHeapFn {
        core::mem::transmute(self.rtl_re_allocate_heap)
    }

    #[inline(always)]
    pub unsafe fn rtl_destroy_heap(&self) -> RtlDestroyHeapFn {
        core::mem::transmute(self.rtl_destroy_heap)
    }

    #[inline(always)]
    pub unsafe fn nt_terminate_process(&self) -> NtTerminateProcessFn {
        core::mem::transmute(self.nt_terminate_process)
    }

    #[inline(always)]
    pub unsafe fn nt_protect_virtual_memory(&self) -> NtProtectVirtualMemoryFn {
        core::mem::transmute(self.nt_protect_virtual_memory)
    }
}
