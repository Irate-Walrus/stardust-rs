/// Constant to the instance is correctly allocated
pub const INSTANCE_MAGIC: u32 = 0xDEADBEEF;

pub struct Instance {
    pub magic: u32,
}

impl Default for Instance {
    fn default() -> Self {
        Instance {
            magic: INSTANCE_MAGIC,
        }
    }
}
