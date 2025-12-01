//! # WARNING: Internal UniFFI Bindings Module
//!
//! This module contains UniFFI binding wrappers and should NOT be used directly in Rust code.
//!
//! **For Rust users:** Use the functions and types from the main library modules instead:
//! - `bacy::hash::crc::*` for CRC functions
//! - `bacy::hash::xxhash::*` for xxHash functions
//! - `bacy::crypto::md5::*` for MD5 functions
//! - `bacy::crypto::xor::*` for XOR operations
//! - `bacy::crypto::table::*` for table encryption
//! - `bacy::utils::crc_manipulator::*` for CRC manipulation
//! - `bacy::utils::strategy::*` for file path strategies
//!
//! **For other languages (Python, Swift, etc.):** Use the generated bindings from UniFFI.
//!
//! This module exists solely to provide UniFFI-compatible wrappers that convert between
//! Rust types and UniFFI-compatible types (e.g., `&str` → `String`, `&[u8]` → `Vec<u8>`)

use std::path::Path;

pub use crate::error::{HashError, TableEncryptionError};

#[derive(Debug, Clone)]
pub struct CrcResult {
    pub value: u32,
    pub hex: String,
}

pub fn crc_compute_streaming(path: String, buffer_size: u64) -> Result<u32, HashError> {
    let path_buf = Path::new(&path);
    crate::hash::crc::compute_streaming(path_buf, buffer_size as usize)
}

#[inline]
pub fn crc_compute_bytes(buffer: Vec<u8>) -> u32 {
    crate::hash::crc::compute_bytes(&buffer)
}

pub fn crc_compare(path: String, expected_crc: u32) -> Result<(), HashError> {
    let path_buf = Path::new(&path);
    crate::hash::crc::compare(path_buf, expected_crc)
}

pub fn crc_evaluate(data: Vec<u8>) -> CrcResult {
    let value = crate::hash::crc::compute_bytes(&data);
    CrcResult {
        value,
        hex: format!("{:08X}", value),
    }
}

pub fn crc_forge(file_path: String, target_crc: u32) -> Result<(), HashError> {
    let manipulator = crate::utils::crc_manipulator::CrcManipulator::new(file_path);
    manipulator.forge_crc(target_crc)
}

pub fn crc_match_file(file_path: String, target_file_path: String) -> Result<(), HashError> {
    let manipulator = crate::utils::crc_manipulator::CrcManipulator::new(file_path);
    let target_path = Path::new(&target_file_path);
    manipulator.match_file(target_path)
}

#[inline]
pub fn md5_to_hex_string(data: Vec<u8>) -> String {
    crate::crypto::md5::to_hex_string(&data)
}

#[inline]
pub fn md5_compute_hash(source: Vec<u8>) -> Vec<u8> {
    crate::crypto::md5::compute_hash(&source).to_vec()
}

#[inline]
pub fn md5_compute_hash_hmac(source: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    crate::crypto::md5::compute_hash_hmac(&source, &key).to_vec()
}

#[inline]
pub fn md5_compute_hash_str(source: String) -> String {
    crate::crypto::md5::compute_hash_str(&source)
}

#[inline]
pub fn md5_compute_hash_str_hmac(source: String, key: String) -> String {
    crate::crypto::md5::compute_hash_str_hmac(&source, &key)
}

#[inline]
pub fn md5_compute_digest(source: String) -> u32 {
    crate::crypto::md5::compute_digest(&source)
}

#[inline]
pub fn md5_compute_digest_hmac(source: String, key: String) -> u32 {
    crate::crypto::md5::compute_digest_hmac(&source, &key)
}

#[inline]
pub fn md5_compute_digest64(source: String) -> u64 {
    crate::crypto::md5::compute_digest64(&source)
}

#[inline]
pub fn md5_compute_digest64_hmac(source: String, key: String) -> u64 {
    crate::crypto::md5::compute_digest64_hmac(&source, &key)
}

#[inline]
pub fn md5_compute_head(source: String) -> String {
    crate::crypto::md5::compute_head(&source)
}

