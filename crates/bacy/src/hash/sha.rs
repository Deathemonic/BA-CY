use std::fs::File;
use std::io::Read;
use std::path::Path;

use sha1::Sha1;
use sha2::{Digest, Sha256};

use crate::crypto::md5;
use crate::error::HashError;

const MIN_BUFFER_SIZE: usize = 4096;

pub fn compute_sha1(source: &[u8]) -> [u8; 20] { Sha1::digest(source).into() }

pub fn compute_sha1_hex(source: &[u8]) -> String { md5::to_hex_string(&compute_sha1(source)) }

pub fn compute_streaming(path: &Path, buffer_size: usize) -> Result<String, HashError> {
    if !path.exists() {
        return Err(HashError::InvalidPath);
    }

    let mut file = File::open(path)?;
    let mut hasher = Sha1::new();
    let mut buffer = vec![0_u8; buffer_size.max(MIN_BUFFER_SIZE)];

    loop {
        let bytes_read = file.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    Ok(md5::to_hex_string(&hasher.finalize()))
}

pub fn compute(source: &[u8]) -> [u8; 32] { Sha256::digest(source).into() }

pub fn compute_str(source: &str) -> [u8; 32] { compute(source.as_bytes()) }
