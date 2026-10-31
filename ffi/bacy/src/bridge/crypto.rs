#[allow(
    clippy::unnecessary_box_returns,
    reason = "Diplomat requires boxed ownership for opaque ABI results"
)]
#[diplomat::bridge]
pub mod ffi {
    use std::fmt::Write;
    use std::path::Path;

    use bacy::crypto::{aes, md5, table, xor};

    use crate::bridge::bytes::ffi::BacyBytes;
    use crate::error::ffi::BacyError;
    use crate::error::{Error, error};

    const PASSWORD_BYTE_FACTOR: usize = 3;
    const FLOAT_SCALE: f32 = 10_000.0;
    const DOUBLE_SCALE: f64 = 1_000_000.0;
    const MULTIPLIER_MODULUS: u8 = 10;
    const FALLBACK_MULTIPLIER: i32 = 7;

    #[diplomat::opaque]
    pub struct BacyAes;

    #[diplomat::opaque]
    pub struct BacyMd5;

    #[diplomat::opaque]
    pub struct BacyXor;

    #[diplomat::opaque]
    pub struct BacyTable;

    impl BacyAes {
        pub fn decrypt(
            data: &[u8],
            key: &[u8],
            iv: &[u8]
        ) -> Result<Box<BacyBytes>, Box<BacyError>> {
            let key = key.try_into().map_err(|_| error(Error::InvalidAesKey))?;
            let iv = iv.try_into().map_err(|_| error(Error::InvalidAesIv))?;

            aes::decrypt(data, key, iv).map(|bytes| Box::new(BacyBytes(bytes))).map_err(error)
        }
    }

    impl BacyMd5 {
        pub fn to_hex_string(
            data: &[u8],
            output: &mut DiplomatWrite
        ) -> Result<(), Box<BacyError>> {
            output.write_str(&md5::to_hex_string(data)).map_err(error)
        }

        pub fn compute_hash(source: &[u8]) -> Box<BacyBytes> {
            Box::new(BacyBytes(md5::compute_hash(source).to_vec()))
        }

        pub fn compute_hash_hmac(source: &[u8], key: &[u8]) -> Box<BacyBytes> {
            Box::new(BacyBytes(md5::compute_hash_hmac(source, key).to_vec()))
        }

        pub fn compute_hash_str(
            source: &str,
            output: &mut DiplomatWrite
        ) -> Result<(), Box<BacyError>> {
            output.write_str(&md5::compute_hash_str(source)).map_err(error)
        }

        pub fn compute_hash_str_hmac(
            source: &str,
            key: &str,
            output: &mut DiplomatWrite
        ) -> Result<(), Box<BacyError>> {
            output.write_str(&md5::compute_hash_str_hmac(source, key)).map_err(error)
        }

        pub fn compute_streaming(
            path: &str,
            buffer_size: usize,
            output: &mut DiplomatWrite
        ) -> Result<(), Box<BacyError>> {
            let hex = md5::compute_streaming(Path::new(path), buffer_size).map_err(error)?;

            output.write_str(&hex).map_err(error)
        }

        pub fn compute_digest(source: &str) -> u32 { md5::compute_digest(source) }

        pub fn compute_digest_hmac(source: &str, key: &str) -> u32 {
            md5::compute_digest_hmac(source, key)
        }

        pub fn compute_digest64(source: &str) -> u64 { md5::compute_digest64(source) }

        pub fn compute_digest64_hmac(source: &str, key: &str) -> u64 {
            md5::compute_digest64_hmac(source, key)
        }

        pub fn compute_head(
            source: &str,
            output: &mut DiplomatWrite
        ) -> Result<(), Box<BacyError>> {
            output.write_str(&md5::compute_head(source)).map_err(error)
        }
    }

    impl BacyXor {
        pub fn encrypt(
            data: &mut [u8],
            offset: usize,
            length: usize
        ) -> Result<(), Box<BacyError>> {
            offset.checked_add(length).ok_or_else(|| error(Error::XorRangeOverflow))?;

            xor::encrypt(data, offset, length);
            Ok(())
        }

