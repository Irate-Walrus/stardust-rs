use std::{mem, ptr};

const SHELLCODE: &[u8] = include_bytes!("../../target/stardust.bin");

#[cfg(target_arch = "x86_64")]
const SHELLCODE_ADDR: usize = 0x700000000000;

#[cfg(target_arch = "x86")]
const SHELLCODE_ADDR: usize = 0x70000000;

#[cfg(target_arch = "x86_64")]
const STARDUST_BANNER: &str = "\n***\t[STARDUST x86_64]\t***";

#[cfg(target_arch = "x86")]
const STARDUST_BANNER: &str = "\n***\t[STARDUST i686]\t***";

fn main() {
    println!("***\t[LOADER]\t***");
    println!("[*] Allocate RW Memory");
    let buffer_ptr = alloc_rw();

    println!("[*] Copy Shellcode Into RW Memory");
    unsafe {
        ptr::copy_nonoverlapping(SHELLCODE.as_ptr(), buffer_ptr as *mut u8, SHELLCODE.len());
    }

    println!("[*] Set Memory RX");
    set_rx(buffer_ptr);

    println!("[*] Allocation Start Address:\t0x{:x}", buffer_ptr as usize);
    println!(
        "[*] Allocation End Address:\t0x{:x}",
        buffer_ptr as usize + SHELLCODE.len()
    );

    println!("[*] Allocation Size:\t\t{}B", SHELLCODE.len());

    println!("{}", STARDUST_BANNER);
    let exec: extern "C" fn() -> ! = unsafe { mem::transmute(buffer_ptr) };
    exec();
}

#[cfg(target_os = "linux")]
fn alloc_rw() -> *mut usize {
    use libc::{mmap, MAP_ANONYMOUS, MAP_PRIVATE, PROT_READ, PROT_WRITE};
    use std::ffi::c_void;

    let buffer_ptr = unsafe {
        mmap(
            SHELLCODE_ADDR as *mut c_void, //SHELLCODE_ADDR as *mut c_void,
            SHELLCODE.len(),
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS,
            -1,
            0,
        )
    };

    if buffer_ptr == libc::MAP_FAILED {
        panic!("RW allocation failed");
    }

    buffer_ptr as *mut usize
}

#[cfg(target_os = "linux")]
fn set_rx(ptr: *mut usize) {
    use std::ffi::c_void;

    use libc::{mprotect, PROT_EXEC, PROT_READ};

    let res = unsafe { mprotect(ptr as *mut c_void, SHELLCODE.len(), PROT_READ | PROT_EXEC) };

    if res != 0 {
        panic!("set allocation RX failed");
    }
}

#[cfg(target_os = "windows")]
fn alloc_rw() -> *mut usize {
    use winapi::um::memoryapi::VirtualAlloc;
    use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE};

    let buffer_ptr = unsafe {
        VirtualAlloc(
            SHELLCODE_ADDR as *mut _, // Let the system choose the address
            SHELLCODE.len(),          // Number of bytes to allocate
            MEM_COMMIT | MEM_RESERVE, // Allocate and reserve pages
            PAGE_READWRITE,           // Read-write protection
        ) as *mut usize
    };

    if buffer_ptr as usize == 0x0 {
        panic!("RW allocation failed");
    }

    buffer_ptr
}

#[cfg(target_os = "windows")]
fn set_rx(ptr: *mut usize) {
    use winapi::um::memoryapi::VirtualProtect;
    use winapi::um::winnt::PAGE_EXECUTE_READ;
    let mut old_protection = 0;
    let res = unsafe {
        VirtualProtect(
            ptr as *mut _,
            SHELLCODE.len(),
            PAGE_EXECUTE_READ,
            &mut old_protection,
        )
    };

    if res == 0x0 {
        panic!("set allocation RX failed");
    }
}
