use crate::error::HashError;

use crc32fast::Hasher;
use md5::{Digest, Md5};
use std::fs;
use std::path::Path;
use xxhash_rust::{xxh32::Xxh32, xxh64::Xxh64};

pub struct CrcResult {
    pub value: u32,
    pub hex: String,
}

impl CrcResult {
    pub fn new(value: u32) -> Self {
        Self {
            value,
            hex: format!("{value:08X}"),
        }
    }
}

pub fn calculate_crc32(path: &Path) -> Result<u32, HashError> {
    let data = fs::read(path)?;
    Ok(crc32fast::hash(&data))
}

pub fn evaluate_crc32(data: &[u8]) -> CrcResult {
    let mut hasher = Hasher::new();
    hasher.update(data);
    let crc_value = hasher.finalize();
    CrcResult::new(crc_value)
}

pub fn calculate_md5(path: &Path) -> Result<String, HashError> {
    let data = fs::read(path)?;
    let mut hasher = Md5::new();
    hasher.update(&data);
    let result = hasher.finalize();
    Ok(format!("{result:x}"))
}

pub fn calculate_xxhash(bytes: &[u8], bit64: bool, endian: bool) -> u64 {
    if !bit64 {
        let mut hasher = Xxh32::new(0);
        hasher.update(bytes);
        return hasher.digest() as u64;
    }

    let mut hasher = Xxh64::new(0);
    hasher.update(bytes);
    let hash = hasher.digest();

    if endian && cfg!(target_endian = "little") { hash.to_be() } else { hash }
}

pub fn encrypt_name(filename: &str, crc: i64) -> Result<String, HashError> {
    Ok(format!(
        "{}_{}",
        calculate_xxhash(filename.to_lowercase().as_bytes(), true, true),
        crc
    ))
}
