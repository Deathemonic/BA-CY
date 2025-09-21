use thiserror::Error;

#[derive(Error, Debug)]
pub enum MemoryPackError {
    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Invalid data length: {0}")]
    InvalidLength(i32),
}

#[derive(Error, Debug)]
pub enum TableZipError {
    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("{0}")]
    Base64Decode(#[from] base64::DecodeError),
}

#[derive(Error, Debug)]
pub enum HashError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum CatalogError {
    #[error("{0}")]
    MemoryPack(#[from] MemoryPackError),

    #[error("{0}")]
    Json(#[from] serde_json::Error),

    #[error("Failed to serialize catalog to JSON")]
    SerializationFailed,

    #[error("Failed to parse catalog from JSON")]
    DeserializationFailed,
}

#[derive(Error, Debug)]
pub enum TableEncryptionError {
    #[error("{0}")]
    Base64Decode(#[from] base64::DecodeError),
    
    #[error("String conversion failed")]
    StringConversionFailed,
}
