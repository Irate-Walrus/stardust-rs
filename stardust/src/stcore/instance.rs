use alloc::boxed::Box;
use core::ffi::c_void;
use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

/// Constant to find the instance in memory with GDB
pub const INSTANCE_MAGIC: u32 = 0xDEADBEEF;

pub struct Instance {
    pub magic: u32,
    pub base: Base,
    #[cfg(feature = "linux")]
    pub libc: super::os::linux::libc::Libc,
    #[cfg(feature = "windows")]
    pub ntdll: super::os::windows::ntdll::Ntdll,
    #[cfg(feature = "windows")]
    pub kernel32: super::os::windows::kernel32::Kernel32,
    #[cfg(feature = "windows")]
    pub heap_handle: *mut c_void,
}

pub struct Base {
    pub ptr: *mut c_void,
    pub len: usize,
}

impl Instance {
    pub fn new() -> Self {
        Instance {
            magic: INSTANCE_MAGIC,
            base: Base {
                ptr: 0x0 as *mut c_void,
                len: 0x0,
            },
            #[cfg(feature = "linux")]
            libc: super::os::linux::libc::Libc::new(),
            #[cfg(feature = "windows")]
            ntdll: super::os::windows::ntdll::Ntdll::new(),
            #[cfg(feature = "windows")]
            kernel32: super::os::windows::kernel32::Kernel32::new(),
            #[cfg(feature = "windows")]
            heap_handle: ptr::null_mut(),
        }
    }

    /// Initializes the global INSTANCE with a provided stack-initialized instance.
    /// This should only be called once.
    pub fn from_local(local_instance: Instance) {
        let ptr = INSTANCE.load(Ordering::Acquire);

        // If the instance is not initialized, we create it
        if ptr.is_null() {
            let instance = Box::new(local_instance);

            // Convert Box to raw pointer and attempt to set it atomically
            let new_ptr = Box::into_raw(instance);
            match INSTANCE.compare_exchange(ptr, new_ptr, Ordering::AcqRel, Ordering::Acquire) {
                Ok(_) => {} // Successfully set the new pointer
                Err(_) => {
                    // If there was an existing instance, we need to drop the new one
                    // We must convert the raw pointer into Box so that it is dropped
                    unsafe {
                        drop(Box::from_raw(new_ptr));
                    }
                }
            }
        }
    }

    pub fn get() -> Option<&'static mut Self> {
        let ptr = INSTANCE.load(Ordering::Acquire);

        if ptr.is_null() {
            return None;
        }

        unsafe { Some(&mut *ptr) }
    }
}

pub static INSTANCE: AtomicPtr<Instance> = AtomicPtr::new(ptr::null_mut());
