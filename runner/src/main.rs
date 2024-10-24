use std::mem;
use virtual_memory::*;

const SHELLCODE: &[u8] = include_bytes!("../../target/stardust.bin");

fn main() {
    let mut memory = VirtualMemory::new(SHELLCODE.len()).expect("failed to allocate rwx memory");
    memory.copy_from_slice(SHELLCODE);

    let exec: extern "C" fn() -> ! = unsafe { mem::transmute(memory.as_ptr()) };

    println!(
        "[>] Allocation Start Address:\t0x{:x}",
        memory.as_ptr() as usize
    );
    println!("[>] Allocation End Address:\t0x{:x}", unsafe {
        memory.as_ptr().offset(memory.len() as isize) as usize
    });
    println!("[>] Allocation Size:\t\t{}B", memory.len());
    exec();
}
