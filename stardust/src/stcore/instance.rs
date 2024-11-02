use alloc::boxed::Box;
use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

/// Constant to find the instance in memory with GDB
pub const INSTANCE_MAGIC: u32 = 0xDEADBEEF;

pub struct Instance {
    pub magic: u32,
    pub base: Base,
    #[cfg(target_os = "linux")]
    pub libc: super::os::linux::libc::Libc,
}

pub struct Base {
    pub ptr: *const usize,
    pub len: usize,
}

impl Instance {
    pub fn new() -> Self {
        Instance {
            magic: INSTANCE_MAGIC,
            base: Base {
                ptr: 0x0 as *const usize,
                len: 0x0,
            },
            #[cfg(target_os = "linux")]
            libc: super::os::linux::libc::Libc::new(),
        }
    }
}

static INSTANCE: AtomicPtr<Instance> = AtomicPtr::new(ptr::null_mut());

/// Get or create new stardust instance in memory.
/// This feels very wrong and is definitely not thread-safe.
pub fn init(instance: Instance) -> &'static mut Instance {
    // Try to load the instance
    let ptr = INSTANCE.load(Ordering::Acquire);

    // If the instance is not initialized, we create it
    if ptr.is_null() {
        let instance = Box::new(instance);

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

    // Return the initialized instance
    unsafe { &mut *INSTANCE.load(Ordering::Acquire) }
}
