#[allow(
    clippy::unnecessary_box_returns,
    reason = "Diplomat requires boxed ownership for opaque ABI results"
)]
#[diplomat::bridge]
pub mod ffi {
    use std::fmt::Write;
    use std::path::Path;

    use bacy::hash::{crc, crc64, sha, xxhash};

    use crate::bridge::bytes::ffi::BacyBytes;
    use crate::error::error;
    use crate::error::ffi::BacyError;

    #[diplomat::opaque]
    pub struct BacyCrc;

    pub struct BacyCrcResult {
        pub value: u32
    }

    #[diplomat::opaque]
    pub struct BacyCrc64;

    #[diplomat::opaque]
    pub struct BacySha;

    #[diplomat::opaque]
    pub struct BacyXxHash;

    impl BacyCrc {
        pub fn compute_bytes(data: &[u8]) -> u32 { crc::compute_bytes(data, None) }

        pub fn compute_bytes_with_suffix(data: &[u8], suffix: &[u8]) -> u32 {
            crc::compute_bytes(data, Some(suffix))
        }

        pub fn compute_streaming(path: &str, buffer_size: usize) -> Result<u32, Box<BacyError>> {
            crc::compute_streaming(Path::new(path), buffer_size).map_err(error)
        }

        pub fn compare(path: &str, expected_crc: u32) -> Result<(), Box<BacyError>> {
            crc::compare(Path::new(path), expected_crc).map_err(error)
        }

        pub fn pad_zeros(value: u32) -> u32 { crc::pad_zeros(value) }

        pub fn evaluate(data: &[u8]) -> BacyCrcResult {
            BacyCrcResult {
                value: Self::compute_bytes(data)
            }
        }
    }

    impl BacyCrcResult {
        pub fn hex(&self, output: &mut DiplomatWrite) -> Result<(), Box<BacyError>> {
            write!(output, "{:08X}", self.value).map_err(error)
        }
    }

    impl BacyCrc64 {
        pub fn compute_bytes(data: &[u8]) -> u64 { crc64::compute_bytes(data) }

        pub fn compute_streaming(path: &str, buffer_size: usize) -> Result<u64, Box<BacyError>> {
            crc64::compute_streaming(Path::new(path), buffer_size).map_err(error)
        }

        pub fn compare(path: &str, expected_crc: u64) -> Result<(), Box<BacyError>> {
            crc64::compare(Path::new(path), expected_crc).map_err(error)
        }
    }

    impl BacySha {
        pub fn compute(source: &[u8]) -> Box<BacyBytes> {
            Box::new(BacyBytes(sha::compute(source).to_vec()))
        }

        pub fn compute_str(source: &str) -> Box<BacyBytes> {
            Box::new(BacyBytes(sha::compute_str(source).to_vec()))
        }

        pub fn compute_sha1(source: &[u8]) -> Box<BacyBytes> {
            Box::new(BacyBytes(sha::compute_sha1(source).to_vec()))
        }

        pub fn compute_sha1_hex(
            source: &[u8],
            output: &mut DiplomatWrite
        ) -> Result<(), Box<BacyError>> {
            output.write_str(&sha::compute_sha1_hex(source)).map_err(error)
        }

        pub fn compute_streaming(
            path: &str,
            buffer_size: usize,
            output: &mut DiplomatWrite
        ) -> Result<(), Box<BacyError>> {
            let hex = sha::compute_streaming(Path::new(path), buffer_size).map_err(error)?;

            output.write_str(&hex).map_err(error)
        }
    }

    impl BacyXxHash {
        pub fn calculate_hash(data: &[u8]) -> u32 { xxhash::calculate_hash(data) }

        pub fn calculate_hash_str(source: &str) -> u32 { xxhash::calculate_hash_str(source) }

        pub fn calculate_hash64(data: &[u8]) -> u64 { xxhash::calculate_hash64(data) }

        pub fn calculate_hash64_str(source: &str) -> u64 { xxhash::calculate_hash64_str(source) }
    }
}
