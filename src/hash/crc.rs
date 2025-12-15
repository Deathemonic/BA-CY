use crate::error::HashError;

use crc32fast::Hasher;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tokio::fs;

pub async fn compute(path: &Path) -> Result<u32, HashError> {
    if !path.exists() {
        return Ok(0);
    }

    let bytes = fs::read(path).await?;
    Ok(compute_bytes(&bytes, None))
}

#[inline]
pub fn compute_bytes(buffer: &[u8], suffix: Option<&[u8]>) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(buffer);
    if let Some(s) = suffix {
        hasher.update(s);
    }
    hasher.finalize()
}

pub fn compute_streaming(
    path: &Path,
    buffer_size: usize,
    suffix: Option<&[u8]>,
) -> Result<u32, HashError> {
    if !path.exists() {
        return Err(HashError::InvalidPath);
    }

    let buffer_size = buffer_size.max(4096);
    let mut file = File::open(path)?;
    let mut hasher = Hasher::new();
    let mut buffer = vec![0u8; buffer_size];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    if let Some(s) = suffix {
        hasher.update(s);
    }

    Ok(hasher.finalize())
}

pub fn compare(path: &Path, expected_crc: u32) -> Result<(), HashError> {
    if !path.exists() {
        return Err(HashError::InvalidPath);
    }

    let file_crc = compute_streaming(path, 0x2000, None)?;

    if file_crc == expected_crc {
        Ok(())
    } else {
        Err(HashError::Mismatch {
            expected: expected_crc,
            actual: file_crc,
        })
    }
}
