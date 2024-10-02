/// Sets the first `n` bytes of the block of memory pointed to by `s`
/// to the specified value `c` (interpreted as an unsigned char).
///
/// # Parameters
/// - `s`: A pointer to the block of memory to fill.
/// - `c`: The value to be set. Only the lower 8 bits of `c` are used.
/// - `n`: The number of bytes to be set to the value.
///
/// # Returns
/// A pointer to the memory area `s`.
#[no_mangle]
pub extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    for i in 0..n {
        unsafe { *s.add(i) = c as u8 };
    }
    s
}

/// Copies `n` bytes from memory area `src` to memory area `dest`.
/// The memory areas must not overlap.
///
/// # Parameters
/// - `dest`: A pointer to the destination array where the content is to be copied.
/// - `src`: A pointer to the source of data to be copied.
/// - `n`: The number of bytes to copy.
///
/// # Returns
/// A pointer to the destination `dest`.
#[no_mangle]
pub extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    for i in 0..n {
        unsafe {
            *dest.add(i) = *src.add(i);
        }
    }
    dest
}

/// Copies `n` bytes from memory area `src` to memory area `dest`.
/// The memory areas may overlap.
///
/// # Parameters
/// - `dest`: A pointer to the destination array where the content is to be copied.
/// - `src`: A pointer to the source of data to be copied.
/// - `n`: The number of bytes to copy.
///
/// # Returns
/// A pointer to the destination `dest`.
#[no_mangle]
pub extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if src < dest as *const u8 {
        for i in (0..n).rev() {
            unsafe {
                *dest.add(i) = *src.add(i);
            }
        }
    } else {
        for i in 0..n {
            unsafe {
                *dest.add(i) = *src.add(i);
            }
        }
    }
    dest
}

/// Compares the first `n` bytes of the memory areas `s1` and `s2`.
///
/// # Parameters
/// - `s1`: A pointer to the first memory area.
/// - `s2`: A pointer to the second memory area.
/// - `n`: The number of bytes to compare.
///
/// # Returns
/// An integer less than, equal to, or greater than zero if the first `n` bytes of `s1`
/// is found, respectively, to be less than, to match, or be greater than the first `n` bytes of `s2`.
#[no_mangle]
pub extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let a = unsafe { *s1.add(i) };
        let b = unsafe { *s2.add(i) };
        if a != b {
            return a as i32 - b as i32;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn bcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    memcmp(s1, s2, n)
}
