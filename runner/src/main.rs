use std::mem;
use virtual_memory::*;

const SHELLCODE: &[u8] = include_bytes!("../../target/stardust.bin");

fn main() {
    let mut memory = VirtualMemory::new(SHELLCODE.len()).expect("failed to allocate rwx memory");
    memory.copy_from_slice(SHELLCODE);

    let exec: extern "C" fn() -> ! = unsafe { mem::transmute(memory.as_ptr()) };
    exec();
}
