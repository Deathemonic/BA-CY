use std::sync::atomic::{AtomicBool, Ordering};
use xxhash_rust::{xxh32::xxh32, xxh64::xxh64};

static USE_BIG_ENDIAN: AtomicBool = AtomicBool::new(true);

pub fn set_use_big_endian(value: bool) {
    USE_BIG_ENDIAN.store(value, Ordering::Relaxed);
}

pub fn get_use_big_endian() -> bool {
    USE_BIG_ENDIAN.load(Ordering::Relaxed)
}

fn apply_endianness<T: Into<u64>>(hash: T) -> u64 {
    let value = hash.into();
    if get_use_big_endian() {
        value.swap_bytes()
    } else {
        value
    }
}

#[inline]
pub fn calculate_hash(bytes: &[u8]) -> u32 {
    apply_endianness(xxh32(bytes, 0)) as u32
}

pub fn calculate_hash_str(s: &str) -> u32 {
    if s.is_empty() {
        return 0;
    }
    calculate_hash(s.as_bytes())
}

#[inline]
pub fn calculate_hash64(bytes: &[u8]) -> u64 {
    apply_endianness(xxh64(bytes, 0))
}

pub fn calculate_hash64_str(s: &str) -> u64 {
    if s.is_empty() {
        return 0;
    }
    calculate_hash64(s.as_bytes())
}
