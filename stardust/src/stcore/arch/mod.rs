#[cfg(target_arch = "x86")]
mod i686;

#[cfg(target_arch = "x86")]
pub use i686::{rip_end, rip_start};

#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use x86_64::{rip_end, rip_start};
