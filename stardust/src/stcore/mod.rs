pub mod arch;
pub mod instance;
pub mod macros;
pub mod os;

pub use arch::{rip_end, rip_start};

pub use os::{log_str, StardustAllocator};

extern "C" {
    static mut _data_offset: usize;
    static mut _got_offset: usize;
    static mut _epilogue_offset: usize;
}

pub fn data_offset() -> usize {
    #[allow(unused_unsafe)]
    unsafe {
        let offset_addr = &raw mut _data_offset;
        offset_addr as usize
    }
}

pub fn got_offset() -> usize {
    #[allow(unused_unsafe)]
    unsafe {
        let offset_addr = &raw mut _got_offset;
        offset_addr as usize
    }
}

pub fn epilogue_offset() -> usize {
    #[allow(unused_unsafe)]
    unsafe {
        let offset_addr = &raw mut _epilogue_offset;
        offset_addr as usize
    }
}
