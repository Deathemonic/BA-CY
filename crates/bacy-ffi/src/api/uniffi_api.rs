//! # UniFFI Bindings
//!
//! Thin wrappers exposing `bacy`'s API to UniFFI-generated bindings
//! (Kotlin, Swift, Python, Ruby, C#, Go).
//!
//! **For Rust users:** depend on `bacy` directly and use its native API.
//! This module exists solely to adapt `bacy`'s Rust-native types
//! (`&Path`, fixed-size arrays, `thiserror` enums) into UniFFI-compatible
//! shapes (`String`, `Vec<u8>`, UniFFI error enums).

use std::path::Path;

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum HashError {
    #[error("I/O error: {0}")]
    Io(String),

    #[error("Invalid file path")]
    InvalidPath,

    #[error("Expected 0x{expected:08X}, got 0x{actual:08X}")]
    Mismatch { expected: u32, actual: u32 }
}

impl From<bacy::error::HashError> for HashError {
    fn from(e: bacy::error::HashError) -> Self {
        match e {
            bacy::error::HashError::Io(err) => HashError::Io(err.to_string()),
            bacy::error::HashError::InvalidPath => HashError::InvalidPath,
            bacy::error::HashError::Mismatch { expected, actual } => {
                HashError::Mismatch { expected, actual }
            }
        }
    }
}

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum TableEncryptionError {
    #[error("Base64 decode error: {0}")]
    Base64Decode(String),

    #[error("UTF-16 conversion error: {0}")]
    FromUtf16Error(String),

    #[error("String conversion failed")]
    StringConversionFailed
}

