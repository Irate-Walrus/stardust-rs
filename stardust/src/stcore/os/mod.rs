#[cfg(feature = "linux")]
pub mod linux;

#[cfg(feature = "linux")]
pub use linux::{initialize, log_str, StLinuxAllocator as StardustAllocator};

#[cfg(feature = "windows")]
pub mod windows;

#[cfg(feature = "windows")]
pub use windows::{initialize, log_str, StWindowsAllocator as StardustAllocator};
