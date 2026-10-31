use std::fmt::Write;
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::path::Path;

use hmac::{Hmac, KeyInit, Mac};
use md5::{Digest, Md5};

use crate::error::HashError;

const MIN_BUFFER_SIZE: usize = 4096;

type HmacMd5 = Hmac<Md5>;

pub fn to_hex_string(bytes: &[u8]) -> String {
    bytes.iter().fold(String::with_capacity(bytes.len() * 2), |mut out, b| {
        let _ = write!(out, "{b:02x}");
        out
    })
}

pub fn compute_hash(source: &[u8]) -> [u8; 16] {
    let mut hasher = Md5::new();
    hasher.update(source);
    hasher.finalize().into()
}

pub fn compute_streaming(path: &Path, buffer_size: usize) -> Result<String, HashError> {
    let mut file = File::open(path).map_err(|error| match error.kind() {
        ErrorKind::NotFound => HashError::InvalidPath,
        _ => HashError::Io(error)
    })?;

    let mut hasher = Md5::new();
    let mut buffer = vec![0_u8; buffer_size.max(MIN_BUFFER_SIZE)];

    loop {
        let bytes_read = file.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    Ok(to_hex_string(&hasher.finalize()))
}

pub fn compute_hash_hmac(source: &[u8], key: &[u8]) -> [u8; 16] {
    let mut mac = HmacMd5::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(source);
    mac.finalize().into_bytes().into()
}

pub fn compute_hash_str(source: &str) -> String { to_hex_string(&compute_hash(source.as_bytes())) }

pub fn compute_hash_str_hmac(source: &str, key: &str) -> String {
    to_hex_string(&compute_hash_hmac(source.as_bytes(), key.as_bytes()))
}

pub fn compute_digest(source: &str) -> u32 {
    let hash = compute_hash(source.as_bytes());
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

pub fn compute_digest_hmac(source: &str, key: &str) -> u32 {
    let hash = compute_hash_hmac(source.as_bytes(), key.as_bytes());
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

pub fn compute_digest64(source: &str) -> u64 {
    let hash = compute_hash(source.as_bytes());
    u64::from_le_bytes([hash[0], hash[1], hash[2], hash[3], hash[4], hash[5], hash[6], hash[7]])
}

pub fn compute_digest64_hmac(source: &str, key: &str) -> u64 {
    let hash = compute_hash_hmac(source.as_bytes(), key.as_bytes());
    u64::from_le_bytes([hash[0], hash[1], hash[2], hash[3], hash[4], hash[5], hash[6], hash[7]])
}

pub fn compute_head(source: &str) -> String {
    let hash = compute_hash(source.as_bytes());
    format!("{:02x}", hash[0])
}
