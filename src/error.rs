use thiserror::Error;

#[derive(Error, Debug)]
pub enum MemoryPackError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Invalid data length: {0}")]
    InvalidLength(i32),
}

#[derive(Error, Debug)]
pub enum TableZipError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),

    #[error(transparent)]
    Base64Decode(#[from] base64::DecodeError),

    #[error(transparent)]
    EncodeSlice(#[from] base64::EncodeSliceError)
}

#[derive(Error, Debug)]
pub enum HashError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum CatalogError {
    #[error(transparent)]
    MemoryPack(#[from] MemoryPackError),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error("Failed to serialize catalog to JSON")]
    SerializationFailed,

    #[error("Failed to parse catalog from JSON")]
    DeserializationFailed,
}

#[derive(Error, Debug)]
pub enum TableEncryptionError {
    #[error(transparent)]
    Base64Decode(#[from] base64::DecodeError),
    
    #[error("String conversion failed")]
    StringConversionFailed,
}
