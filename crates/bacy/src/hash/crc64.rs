use std::fs::File;
use std::io::Read;
use std::path::Path;

use crc64fast::Digest;

use crate::error::HashError;

const MIN_BUFFER_SIZE: usize = 4096;

#[inline]
pub fn compute_bytes(buffer: &[u8]) -> u64 {
    let mut digest = Digest::new();
    digest.write(buffer);
    digest.sum64()
}

pub fn compute_streaming(path: &Path, buffer_size: usize) -> Result<u64, HashError> {
    if !path.exists() {
        return Err(HashError::InvalidPath);
    }

    let mut file = File::open(path)?;
    let mut digest = Digest::new();
    let mut buffer = vec![0u8; buffer_size.max(MIN_BUFFER_SIZE)];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        digest.write(&buffer[..bytes_read]);
    }

    Ok(digest.sum64())
}

pub fn compare(path: &Path, expected_crc: u64) -> Result<(), HashError> {
    let file_crc = compute_streaming(path, 0x2000)?;

    if file_crc == expected_crc {
        Ok(())
    } else {
        Err(HashError::Crc64Mismatch {
            expected: expected_crc,
            actual: file_crc
        })
    }
}