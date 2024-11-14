#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "linux")]
pub use linux::{initialize, log_str, StLinuxAllocator as StardustAllocator};

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub use windows::{initialize, log_str, StWindowsAllocator as StardustAllocator};
