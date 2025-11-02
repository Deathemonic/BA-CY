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

use crate::hash::CrcResult;
pub use crate::error::{CatalogError, HashError, TableEncryptionError, TableZipError};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

macro_rules! impl_bindings_struct {
    (
        $(#[$meta:meta])*
        $name:ident {
            $($field:ident: $ty:ty),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $name {
            $(pub $field: $ty),*
        }

        impl From<crate::catalog::$name> for $name {
            fn from(value: crate::catalog::$name) -> Self {
                Self {
                    $($field: value.$field),*
                }
            }
        }

        impl From<$name> for crate::catalog::$name {
            fn from(value: $name) -> Self {
                Self {
                    $($field: value.$field),*
                }
            }
        }
    };
}

impl_bindings_struct!(
    Media {
        path: String,
        file_name: String,
        bytes: i64,
        crc: i64,
        is_prologue: bool,
        is_split_download: bool,
        media_type: i32,
    }
);

impl_bindings_struct!(
    Table {
        name: String,
        size: i64,
        crc: i64,
        is_in_build: bool,
        is_changed: bool,
        is_prologue: bool,
        is_split_download: bool,
        includes: Vec<String>,
    }
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaCatalog {
    pub table: HashMap<String, Media>,
}

impl From<crate::catalog::MediaCatalog> for MediaCatalog {
    fn from(catalog: crate::catalog::MediaCatalog) -> Self {
        MediaCatalog {
            table: catalog.table.into_iter().map(|(k, v)| (k, v.into())).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCatalog {
    pub table: HashMap<String, Table>,
}

impl From<crate::catalog::TableCatalog> for TableCatalog {
    fn from(catalog: crate::catalog::TableCatalog) -> Self {
        TableCatalog {
            table: catalog.table.into_iter().map(|(k, v)| (k, v.into())).collect(),
        }
    }
}

pub fn calculate_crc32(path: String) -> Result<u32, HashError> {
    let path_buf = std::path::PathBuf::from(path);
    crate::hash::calculate_crc32(&path_buf)
}

pub fn calculate_md5(path: String) -> Result<String, HashError> {
    let path_buf = std::path::PathBuf::from(path);
    crate::hash::calculate_md5(&path_buf)
}

pub fn encrypt_name(filename: String, crc: i64) -> Result<String, HashError> {
    crate::hash::encrypt_name(&filename, crc)
}

pub fn evaluate_crc32(data: Vec<u8>) -> CrcResult {
    crate::hash::evaluate_crc32(&data)
}

pub fn calculate_xxhash(bytes: Vec<u8>, bit64: bool, endian: bool) -> u64 {
    crate::hash::calculate_xxhash(&bytes, bit64, endian)
}

pub fn deserialize_media_catalog(bytes: Vec<u8>) -> Result<MediaCatalog, CatalogError> {
    Ok(crate::catalog::deserialize_media_catalog(&bytes)?.into())
}

pub fn deserialize_table_catalog(bytes: Vec<u8>) -> Result<TableCatalog, CatalogError> {
    Ok(crate::catalog::deserialize_table_catalog(&bytes)?.into())
}

pub fn media_catalog_to_json(catalog: MediaCatalog) -> Result<String, CatalogError> {
    Ok(serde_json::to_string_pretty(&catalog.table)?)
}

pub fn table_catalog_to_json(catalog: TableCatalog) -> Result<String, CatalogError> {
    Ok(serde_json::to_string_pretty(&catalog.table)?)
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

pub fn convert_string(value: String, key: Vec<u8>) -> Result<String, TableEncryptionError> {
    crate::table_encryption::convert_string(&value, &key)
}

pub fn encrypt_string(value: String, key: Vec<u8>) -> Result<String, TableEncryptionError> {
    crate::table_encryption::encrypt_string(&value, &key)
}

pub fn extract_zip_file(
    zip_data: Vec<u8>,
    filename: String,
    file_to_extract: String,
) -> Result<Vec<u8>, TableZipError> {
    let mut zip_file = crate::table_zip::TableZipFile::new(zip_data, filename.as_bytes())?;
    Ok(zip_file.get_by_name(&file_to_extract)?)
}

pub fn extract_all_zip_files(
    zip_data: Vec<u8>,
    filename: String,
) -> Result<Vec<ZipFileEntry>, TableZipError> {
    let mut zip_file = crate::table_zip::TableZipFile::new(zip_data, filename.as_bytes())?;
    let files = zip_file.extract_all()?;

    Ok(files
        .into_iter()
        .map(|(name, data)| ZipFileEntry { name, data })
        .collect())
}

pub use crate::table_encryption::{use_encryption, set_use_encryption};

pub fn forge_crc(file_path: String, target_crc: u32) -> Result<(), HashError> {
    let manipulator = crate::crc::CrcManipulator::new(file_path);
    Ok(manipulator.forge_crc(target_crc)?)
}

pub fn match_crc(file_path: String, target_file_path: String) -> Result<(), HashError> {
    let manipulator = crate::crc::CrcManipulator::new(file_path);
    Ok(manipulator.match_file(std::path::Path::new(&target_file_path))?)
}

#[derive(Debug, Clone)]
pub struct ZipFileEntry {
    pub name: String,
    pub data: Vec<u8>,
}
