use syscalls::{syscall, Sysno};

#[macro_export]
macro_rules! info_addr {
    ($name:expr, $addr:expr) => {
        let mut prefix = concat!("[*] ", $name, ":\t");
        if prefix.len() < 25 {
            prefix = concat!("[*] ", $name, ":\t\t");
        }

        let hex_buf = usize_to_hex_str($addr as usize);
        let hex_str: &str = unsafe { core::str::from_utf8_unchecked(&hex_buf) };

        $crate::utils::print(prefix);
        $crate::utils::print(&hex_str);
        $crate::utils::print("\n");
    };
}

#[macro_export]
macro_rules! info_size {
    ($name:expr, $addr:expr) => {
        let mut prefix = concat!("[*] ", $name, ":\t");
        if prefix.len() < 25 {
            prefix = concat!("[*] ", $name, ":\t\t");
        }

        let hex_buf = usize_to_int_str($addr as usize);
        let hex_str: &str = unsafe { core::str::from_utf8_unchecked(&hex_buf) };

        $crate::utils::print(prefix);
        $crate::utils::print(&hex_str);
        $crate::utils::print("B\n");
    };
}

#[macro_export]
macro_rules! info {
    ($s:expr) => {
        $crate::utils::print("[*] ");
        $crate::utils::print($s);
        $crate::utils::print("\n");
    };
}

pub fn print(s: &str) {
    unsafe {
        let _ = syscall!(Sysno::write, 0x01, s.as_ptr(), s.len());
    }
}
