use core::ffi::c_void;
use core::ffi::CStr;
use core::slice;

use syscalls::{syscall, Sysno};

#[cfg(target_pointer_width = "64")]
pub use goblin::elf64 as elf;

#[cfg(target_pointer_width = "32")]
pub use goblin::elf32 as elf;

use elf::dynamic::DT_DEBUG;
use elf::program_header::PT_DYNAMIC;

define_djb2_hash_fn!(rt_djb2_hash);

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

const AT_PHDR: usize = 3;
//const AT_PHENT: usize = 4;
const AT_PHNUM: usize = 5;
//const AT_BASE: usize = 7;

pub unsafe fn resolve_module(sym_hash: u32) -> Option<*const c_void> {
    let path = b"/proc/self/auxv\0";
    let fd = open(path.as_ptr(), 0);

    let mut buf = [0u8; size_of::<Auxv>() * 32];
    let count = read(fd, buf.as_mut_ptr(), buf.len());
    close(fd);

    let auxv = slice::from_raw_parts(buf.as_ptr() as *const Auxv, count as usize); // Approximate length

    let at_phdr = match auxv.iter().find(|e| e.key == AT_PHDR) {
        Some(kv) => kv.value,
        None => return None,
    };

    let at_phnum = match auxv.iter().find(|e| e.key == AT_PHNUM) {
        Some(kv) => kv.value,
        None => return None,
    };
    //println!("AT_PHDR@{:#x}", at_phdr);

    let phdr = at_phdr as *const elf::program_header::ProgramHeader;

    for i in 0..at_phnum {
        let phdr_entry = phdr.add(i as usize);
        //println!("PT_{}@{:#x}", (*phdr_entry).p_type, phdr_entry as usize);

        if (*phdr_entry).p_type == PT_DYNAMIC {
            // PT_DYNAMIC
            let dynamic = (at_phdr + (*phdr_entry).p_vaddr as usize) as *const elf::dynamic::Dyn;
            let mut dyn_entry = dynamic;

            while (*dyn_entry).d_tag != 0 {
                //println!("DT_{}@{:#x}", (*dyn_entry).d_tag, dynamic as usize);
                if (*dyn_entry).d_tag as u64 == DT_DEBUG {
                    // DT_DEBUG
                    let r_debug = (*dyn_entry).d_val as *const RDebug;
                    //println!("RDBG@{:#x}", r_debug as usize);

                    let mut current = (*r_debug).r_map;
                    while !current.is_null() {
                        let name_ptr = (*current).name;
                        if !name_ptr.is_null() {
                            let name_cstr = CStr::from_ptr(name_ptr as *const i8);
                            let name_str = match name_cstr.to_str() {
                                Ok(s) => s,
                                Err(_) => continue,
                            };

                            //println!("{}@{:#x}", name_str, (*current).addr as usize);

                            let mut rt_hash = 0;
                            if let (Some(dot), Some(slash)) =
                                (name_str.rfind('.'), name_str.rfind('/'))
                            {
                                let slice = &name_str[slash + 1..dot];
                                rt_hash = rt_djb2_hash(slice.as_bytes());
                            }

                            if rt_hash == sym_hash {
                                return Some((*current).addr as *const c_void);
                            }
                        }
                        current = (*current).next;
                    }
                    break;
                }
                dyn_entry = dyn_entry.add(1);
            }
        }
    }

    None
}

#[repr(C)]
struct Auxv {
    key: usize,
    value: usize,
}

#[repr(C)]
struct LinkMap {
    addr: usize,
    name: *const u8,
    ld: usize,
    next: *mut LinkMap,
    prev: *mut LinkMap,
}

#[repr(C)]
struct RDebug {
    r_version: i32,
    r_map: *mut LinkMap,
    r_brk: usize,
    r_state: i32,
    r_ldbase: usize,
}

pub unsafe fn resolve_function(module_base: *const c_void, sym_hash: u32) -> Option<*const usize> {
    let elf_header = module_base as *const elf::header::Header;
    let program_header_offset = (*elf_header).e_phoff as usize;
    // This would allow me to calulate the length of Elf64_Sym[]
    let _section_header_offset = (*elf_header).e_shoff as usize;
    let num_headers = (*elf_header).e_phnum as usize;
    let header_size = (*elf_header).e_phentsize as usize;

    let mut dynsym_addr = 0;
    let mut dynstr_addr = 0;

    //info_int!("num_headers", num_headers);
    for i in 0..num_headers {
        let program_header = (module_base as usize + program_header_offset + i * header_size)
            as *const elf::program_header::ProgramHeader;

        // info_addr!("program_header", program_header);
        // p_type, identifies the type of segment
        if (*program_header).p_type == 2 {
            // PT_DYNAMIC, dynamic linking tables
            let dyn_section = (*program_header).p_vaddr as usize + module_base as usize;
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
        let name_cstr = CStr::from_ptr(name_ptr as *const i8);

        //println!(
        //    "{:?}@{:#x}",
        //    name_cstr,
        //    module_base as usize + (*sym).st_value as usize
        //);

        if rt_djb2_hash(name_cstr.to_bytes()) == sym_hash {
            let addr = module_base as usize + (*sym).st_value as usize;
            return Some(addr as *const usize);
        }
        sym = sym.add(1);
    }

    None
}
