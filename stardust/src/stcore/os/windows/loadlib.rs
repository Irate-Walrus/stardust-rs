use super::ntpeb::{find_peb, ImageNtHeaders, LoaderDataTableEntry, PebLoaderData};

use core::ptr;
use goblin;

define_djb2_hash_fn!(rt_djb2_hash);

/// Retrieves the NT headers from the base address of a module.
///
/// # Arguments
/// * `base_addr` - The base address of the module.
///
/// Returns a pointer to `ImageNtHeaders` or null if the headers are invalid.
#[cfg(target_arch = "x86_64")]
pub unsafe fn get_nt_headers(base_addr: *const usize) -> *const ImageNtHeaders {
    use super::ntpeb::ImageNtHeaders;

    let dos_header = base_addr as *const goblin::pe::header::DosHeader;

    // Check if the DOS signature is valid (MZ)
    if (*dos_header).signature != goblin::pe::header::DOS_MAGIC {
        return ptr::null_mut();
    }

    // Calculate the address of NT headers
    let nt_headers = base_addr.add((*dos_header).pe_pointer as usize / size_of::<usize>())
        as *const ImageNtHeaders;

    // Check if the NT signature is valid (PE\0\0)
    if (*nt_headers).signature != goblin::pe::header::PE_MAGIC {
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
pub unsafe fn ldr_module(module_hash: u32) -> *mut usize {
    let peb = find_peb(); // Retrieve the PEB (Process Environment Block)

    if peb.is_null() {
        return ptr::null_mut();
    }

    let peb_ldr_data_ptr = (*peb).loader_data as *mut PebLoaderData;
    if peb_ldr_data_ptr.is_null() {
        return ptr::null_mut();
    }

    // Start with the first module in the InLoadOrderModuleList
    let mut module_list =
        (*peb_ldr_data_ptr).in_load_order_module_list.flink as *mut LoaderDataTableEntry;

    // Iterate through the list of loaded modules
    while !(*module_list).dll_base.is_null() {
        let dll_buffer_ptr = (*module_list).base_dll_name.buffer;
        let dll_length = (*module_list).base_dll_name.length as usize;

        // Create a slice from the DLL name
        let dll_name_slice = core::slice::from_raw_parts(dll_buffer_ptr as *const u8, dll_length);

        // Compare the hash of the DLL name with the provided hash
        if module_hash == rt_djb2_hash(dll_name_slice) {
            return (*module_list).dll_base as _; // Return the base address of the module if the hash matches
        }

        // Move to the next module in the list
        module_list = (*module_list).in_load_order_links.flink as *mut LoaderDataTableEntry;
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
pub unsafe fn ldr_function(module_base: *const usize, function_hash: u32) -> *const usize {
    let p_img_nt_headers = get_nt_headers(module_base); // Retrieve NT headers for the module

    if p_img_nt_headers.is_null() {
        return ptr::null_mut();
    }

    // Get the export directory from the NT headers
    let optional_header = &(*p_img_nt_headers).optional_header;

    // Assuming IMAGE_DIRECTORY_ENTRY_EXPORT is 0
    let data_dir = &(*optional_header).data_directory[0];

    let export_dir_table = module_base.byte_add(data_dir.virtual_address as usize)
        as *const goblin::pe::export::ExportDirectoryTable;

    if export_dir_table.is_null() {
        return ptr::null_mut();
    }

    let entry_len = (*export_dir_table).address_table_entries as usize;
    let names_addr =
        module_base.byte_add((*export_dir_table).name_pointer_rva as usize) as *const u32;
    let addresses_addr =
        module_base.byte_add((*export_dir_table).export_address_table_rva as usize) as *const u32;
    let ordinals_addr =
        module_base.byte_add((*export_dir_table).ordinal_table_rva as usize) as *const u16;

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
            return module_base.byte_add(functions[ordinal] as usize) as *mut usize;
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