impl From<bacy::error::TableEncryptionError> for TableEncryptionError {
    fn from(e: bacy::error::TableEncryptionError) -> Self {
        match e {
            bacy::error::TableEncryptionError::Base64Decode(err) => {
                TableEncryptionError::Base64Decode(err.to_string())
            }
            bacy::error::TableEncryptionError::FromUtf16Error(err) => {
                TableEncryptionError::FromUtf16Error(err.to_string())
            }
            bacy::error::TableEncryptionError::StringConversionFailed => {
                TableEncryptionError::StringConversionFailed
            }
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct CrcResult {
    pub value: u32,
    pub hex: String
}

#[uniffi::export]
pub fn crc_compute_streaming(path: &str, buffer_size: u64) -> Result<u32, HashError> {
    let path_buf = Path::new(path);
    bacy::hash::crc::compute_streaming(path_buf, buffer_size as usize, None).map_err(Into::into)
}

#[uniffi::export]
pub fn crc_compute_bytes(buffer: &[u8]) -> u32 { bacy::hash::crc::compute_bytes(buffer, None) }

#[uniffi::export]
pub fn crc_compare(path: &str, expected_crc: u32) -> Result<(), HashError> {
    let path_buf = Path::new(path);
    bacy::hash::crc::compare(path_buf, expected_crc).map_err(Into::into)
}

#[uniffi::export]
pub fn crc_evaluate(data: &[u8]) -> CrcResult {
    let value = bacy::hash::crc::compute_bytes(data, None);
    CrcResult {
        value,
        hex: format!("{:08X}", value)
    }
}

#[uniffi::export]
pub fn crc_forge(file_path: &str, target_crc: u32) -> Result<(), HashError> {
    let manipulator = bacy::utils::crc_manipulator::CrcManipulator::new(file_path);
    manipulator.forge_crc(target_crc).map_err(Into::into)
}

#[uniffi::export]
pub fn crc_match_file(file_path: &str, target_file_path: &str) -> Result<(), HashError> {
    let manipulator = bacy::utils::crc_manipulator::CrcManipulator::new(file_path);
    let target_path = Path::new(target_file_path);
    manipulator.match_file(target_path).map_err(Into::into)
}

#[uniffi::export]
pub fn md5_to_hex_string(data: &[u8]) -> String { bacy::crypto::md5::to_hex_string(data) }

#[uniffi::export]
pub fn md5_compute_hash(source: &[u8]) -> Vec<u8> {
    bacy::crypto::md5::compute_hash(source).to_vec()
}

#[uniffi::export]
pub fn md5_compute_hash_hmac(source: &[u8], key: &[u8]) -> Vec<u8> {
    bacy::crypto::md5::compute_hash_hmac(source, key).to_vec()
}

#[uniffi::export]
pub fn md5_compute_hash_str(source: &str) -> String { bacy::crypto::md5::compute_hash_str(source) }

#[uniffi::export]
pub fn md5_compute_hash_str_hmac(source: &str, key: &str) -> String {
    bacy::crypto::md5::compute_hash_str_hmac(source, key)
}

#[uniffi::export]
pub fn md5_compute_digest(source: &str) -> u32 { bacy::crypto::md5::compute_digest(source) }

#[uniffi::export]
pub fn md5_compute_digest_hmac(source: &str, key: &str) -> u32 {
    bacy::crypto::md5::compute_digest_hmac(source, key)
}

#[uniffi::export]
pub fn md5_compute_digest64(source: &str) -> u64 { bacy::crypto::md5::compute_digest64(source) }

#[uniffi::export]
pub fn md5_compute_digest64_hmac(source: &str, key: &str) -> u64 {
    bacy::crypto::md5::compute_digest64_hmac(source, key)
}

#[uniffi::export]
pub fn md5_compute_head(source: &str) -> String { bacy::crypto::md5::compute_head(source) }

#[uniffi::export]
pub fn sha_compute(source: &[u8]) -> Vec<u8> { bacy::hash::sha::compute(source).to_vec() }

#[uniffi::export]
pub fn sha_compute_str(source: &str) -> Vec<u8> { bacy::hash::sha::compute_str(source).to_vec() }

#[uniffi::export]
pub fn xxhash_calculate_hash(data: &[u8]) -> u32 { bacy::hash::xxhash::calculate_hash(data) }

#[uniffi::export]
pub fn xxhash_calculate_hash_str(s: &str) -> u32 { bacy::hash::xxhash::calculate_hash_str(s) }

#[uniffi::export]
pub fn xxhash_calculate_hash64(data: &[u8]) -> u64 { bacy::hash::xxhash::calculate_hash64(data) }

#[uniffi::export]
pub fn xxhash_calculate_hash64_str(s: &str) -> u64 { bacy::hash::xxhash::calculate_hash64_str(s) }

#[uniffi::export]
pub fn xor_encrypt(mut data: Vec<u8>, offset: u64, length: u64) -> Vec<u8> {
    bacy::crypto::xor::encrypt(&mut data, offset as usize, length as usize);
    data
}

#[uniffi::export]
pub fn xor_encrypt_with_key(data: &[u8], key: &[u8]) -> Option<Vec<u8>> {
    bacy::crypto::xor::encrypt_with_key(data, key)
}

#[uniffi::export]
pub fn xor_exact(value: &[u8], key: &[u8]) -> Vec<u8> { bacy::crypto::xor::xor_exact(value, key) }

#[uniffi::export]
pub fn xor_inplace(mut data: Vec<u8>, key: &[u8]) -> Vec<u8> {
    bacy::crypto::xor::xor_inplace(&mut data, key);
    data
}

#[uniffi::export]
pub fn table_create_key(name: &str) -> Vec<u8> { bacy::crypto::table::create_key(name).to_vec() }

#[uniffi::export]
pub fn table_create_password(key: &str, length: u64) -> String {
    bacy::crypto::table::create_password(key, length as usize)
}

#[uniffi::export]
pub fn table_xor(name: &str, mut data: Vec<u8>) -> Vec<u8> {
    bacy::crypto::table::xor(name, &mut data);
    data
}

#[uniffi::export]
pub fn table_decrypt_i32(value: i32, key: &[u8]) -> i32 {
    bacy::crypto::table::decrypt_i32(value, key)
}

#[uniffi::export]
pub fn table_decrypt_i64(value: i64, key: &[u8]) -> i64 {
    bacy::crypto::table::decrypt_i64(value, key)
}

#[uniffi::export]
pub fn table_decrypt_u32(value: u32, key: &[u8]) -> u32 {
    bacy::crypto::table::decrypt_u32(value, key)
}

#[uniffi::export]
pub fn table_decrypt_u64(value: u64, key: &[u8]) -> u64 {
    bacy::crypto::table::decrypt_u64(value, key)
}

#[uniffi::export]
pub fn table_decrypt_f32(value: f32, key: &[u8]) -> f32 {
    bacy::crypto::table::decrypt_f32(value, key)
}

#[uniffi::export]
pub fn table_decrypt_f64(value: f64, key: &[u8]) -> f64 {
    bacy::crypto::table::decrypt_f64(value, key)
}

#[uniffi::export]
pub fn table_decrypt_string(value: &str, key: &[u8]) -> Result<String, TableEncryptionError> {
    bacy::crypto::table::decrypt_string(value, key).map_err(Into::into)
}

#[uniffi::export]
pub fn table_encrypt_f32(value: f32, key: &[u8]) -> f32 {
    bacy::crypto::table::encrypt_f32(value, key)
}

#[uniffi::export]
pub fn table_encrypt_f64(value: f64, key: &[u8]) -> f64 {
    bacy::crypto::table::encrypt_f64(value, key)
}

#[uniffi::export]
pub fn table_encrypt_string(value: &str, key: &[u8]) -> String {
    bacy::crypto::table::encrypt_string(value, key)
}

#[uniffi::export]
pub fn get_file_path(path: &str, crc: Option<i64>, no_hash: bool, to_lower: bool) -> String {
    bacy::utils::strategy::get_file_path(path, crc, no_hash, to_lower).to_string_lossy().to_string()
}
