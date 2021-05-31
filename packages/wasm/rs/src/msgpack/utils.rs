pub struct Block {
    /// Memory manager info
    mm_info: u32,
}

pub const BLOCK_OVERHEAD: usize = 100; // What is offsetof in typescript?
pub const BLOCK_MAX_SIZE: usize = (1 << 30) - BLOCK_OVERHEAD;
pub const E_INDEX_OUT_OF_RANGE: &str = "Index out of range";
pub const E_INVALID_LENGTH: &str = "Invalid length";
