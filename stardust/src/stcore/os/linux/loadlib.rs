use core::{ffi::CStr, intrinsics::breakpoint};
use syscalls::{syscall, Sysno};

#[cfg(target_pointer_width = "64")]
pub use goblin::elf64 as elf;

#[cfg(target_pointer_width = "32")]
pub use goblin::elf32 as elf;

define_djb2_hash_fn!(runtime_djb2_hash);

// Opens a file using the `open` syscall
pub unsafe fn open(path: *const u8, flags: usize) -> usize {
    syscall!(Sysno::open, path, flags).unwrap()
}

// Reads from a file using the `read` syscall
pub unsafe fn read(fd: usize, buf: *mut u8, count: usize) -> usize {
    syscall!(Sysno::read, fd, buf, count).unwrap()
}

// Closes a file using the `close` syscall
pub unsafe fn close(fd: usize) -> usize {
    syscall!(Sysno::close, fd).unwrap()
}

pub unsafe fn find_lib(sym_hash: u32, sym_len: usize) -> Option<*const usize> {
    let path = b"/proc/self/maps\0";
    let mut buffer = [0u8; 4096];

    // Open /proc/self/maps
    let fd = unsafe { open(path.as_ptr(), 0) };

    // Read the contents of /proc/self/maps
    let count = unsafe { read(fd, buffer.as_mut_ptr(), buffer.len()) };

    // close proc/self/map
    unsafe { close(fd) };

    find_lib_in_proc_map(&buffer[..count as usize], sym_hash, sym_len)
}

pub fn find_lib_in_proc_map(buffer: &[u8], sym_hash: u32, sym_len: usize) -> Option<*const usize> {
    let mut lines = buffer.split(|&c| c == b'\n');

    while let Some(line) = lines.next() {
        if line
            .windows(sym_len) // size of b"libc", hardcoded for time being
            .any(|w| runtime_djb2_hash(w) == sym_hash)
        {
            // Parse the starting address in hex
            if let Some(pos) = line.iter().position(|&c| c == b'-') {
                let addr_hex = &line[..pos];
                let addr_str = core::str::from_utf8(addr_hex).ok()?;
                return Some(usize::from_str_radix(addr_str, 16).unwrap() as *const usize);
            }
        }
    }
    None
}

pub unsafe fn find_fn_in_lib(lib_base: *const usize, sym_hash: u32) -> Option<*const usize> {
    let elf_header = lib_base as *const elf::header::Header;
    let program_header_offset = (*elf_header).e_phoff as usize;
    // This would allow me to calulate the length of Elf64_Sym[]
    let _section_header_offset = (*elf_header).e_shoff as usize;
    let num_headers = (*elf_header).e_phnum as usize;
    let header_size = (*elf_header).e_phentsize as usize;

    let mut dynsym_addr = 0;
    let mut dynstr_addr = 0;

    //info_int!("num_headers", num_headers);
    for i in 0..num_headers {
        let program_header = (lib_base as usize + program_header_offset + i * header_size)
            as *const elf::program_header::ProgramHeader;

        // info_addr!("program_header", program_header);
        // p_type, identifies the type of segment
        if (*program_header).p_type == 2 {
            // PT_DYNAMIC, dynamic linking tables
            let dyn_section = (*program_header).p_vaddr as usize + lib_base as usize;
            let mut dyn_entry = dyn_section as *const elf::dynamic::Dyn;
            // info_addr!("dyn_entry", dyn_entry);

            // DT_NULL, marks the end of the dynamic array(maybe?)
            while (*dyn_entry).d_tag != 0 {
                match (*dyn_entry).d_tag {
                    5 => dynstr_addr = (*dyn_entry).d_val as usize, // DT_STRTAB, address of the dynamic string table
                    6 => dynsym_addr = (*dyn_entry).d_val as usize, // DT_SYMTAB, address of the dynamic symbol table
                    _ => {}
                }
                dyn_entry = dyn_entry.add(1);
            }
            break;
        }
    }

    if dynsym_addr == 0 || dynstr_addr == 0 {
        return None;
    }

    // info_addr!("dynsym_addr", dynsym_addr);
    // info_addr!("dynstr_addr", dynstr_addr);
    let mut sym = dynsym_addr as *const elf::sym::Sym;
    // st_name, contains the offset, in bytes, to the symbol name, relative to the
    // start of the symbol string table. If this field contains zero, the symbol has
    // no name
    // this loop is bad as it assumes you will find the symbol...
    sym = sym.add(1);
    while (*sym).st_name != 0 {
        let name_ptr = (dynstr_addr + (*sym).st_name as usize) as *const u8;
        //info_addr!("name_ptr", name_ptr);
        let cstr = CStr::from_ptr(name_ptr as *const i8);
        runtime_djb2_hash(cstr.to_bytes());

        if runtime_djb2_hash(cstr.to_bytes()) == sym_hash {
            return Some(core::mem::transmute(
                lib_base as usize + (*sym).st_value as usize,
            ));
        }
        sym = sym.add(1);
    }

    None
}
