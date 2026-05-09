use core::ffi::c_void;

pub struct Kernel32 {
    pub base_addr: *mut c_void,
    pub output_debug_string_a: *mut c_void,
    pub output_debug_string_w: *mut c_void,
    pub write_file: *mut c_void,
}

impl Kernel32 {
    pub fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    #[inline(always)]
    pub unsafe fn output_debug_string_a(&self) -> OutputDebugStringA {
        core::mem::transmute(self.output_debug_string_a)
    }

    #[inline(always)]
    pub unsafe fn output_debug_string_w(&self) -> OutputDebugStringW {
        core::mem::transmute(self.output_debug_string_w)
    }

    #[inline(always)]
    pub unsafe fn write_file(&self) -> WriteFile {
        core::mem::transmute(self.write_file)
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
