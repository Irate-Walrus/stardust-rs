use core::ffi::c_void;
use core::ptr::null_mut;

pub struct Ntdll {
    pub base_addr: *mut c_void,
    pub rtl_create_heap: RtlCreateHeap,
    pub rtl_allocate_heap: RtlAllocateHeap,
    pub rtl_free_heap: RtlFreeHeap,
    pub rtl_re_allocate_heap: RtlReAllocateHeap,
    pub rtl_destroy_heap: RtlDestroyHeap,
    pub nt_terminate_process: NtTerminateProcess,
    pub nt_protect_virtual_memory: NtProtectVirtualMemory,
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

pub type RtlCreateHeap = unsafe extern "system" fn(
    Flags: u32,
    HeapBase: *mut u8,
    ReserveSize: usize,
    CommitSize: usize,
    Lock: *mut u8,
    Parameters: *mut u8,
) -> *mut c_void;

pub type RtlAllocateHeap =
    unsafe extern "system" fn(HeapHandle: *mut c_void, Flags: u32, Size: usize) -> *mut u8;

pub type RtlFreeHeap =
    unsafe extern "system" fn(HeapHandle: *mut c_void, Flags: u32, Size: *mut u8) -> i32;

pub type RtlReAllocateHeap = unsafe extern "system" fn(
    HeapHandle: *mut c_void,
    Flags: u32,
    MemoryPointer: *mut u8,
    Size: usize,
) -> *mut u8;

pub type RtlDestroyHeap = unsafe extern "system" fn(HeapHandle: *mut c_void) -> *mut c_void;

pub type NtTerminateProcess =
    unsafe extern "system" fn(ProcessHandle: *mut c_void, ExitStatus: i32) -> i32;

pub type NtProtectVirtualMemory = unsafe extern "system" fn(
    ProcessHandle: *mut c_void,    //HANDLE
    BaseAddress: &*mut c_void,     //PVOID
    RegionSize: *mut usize,        //PSIZE_T
    NewAccessProtection: u32,      //ULONG
    OldAccessProtection: *mut u32, //PULONG
) -> u32;
