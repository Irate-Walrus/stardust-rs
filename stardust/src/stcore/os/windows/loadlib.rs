use core::arch::asm;
use core::{ffi::c_void, ptr};
use phnt::ffi::{IMAGE_DOS_HEADER, PIMAGE_EXPORT_DIRECTORY, PLDR_DATA_TABLE_ENTRY, PPEB_LDR_DATA};
#[cfg(target_arch = "x86")]
use phnt::ffi::{PIMAGE_NT_HEADERS32 as PIMAGE_NT_HEADERS, PPEB32 as PPEB};
#[cfg(target_arch = "x86_64")]
use phnt::ffi::{PIMAGE_NT_HEADERS64 as PIMAGE_NT_HEADERS, PPEB};

define_djb2_hash_fn!(rt_djb2_hash);

#[cfg(target_arch = "x86_64")]
pub fn find_peb() -> PPEB {
    let peb_ptr: PPEB;
    unsafe {
        asm!(
        "mov {}, gs:[0x60]",
        out(reg) peb_ptr
        );
    }
    peb_ptr
}

#[cfg(target_arch = "x86")]
pub fn find_peb() -> PPEB {
    let peb_ptr: PPEB;
    unsafe {
        asm!(
        "mov {}, gs:[0x30]",
        out(reg) peb_ptr
        );
    }
    peb_ptr
}

/// Retrieves the NT headers from the base address of a module.
///
/// # Arguments
/// * `base_addr` - The base address of the module.
///
/// Returns a pointer to `ImageNtHeaders` or null if the headers are invalid.
pub unsafe fn get_nt_headers(base_addr: *mut c_void) -> PIMAGE_NT_HEADERS {
    let dos_header = base_addr as *const IMAGE_DOS_HEADER;

    // Check if the DOS signature is valid (MZ)
    if (*dos_header).e_magic != 0x5A4D {
        return ptr::null_mut();
    }

    // Calculate the address of NT headers
    let nt_headers = base_addr.byte_add((*dos_header).e_lfanew as usize) as PIMAGE_NT_HEADERS;

    // Check if the NT signature is valid (PE\0\0)
    if (*nt_headers).Signature != 0x4550 {
        return ptr::null_mut();
    }

    nt_headers
}

/// Finds and returns the base address of a module by its hash.
///
/// # Arguments
/// * `module_hash` - The hash of the module name to locate.
///
/// Returns the base address of the module or null if not found.
pub unsafe fn ldr_module(module_hash: u32) -> *mut c_void {
    let peb = find_peb(); // Retrieve the PEB (Process Environment Block)

    if peb.is_null() {
        return ptr::null_mut();
    }

    let peb_ldr_data_ptr = (*peb).Ldr as PPEB_LDR_DATA;
    if peb_ldr_data_ptr.is_null() {
        return ptr::null_mut();
    }

    // Start with the first module in the InLoadOrderModuleList
    let mut module_list = (*peb_ldr_data_ptr).InLoadOrderModuleList.Flink as PLDR_DATA_TABLE_ENTRY;

    // Iterate through the list of loaded modules
    while !(*module_list).DllBase.is_null() {
        let dll_buffer_ptr = (*module_list).BaseDllName.Buffer;
        let dll_length = (*module_list).BaseDllName.Length as usize;

        // Create a slice from the DLL name
        let dll_name_slice = core::slice::from_raw_parts(dll_buffer_ptr as *const u8, dll_length);

        // Compare the hash of the DLL name with the provided hash
        if module_hash == rt_djb2_hash(dll_name_slice) {
            return (*module_list).DllBase; // Return the base address of the module if the hash matches
        }

        // Move to the next module in the list
        module_list = (*module_list).InLoadOrderLinks.Flink as PLDR_DATA_TABLE_ENTRY;
    }

    ptr::null_mut() // Return null if no matching module is found
}

/// Finds a function by its hash from the export directory of a module.
///
/// # Arguments
/// * `module_base` - The base address of the module.
/// * `function_hash` - The hash of the function name to locate.
///
/// Returns the function's address or null if not found.
pub unsafe fn ldr_function(module_base: *mut c_void, function_hash: u32) -> *mut c_void {
    let p_img_nt_headers = get_nt_headers(module_base); // Retrieve NT headers for the module

    if p_img_nt_headers.is_null() {
        return ptr::null_mut();
    }

    // Get the export directory from the NT headers
    let optional_header = &(*p_img_nt_headers).OptionalHeader;

    // Assuming IMAGE_DIRECTORY_ENTRY_EXPORT is 0
    let data_dir = &(*optional_header).DataDirectory[0];

    let export_dir_table =
        module_base.byte_add(data_dir.VirtualAddress as usize) as PIMAGE_EXPORT_DIRECTORY;

    if export_dir_table.is_null() {
        return ptr::null_mut();
    }

    let entry_len = (*export_dir_table).NumberOfNames as usize;
    let names_addr =
        module_base.byte_add((*export_dir_table).AddressOfNames as usize) as *const u32;
    let addresses_addr =
        module_base.byte_add((*export_dir_table).AddressOfFunctions as usize) as *const u32;
    let ordinals_addr =
        module_base.byte_add((*export_dir_table).AddressOfNameOrdinals as usize) as *const u16;

    // Create slices from the export directory arrays
    let names = core::slice::from_raw_parts(names_addr, entry_len);
    let functions = core::slice::from_raw_parts(addresses_addr, entry_len);
    let ordinals = core::slice::from_raw_parts(ordinals_addr, entry_len);

    // Iterate through the export names to find the function matching the given hash
    for i in 0..entry_len {
        let name_addr = module_base.byte_add(names[i] as usize);

        // Using CStr here increased shellcode size by 10x
        let name_len = get_cstr_len(name_addr as _);
        let name_slice: &[u8] = core::slice::from_raw_parts(name_addr as _, name_len);

        // Compare the hash of the function name with the provided hash
        if function_hash as u32 == rt_djb2_hash(name_slice) {
            // Retrieve the function's address by its ordinal
            let ordinal = ordinals[i] as usize;
            return module_base.byte_add(functions[ordinal] as usize);
        }
    }

    ptr::null_mut()
}

/// Calculates the length of a C-style null-terminated string.
///
/// This function counts the number of characters in the string until it encounters a null byte.
pub fn get_cstr_len(pointer: *const char) -> usize {
    let mut tmp: u64 = pointer as u64;

    // Iterate over the string until a null byte (0) is found
    unsafe {
        while *(tmp as *const u8) != 0 {
            tmp += 1;
        }
    }

    // Return the length of the string (difference between the end and start)
    (tmp - pointer as u64) as _
}
