#[macro_export]
macro_rules! info_addr {
    ($name:expr, $addr:expr) => {
        let prefix = match $name.len() {
            x if x < 11 => concat!("[*] ", $name, ":\t\t\t"),
            x if x < 19 => concat!("[*] ", $name, ":\t\t"),
            _ => concat!("[*] ", $name, ":\t"),
        };

        let hex_buf = $crate::stcore::macros::usize_to_hex_str($addr as usize);
        let hex_str: &str = unsafe { core::str::from_utf8_unchecked(&hex_buf) };

        $crate::stcore::log_str(prefix);
        $crate::stcore::log_str(&hex_str);
        $crate::stcore::log_str("\n");
    };
}

#[macro_export]
macro_rules! info_int {
    ($name:expr, $addr:expr) => {
        let prefix = match $name.len() {
            x if x < 11 => concat!("[*] ", $name, ":\t\t\t"),
            x if x < 19 => concat!("[*] ", $name, ":\t\t"),
            _ => concat!("[*] ", $name, ":\t"),
        };

        let hex_buf = $crate::stcore::macros::usize_to_int_str($addr as usize);
        let hex_str: &str = unsafe { core::str::from_utf8_unchecked(&hex_buf) };

        $crate::stcore::log_str(prefix);
        $crate::stcore::log_str(&hex_str);
        $crate::stcore::log_str("\n");
    };
}

#[macro_export]
macro_rules! info {
    ($s:expr) => {
        $crate::stcore::log_str("[*] ");
        $crate::stcore::log_str($s);
        $crate::stcore::log_str("\n");
    };
}

pub fn usize_to_hex_str(num: usize) -> [u8; 18] {
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

pub fn usize_to_int_str(num: usize) -> [u8; 20] {
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
