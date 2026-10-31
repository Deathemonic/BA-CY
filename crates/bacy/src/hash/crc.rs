use std::fs::File;
use std::io::Read;
use std::path::Path;

use crc32fast::Hasher;
use tokio::fs;

use crate::error::HashError;

const MIN_BUFFER_SIZE: usize = 4096;

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

#[inline]
pub fn pad_zeros(crc: u32) -> u32 {
    let mut hasher = Hasher::new_with_initial(crc);
    hasher.update(&[0, 0, 0, 0]);
    hasher.finalize()
}

pub fn compute_streaming(path: &Path, buffer_size: usize) -> Result<u32, HashError> {
    if !path.exists() {
        return Err(HashError::InvalidPath);
    }

    let mut file = File::open(path)?;
    let mut hasher = Hasher::new();
    let mut buffer = vec![0_u8; buffer_size.max(MIN_BUFFER_SIZE)];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hasher.finalize())
}

pub fn compare(path: &Path, expected_crc: u32) -> Result<(), HashError> {
    if !path.exists() {
        return Err(HashError::InvalidPath);
    }

    let file_crc = compute_streaming(path, 0x2000)?;

    if file_crc == expected_crc {
        Ok(())
    } else {
        Err(HashError::Crc32Mismatch {
            expected: expected_crc,
            actual: file_crc
        })
    }
}
