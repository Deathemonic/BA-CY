use xxhash_rust::{xxh32::xxh32, xxh64::xxh64};

#[inline]
pub fn calculate_hash(bytes: &[u8]) -> u32 {
    xxh32(bytes, 0)
}

pub fn calculate_hash_str(s: &str) -> u32 {
    if s.is_empty() {
        return 0;
    }
    calculate_hash(s.as_bytes())
}

#[inline]
pub fn calculate_hash64(bytes: &[u8]) -> u64 {
    xxh64(bytes, 0)
}

pub fn calculate_hash64_str(s: &str) -> u64 {
    if s.is_empty() {
        return 0;
    }
    calculate_hash64(s.as_bytes())
}
