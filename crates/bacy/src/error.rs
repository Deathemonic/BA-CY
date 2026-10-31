use std::io;
use std::string::FromUtf16Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HashError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("Invalid file path")]
    InvalidPath,

    #[error("CRC32 mismatch: expected 0x{expected:08X}, got 0x{actual:08X}")]
    Crc32Mismatch { expected: u32, actual: u32 },

    #[error("CRC64 mismatch: expected 0x{expected:016X}, got 0x{actual:016X}")]
    Crc64Mismatch { expected: u64, actual: u64 }
}

#[derive(Error, Debug)]
pub enum TableEncryptionError {
    #[error(transparent)]
    Base64Decode(#[from] base64::DecodeError),

    #[error(transparent)]
    FromUtf16Error(#[from] FromUtf16Error),

    #[error("String conversion failed")]
    StringConversionFailed
}

#[derive(Error, Debug)]
pub enum AesError {
    #[error("Decryption failed")]
    Decrypt
}
