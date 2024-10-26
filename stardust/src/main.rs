#![no_std]
#![no_main]

extern crate alloc;

use core::str;
use syscalls::{syscall, Sysno};

pub mod allocator;
pub mod instance;
pub mod nocrt;
pub mod prelude;

use allocator::StardustAllocator;
use instance::instance;
use instance::Instance;
use prelude::*;

#[cfg(not(test))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
pub static ALLOCATOR: StardustAllocator = StardustAllocator;

#[link_section = ".text.implant"]
#[no_mangle]
pub extern "C" fn main() {
    print("[*] Hello Stardust!\n");

    let start = rip_start();
    let end = rip_end();
    let length = end as usize - start as usize;
    let offset = data_offset();

    print("[*] Stardust Start Address:\t");
    let hex_buf = usize_to_hex_str(start as usize);
    let hex_str: &str = unsafe { str::from_utf8_unchecked(&hex_buf) };
    print(&hex_str);
    print("\n");

    print("[*] Stardust End Address:\t");
    let hex_buf = usize_to_hex_str(end as usize);
    let hex_str: &str = unsafe { str::from_utf8_unchecked(&hex_buf) };
    print(&hex_str);
    print("\n");

    print("[*] Stardust Length:\t\t");
    let hex_buf = usize_to_int_str(length);
    let hex_str: &str = unsafe { str::from_utf8_unchecked(&hex_buf) };
    print(&hex_str);
    print("B\n");

    print("[*] Stardust Data Offset:\t");
    let hex_buf = usize_to_hex_str(offset);
    let hex_str: &str = unsafe { str::from_utf8_unchecked(&hex_buf) };
    print(&hex_str);
    print("\n");

    let data_addr = unsafe { start.add(offset / size_of::<usize>()) };
    print("[*] Stardust Data Address:\t");
    let hex_buf = usize_to_hex_str(data_addr as usize);
    let hex_str: &str = unsafe { str::from_utf8_unchecked(&hex_buf) };
    print(&hex_str);
    print("\n");

    unsafe {
        let _ = syscall!(Sysno::mprotect, data_addr, size_of::<usize>(), 0x1 | 0x2);
    }

    let instance = instance();
    let instance_ptr = instance as *const Instance;

    print("[*] Stardust Instance:\t\t");
    let hex_buf = usize_to_hex_str(instance_ptr as usize);
    let hex_str: &str = unsafe { str::from_utf8_unchecked(&hex_buf) };
    print(&hex_str);
    print("\n");

    exit(0);
}

fn print(s: &str) {
    unsafe {
        let _ = syscall!(Sysno::write, 0x01, s.as_ptr(), s.len());
    }
}

fn exit(code: u8) {
    unsafe {
        _ = syscall!(Sysno::exit, code);
    }
}

fn usize_to_hex_str(num: usize) -> [u8; 18] {
    let mut buffer = [b'0'; 16]; // Buffer to hold the hex characters
    let mut value = num;
    let mut index = 15; // Start from the end of the buffer

    while value > 0 {
        let digit = (value % 16) as usize; // Get the last hex digit
        buffer[index] = match digit {
            0..=9 => b'0' + digit as u8,
            _ => b'a' + (digit - 10) as u8,
        };
        value /= 16; // Move to the next digit
        index -= 1;
    }

    let start_index = index + 1; // First valid character index

    // Create a new buffer to store the valid hex characters
    let mut result = [0u8; 18];
    result[0] = b'0';
    result[1] = b'x';

    // Manually copy valid characters to the result buffer to avoid memcpy
    let mut result_index = 2;
    for i in start_index..16 {
        result[result_index] = buffer[i];
        result_index += 1;
    }

    result
}

fn usize_to_int_str(num: usize) -> [u8; 20] {
    let mut buffer = [b'0'; 20]; // Buffer to hold the integer characters
    let mut value = num;
    let mut index = 19; // Start from the end of the buffer

    // Handle the case when num is 0
    if value == 0 {
        buffer[index] = b'0';
        return buffer;
    }

    // Convert the number to its string representation
    while value > 0 {
        let digit = (value % 10) as usize; // Get the last decimal digit
        buffer[index] = b'0' + digit as u8; // Convert to ASCII
        value /= 10; // Move to the next digit
        index -= 1;
    }

    let start_index = index + 1; // First valid character index

    let mut result = [0u8; 20];

    // Manually copy valid characters to the result buffer to avoid memcpy
    let mut result_index = 0;
    for i in start_index..20 {
        result[result_index] = buffer[i];
        result_index += 1;
    }

    result
}