        pub fn encrypt_with_key(data: &[u8], key: &[u8]) -> Option<Box<BacyBytes>> {
            xor::encrypt_with_key(data, key).map(|bytes| Box::new(BacyBytes(bytes)))
        }

        pub fn xor_exact(value: &[u8], key: &[u8]) -> Box<BacyBytes> {
            Box::new(BacyBytes(xor::xor_exact(value, key)))
        }

        pub fn xor_inplace(data: &mut [u8], key: &[u8]) { xor::xor_inplace(data, key); }
    }

    impl BacyTable {
        pub fn create_key(name: &str) -> Box<BacyBytes> {
            Box::new(BacyBytes(table::create_key(name).to_vec()))
        }

        pub fn create_password(
            key: &str,
            length: usize,
            output: &mut DiplomatWrite
        ) -> Result<(), Box<BacyError>> {
            length
                .checked_mul(PASSWORD_BYTE_FACTOR)
                .ok_or_else(|| error(Error::PasswordLengthOverflow))?;

            output.write_str(&table::create_password(key, length)).map_err(error)
        }

        pub fn xor(name: &str, data: &mut [u8]) { table::xor(name, data); }

        pub fn decrypt_i32(value: i32, key: &[u8]) -> i32 { table::decrypt_i32(value, key) }

        pub fn decrypt_i64(value: i64, key: &[u8]) -> i64 { table::decrypt_i64(value, key) }

        pub fn decrypt_u32(value: u32, key: &[u8]) -> u32 { table::decrypt_u32(value, key) }

        pub fn decrypt_u64(value: u64, key: &[u8]) -> u64 { table::decrypt_u64(value, key) }

        pub fn decrypt_f32(value: f32, key: &[u8]) -> Result<f32, Box<BacyError>> {
            if key.is_empty() {
                return Err(error(Error::EmptyFloatKey));
            }

            Ok(table::decrypt_f32(value, key))
        }

        pub fn decrypt_f64(value: f64, key: &[u8]) -> Result<f64, Box<BacyError>> {
            if key.is_empty() {
                return Err(error(Error::EmptyFloatKey));
            }

            Ok(table::decrypt_f64(value, key))
        }

        pub fn encrypt_f32(value: f32, key: &[u8]) -> Result<f32, Box<BacyError>> {
            validate_float((value * FLOAT_SCALE) as i32, key)?;

            Ok(table::encrypt_f32(value, key))
        }

        pub fn encrypt_f64(value: f64, key: &[u8]) -> Result<f64, Box<BacyError>> {
            validate_float((value * DOUBLE_SCALE) as i32, key)?;

            Ok(table::encrypt_f64(value, key))
        }

        pub fn decrypt_string(
            value: &str,
            key: &[u8],
            output: &mut DiplomatWrite
        ) -> Result<(), Box<BacyError>> {
            let value = table::decrypt_string(value, key).map_err(error)?;

            output.write_str(&value).map_err(error)
        }

        pub fn encrypt_string(
            value: &str,
            key: &[u8],
            output: &mut DiplomatWrite
        ) -> Result<(), Box<BacyError>> {
            output.write_str(&table::encrypt_string(value, key)).map_err(error)
        }
    }

    fn validate_float(scaled: i32, key: &[u8]) -> Result<(), Box<BacyError>> {
        let Some(&byte) = key.first() else {
            return Err(error(Error::EmptyFloatKey));
        };

        let remainder = i32::from(byte % MULTIPLIER_MODULUS);
        let magnitude = if remainder <= 1 { FALLBACK_MULTIPLIER } else { remainder };
        let multiplier = if byte & 1 != 0 { -magnitude } else { magnitude };

        scaled.checked_mul(multiplier).ok_or_else(|| error(Error::FloatOverflow))?;

        Ok(())
    }
}
