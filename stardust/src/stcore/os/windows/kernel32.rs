use core::ffi::c_void;
use core::ptr::null_mut;

pub struct Kernel32 {
    pub base_addr: *const usize,
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

type OutputDebugStringA = unsafe extern "system" fn(lpOutputString: *const u8);
type OutputDebugStringW = unsafe extern "system" fn(lpOutputString: *const u16);
pub type WriteFile = unsafe extern "system" fn(
    hFile: *mut c_void,
    lpBuffer: *const c_void,
    nNumberOfBytesToWrite: u32,
    lpNumberOfBytesWritten: *mut u32,
    lpOverlapped: *mut c_void,
) -> i32;
