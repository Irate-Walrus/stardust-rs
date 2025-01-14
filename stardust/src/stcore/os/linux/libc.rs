use core::ffi::c_void;

pub struct Libc {
    pub base_addr: Option<*const c_void>,
    pub write: Option<unsafe extern "C" fn(isize, *const u8, usize) -> isize>,
}

impl Libc {
    pub fn new() -> Self {
        Self {
            base_addr: None,
            write: None,
        }
    }
}
