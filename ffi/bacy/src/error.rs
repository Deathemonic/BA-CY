use std::fmt;

use bacy::error::{AesError, HashError, TableEncryptionError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Hash(#[from] HashError),

    #[error(transparent)]
    Table(#[from] TableEncryptionError),

    #[error(transparent)]
    Aes(#[from] AesError),

    #[error(transparent)]
    Write(#[from] fmt::Error),

    #[error("AES-256 key must contain 32 bytes")]
    InvalidAesKey,

    #[error("AES-CBC IV must contain 16 bytes")]
    InvalidAesIv,

    #[error("XOR offset plus length exceeds usize")]
    XorRangeOverflow,

    #[error("Password length times three exceeds usize")]
    PasswordLengthOverflow,

    #[error("Table float key must not be empty")]
    EmptyFloatKey,

    #[error("Table float encryption exceeds i32")]
    FloatOverflow,

    #[error("Mersenne range width exceeds i32")]
    RandomRangeOverflow
}

#[diplomat::bridge]
pub mod ffi {
    use std::fmt::Write;

    use bacy::error::{HashError, TableEncryptionError};

    use crate::error::{Error, error};

    pub enum BacyErrorKind {
        InvalidArgument,
        Io,
        InvalidPath,
        Crc32Mismatch,
        Crc64Mismatch,
        Base64Decode,
        Utf16,
        StringConversion,
        AesDecrypt,
        Write
    }

    #[diplomat::opaque]
    pub struct BacyError(pub Error);

    impl BacyError {
        pub fn message(&self, output: &mut DiplomatWrite) -> Result<(), Box<Self>> {
            write!(output, "{}", self.0).map_err(error)
        }

        pub const fn kind(&self) -> BacyErrorKind {
            match &self.0 {
                Error::Hash(HashError::Io(_)) => BacyErrorKind::Io,
                Error::Hash(HashError::InvalidPath) => BacyErrorKind::InvalidPath,
                Error::Hash(HashError::Crc32Mismatch { .. }) => BacyErrorKind::Crc32Mismatch,
                Error::Hash(HashError::Crc64Mismatch { .. }) => BacyErrorKind::Crc64Mismatch,
                Error::Table(TableEncryptionError::Base64Decode(_)) => BacyErrorKind::Base64Decode,
                Error::Table(TableEncryptionError::FromUtf16Error(_)) => BacyErrorKind::Utf16,
                Error::Table(TableEncryptionError::StringConversionFailed) => {
                    BacyErrorKind::StringConversion
                }
                Error::Aes(_) => BacyErrorKind::AesDecrypt,
                Error::Write(_) => BacyErrorKind::Write,
                _ => BacyErrorKind::InvalidArgument
            }
        }
    }
}

pub fn error(error: impl Into<Error>) -> Box<ffi::BacyError> {
    Box::new(ffi::BacyError(error.into()))
}
