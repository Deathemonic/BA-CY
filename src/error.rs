use thiserror::Error;

#[derive(Error, Debug)]
pub enum HashError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Invalid file path")]
    InvalidPath,

    #[error("Expected 0x{expected:08X}, got 0x{actual:08X}")]
    Mismatch { expected: u32, actual: u32 },
}

#[derive(Error, Debug)]
pub enum TableEncryptionError {
    #[error(transparent)]
    Base64Decode(#[from] base64::DecodeError),

    #[error(transparent)]
    FromUtf16Error(#[from] std::string::FromUtf16Error),

    #[error("String conversion failed")]
    StringConversionFailed,
}
