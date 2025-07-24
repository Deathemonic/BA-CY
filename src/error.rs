#[derive(thiserror::Error, Debug)]
pub enum BacyError {
    #[error("IO error: {0}")]
    IoError(String),
    #[error("Other error: {0}")]
    Other(String),
}

impl From<std::io::Error> for BacyError {
    fn from(err: std::io::Error) -> Self {
        BacyError::IoError(err.to_string())
    }
}

impl From<anyhow::Error> for BacyError {
    fn from(err: anyhow::Error) -> Self {
        BacyError::Other(err.to_string())
    }
}