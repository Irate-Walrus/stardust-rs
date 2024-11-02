use alloc::boxed::Box;
use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

/// Constant to find the instance in memory with GDB
pub const INSTANCE_MAGIC: u32 = 0xDEADBEEF;

pub struct Instance {
    pub magic: u32,
    pub libc: Libc,
}

pub struct Libc {
    pub base_addr: Option<*const usize>,
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

static INSTANCE: AtomicPtr<Instance> = AtomicPtr::new(ptr::null_mut());

/// Get or create new stardust instance in memory.
/// This feels very wrong and is definitely not thread-safe.
pub fn instance() -> &'static mut Instance {
    // Try to load the instance
    let ptr = INSTANCE.load(Ordering::Acquire);

    // If the instance is not initialized, we create it
    if ptr.is_null() {
        let instance = Box::new(Instance {
            // Initialize your instance fields here
            magic: INSTANCE_MAGIC,
            libc: Libc::new(),
        });

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