#[inline]
pub fn xxhash_set_use_big_endian(value: bool) {
    crate::hash::xxhash::set_use_big_endian(value);
}

#[inline]
pub fn xxhash_get_use_big_endian() -> bool {
    crate::hash::xxhash::get_use_big_endian()
}

#[inline]
pub fn xxhash_calculate_hash(data: Vec<u8>) -> u32 {
    crate::hash::xxhash::calculate_hash(&data)
}

#[inline]
pub fn xxhash_calculate_hash_str(s: String) -> u32 {
    crate::hash::xxhash::calculate_hash_str(&s)
}

#[inline]
pub fn xxhash_calculate_hash64(data: Vec<u8>) -> u64 {
    crate::hash::xxhash::calculate_hash64(&data)
}

#[inline]
pub fn xxhash_calculate_hash64_str(s: String) -> u64 {
    crate::hash::xxhash::calculate_hash64_str(&s)
}

pub fn xor_encrypt(mut data: Vec<u8>, offset: u64, length: u64) {
    crate::crypto::xor::encrypt(&mut data, offset as usize, length as usize);
}

pub fn xor_encrypt_with_key(data: Vec<u8>, key: Vec<u8>) -> Option<Vec<u8>> {
    crate::crypto::xor::encrypt_with_key(&data, &key)
}

#[inline]
pub fn xor_exact(value: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    crate::crypto::xor::xor_exact(&value, &key)
}

pub fn xor_inplace_bytes(mut data: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    crate::crypto::xor::xor_inplace(&mut data, &key);
    data
}

#[inline]
pub fn table_create_key(name: String) -> Vec<u8> {
    crate::crypto::table::create_key(&name).to_vec()
}

#[inline]
pub fn table_create_password(key: String, length: u64) -> String {
    crate::crypto::table::create_password(&key, length as usize)
}

pub fn table_xor(name: String, mut data: Vec<u8>) -> Vec<u8> {
    crate::crypto::table::xor(&name, &mut data);
    data
}

#[inline]
pub fn table_decrypt_i32(value: i32, key: Vec<u8>) -> i32 {
    crate::crypto::table::decrypt_i32(value, &key)
}

#[inline]
pub fn table_decrypt_i64(value: i64, key: Vec<u8>) -> i64 {
    crate::crypto::table::decrypt_i64(value, &key)
}

#[inline]
pub fn table_decrypt_u32(value: u32, key: Vec<u8>) -> u32 {
    crate::crypto::table::decrypt_u32(value, &key)
}

#[inline]
pub fn table_decrypt_u64(value: u64, key: Vec<u8>) -> u64 {
    crate::crypto::table::decrypt_u64(value, &key)
}

#[inline]
pub fn table_decrypt_f32(value: f32, key: Vec<u8>) -> f32 {
    crate::crypto::table::decrypt_f32(value, &key)
}

#[inline]
pub fn table_decrypt_f64(value: f64, key: Vec<u8>) -> f64 {
    crate::crypto::table::decrypt_f64(value, &key)
}

pub fn table_decrypt_string(value: String, key: Vec<u8>) -> Result<String, TableEncryptionError> {
    crate::crypto::table::decrypt_string(&value, &key)
}

#[inline]
pub fn table_encrypt_f32(value: f32, key: Vec<u8>) -> f32 {
    crate::crypto::table::encrypt_f32(value, &key)
}

#[inline]
pub fn table_encrypt_f64(value: f64, key: Vec<u8>) -> f64 {
    crate::crypto::table::encrypt_f64(value, &key)
}

#[inline]
pub fn table_encrypt_string(value: String, key: Vec<u8>) -> String {
    crate::crypto::table::encrypt_string(&value, &key)
}

pub fn get_file_path(path: String, crc: Option<i64>, no_hash: bool, to_lower: bool) -> String {
    let result = crate::utils::strategy::get_file_path(&path, crc, no_hash, to_lower);
    result.to_string_lossy().to_string()
}
