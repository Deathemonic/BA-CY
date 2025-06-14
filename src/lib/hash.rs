use std::path::PathBuf;

use xxhash_rust::xxh32::Xxh32;
use anyhow::Result;
use md5::{Digest, Md5};

pub fn calculate_crc32(path: PathBuf) -> Result<u32> {
    let data: Vec<u8> = std::fs::read(path)?;
    Ok(crc32fast::hash(&data))
}

pub fn calculate_md5(path: PathBuf) -> Result<String> {
    let data: Vec<u8> = std::fs::read(path)?;
    let mut hasher = Md5::new();
    hasher.update(&data);
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

pub fn calculate_xxhash(bytes: &[u8]) -> u32 {
    let mut hasher: Xxh32 = Xxh32::new(0);
    hasher.update(bytes);
    hasher.digest()
}