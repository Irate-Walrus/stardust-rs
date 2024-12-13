use core::ffi::c_void;
use core::ptr::null_mut;

pub struct Kernel32 {
    pub base_addr: *mut c_void,
    pub output_debug_string_a: OutputDebugStringA,
    pub output_debug_string_w: OutputDebugStringW,
    pub write_file: WriteFile,
}

impl Kernel32 {
    pub fn new() -> Self {
        Kernel32 {
            base_addr: null_mut(),
            output_debug_string_a: unsafe { core::mem::transmute(null_mut::<c_void>()) },
            output_debug_string_w: unsafe { core::mem::transmute(null_mut::<c_void>()) },
            write_file: unsafe { core::mem::transmute(null_mut::<c_void>()) },
        }
    }
}

#[cfg(target_arch = "x86")]
type OutputDebugStringA = unsafe extern "stdcall" fn(lpOutputString: *const u8);

#[cfg(target_arch = "x86")]
type OutputDebugStringW = unsafe extern "stdcall" fn(lpOutputString: *const u16);

#[cfg(target_arch = "x86")]
pub type WriteFile = unsafe extern "stdcall" fn(
    hFile: *mut c_void,
    lpBuffer: *const c_void,
    nNumberOfBytesToWrite: u32,
    lpNumberOfBytesWritten: *mut u32,
    lpOverlapped: *mut c_void,
) -> i32;

#[cfg(target_arch = "x86_64")]
type OutputDebugStringA = unsafe extern "win64" fn(lpOutputString: *const u8);

#[cfg(target_arch = "x86_64")]
type OutputDebugStringW = unsafe extern "win64" fn(lpOutputString: *const u16);

#[cfg(target_arch = "x86_64")]
pub type WriteFile = unsafe extern "win64" fn(
    hFile: *mut c_void,
    lpBuffer: *const c_void,
    nNumberOfBytesToWrite: u32,
    lpNumberOfBytesWritten: *mut u32,
    lpOverlapped: *mut c_void,
) -> i32;
