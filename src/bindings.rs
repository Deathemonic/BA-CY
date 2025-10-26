//! # WARNING: Internal UniFFI Bindings Module
//!
//! This module contains UniFFI binding wrappers and should NOT be used directly in Rust code.
//!
//! **For Rust users:** Use the functions and types from the main library modules instead:
//! - `bacy::hash::*` for hash functions
//! - `bacy::catalog::*` for catalog operations
//! - `bacy::crc_service::*` for CRC manipulation
//! - `bacy::table_encryption::*` for encryption
//! - `bacy::table_zip::*` for ZIP operations
//!
//! **For other languages (Python, Swift, etc.):** Use the generated bindings from UniFFI.
//!
//! This module exists solely to provide UniFFI-compatible wrappers that convert between
//! Rust types and UniFFI-compatible types (e.g., `&str` → `String`, `&[u8]` → `Vec<u8>`

use crate::catalog::{MediaCatalog, TableCatalog};
use crate::error::{CatalogError, HashError, TableEncryptionError, TableZipError};
use crate::hash::CrcResult;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Generic(String),
}

impl From<HashError> for Error {
    fn from(err: HashError) -> Self {
        Error::Generic(err.to_string())
    }
}

impl From<CatalogError> for Error {
    fn from(err: CatalogError) -> Self {
        Error::Generic(err.to_string())
    }
}

impl From<TableEncryptionError> for Error {
    fn from(err: TableEncryptionError) -> Self {
        Error::Generic(err.to_string())
    }
}

impl From<TableZipError> for Error {
    fn from(err: TableZipError) -> Self {
        Error::Generic(err.to_string())
    }
}

impl From<memorypack::MemoryPackError> for Error {
    fn from(err: memorypack::MemoryPackError) -> Self {
        Error::Generic(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Generic(err.to_string())
    }
}

pub fn calculate_crc32(path: String) -> Result<u32, Error> {
    let path_buf = PathBuf::from(path);
    Ok(crate::hash::calculate_crc32(&path_buf)?)
}

pub fn evaluate_crc32(data: Vec<u8>) -> CrcResult {
    crate::hash::evaluate_crc32(&data)
}

pub fn calculate_md5(path: String) -> Result<String, Error> {
    let path_buf = PathBuf::from(path);
    Ok(crate::hash::calculate_md5(&path_buf)?)
}

pub fn calculate_xxhash(bytes: Vec<u8>, bit64: bool, endian: bool) -> u64 {
    crate::hash::calculate_xxhash(&bytes, bit64, endian)
}

pub fn encrypt_name(filename: String, crc: i64) -> Result<String, Error> {
    Ok(crate::hash::encrypt_name(&filename, crc)?)
}

pub fn deserialize_media_catalog(bytes: Vec<u8>, base_url: String) -> Result<MediaCatalog, Error> {
    Ok(MediaCatalog::deserialize(&bytes, &base_url)?)
}

pub fn deserialize_table_catalog(bytes: Vec<u8>, base_url: String) -> Result<TableCatalog, Error> {
    Ok(TableCatalog::deserialize(&bytes, &base_url)?)
}

pub fn media_catalog_to_json(catalog: MediaCatalog) -> Result<String, Error> {
    Ok(catalog.to_json()?)
}

pub fn table_catalog_to_json(catalog: TableCatalog) -> Result<String, Error> {
    Ok(catalog.to_json()?)
}

pub fn xor_str(value: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    crate::table_encryption::xor_str(&value, &key)
}

pub fn xor_data(name: String, data: Vec<u8>) -> Vec<u8> {
    crate::table_encryption::xor(&name, &data)
}

pub fn xor_bytes(value: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    crate::table_encryption::xor_bytes(&value, &key)
}

pub fn xor_int32(value: i32, key: Vec<u8>) -> i32 {
    crate::table_encryption::xor_int32(value, &key)
}

pub fn xor_int64(value: i64, key: Vec<u8>) -> i64 {
    crate::table_encryption::xor_int64(value, &key)
}

pub fn xor_uint32(value: u32, key: Vec<u8>) -> u32 {
    crate::table_encryption::xor_uint32(value, &key)
}

pub fn xor_uint64(value: u64, key: Vec<u8>) -> u64 {
    crate::table_encryption::xor_uint64(value, &key)
}

pub fn convert_int(value: i32, key: Vec<u8>) -> i32 {
    crate::table_encryption::convert_int(value, &key)
}

pub fn convert_long(value: i64, key: Vec<u8>) -> i64 {
    crate::table_encryption::convert_long(value, &key)
}

pub fn convert_uint(value: u32, key: Vec<u8>) -> u32 {
    crate::table_encryption::convert_uint(value, &key)
}

pub fn convert_ulong(value: u64, key: Vec<u8>) -> u64 {
    crate::table_encryption::convert_ulong(value, &key)
}

pub fn convert_float(value: f32, key: Vec<u8>) -> f32 {
    crate::table_encryption::convert_float(value, &key)
}

pub fn convert_double(value: f64, key: Vec<u8>) -> f64 {
    crate::table_encryption::convert_double(value, &key)
}

pub fn encrypt_float(value: f32, key: Vec<u8>) -> f32 {
    crate::table_encryption::encrypt_float(value, &key)
}

pub fn encrypt_double(value: f64, key: Vec<u8>) -> f64 {
    crate::table_encryption::encrypt_double(value, &key)
}

pub fn create_key(bytes: Vec<u8>) -> Vec<u8> {
    crate::table_encryption::create_key(&bytes).to_vec()
}

pub fn convert_string(value: String, key: Vec<u8>) -> Result<String, Error> {
    Ok(crate::table_encryption::convert_string(&value, &key)?)
}

pub fn encrypt_string(value: String, key: Vec<u8>) -> Result<String, Error> {
    Ok(crate::table_encryption::encrypt_string(&value, &key)?)
}

pub fn extract_zip_file(
    zip_data: Vec<u8>,
    filename: String,
    file_to_extract: String,
) -> Result<Vec<u8>, Error> {
    let mut zip_file = crate::table_zip::TableZipFile::new(zip_data, filename.as_bytes())?;
    Ok(zip_file.get_by_name(&file_to_extract)?)
}

pub fn extract_all_zip_files(
    zip_data: Vec<u8>,
    filename: String,
) -> Result<Vec<ZipFileEntry>, Error> {
    let mut zip_file = crate::table_zip::TableZipFile::new(zip_data, filename.as_bytes())?;
    let files = zip_file.extract_all()?;

    Ok(files
        .into_iter()
        .map(|(name, data)| ZipFileEntry { name, data })
        .collect())
}

pub fn use_encryption() -> bool {
    crate::table_encryption::use_encryption()
}

pub fn set_use_encryption(enabled: bool) {
    crate::table_encryption::set_use_encryption(enabled)
}

#[derive(Debug, Clone)]
pub struct ZipFileEntry {
    pub name: String,
    pub data: Vec<u8>,
}
