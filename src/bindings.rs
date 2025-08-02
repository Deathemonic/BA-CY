//! # WARNING: Internal UniFFI Bindings Module
//! 
//! This module contains UniFFI binding wrappers and should NOT be used directly in Rust code.
//! 
//! **For Rust users:** Use the functions and types from the main library modules instead:
//! - `bacy::hash::*` for hash functions
//! - `bacy::catalog::*` for catalog operations  
//! - `bacy::crc_service::*` for CRC manipulation
//! - `bacy::table_encryption::table_encryption_service::*` for encryption
//! - `bacy::table_zip::*` for ZIP operations
//! 
//! **For other languages (Python, Swift, etc.):** Use the generated bindings from UniFFI.
//! 
//! This module exists solely to provide UniFFI-compatible wrappers that convert between
//! Rust types and UniFFI-compatible types (e.g., `&str` → `String`, `&[u8]` → `Vec<u8>`

use crate::hash::CrcResult;
use crate::catalog::{Media, Table};
use anyhow::Result;

use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Generic(String),
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::Generic(err.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct MediaCatalog {
    pub table: HashMap<String, Media>,
    pub base_url: String,
}

#[derive(Debug, Clone)]
pub struct TableCatalog {
    pub table: HashMap<String, Table>,
    pub base_url: String,
}

pub fn calculate_crc32(path: String) -> Result<u32, Error> {
    let path_buf = PathBuf::from(path);
    crate::hash::calculate_crc32(&path_buf).map_err(Error::from)
}

pub fn evaluate_crc32(data: Vec<u8>) -> CrcResult {
    crate::hash::evaluate_crc32(&data)
}

pub fn calculate_md5(path: String) -> Result<String, Error> {
    let path_buf = PathBuf::from(path);
    crate::hash::calculate_md5(&path_buf).map_err(Error::from)
}

pub fn calculate_xxhash(bytes: Vec<u8>, bit64: bool, endian: bool) -> u64 {
    crate::hash::calculate_xxhash(&bytes, bit64, endian)
}

pub fn encrypt_name(filename: String, crc: i64) -> Result<String, Error> {
    crate::hash::encrypt_name(&filename, crc).map_err(Error::from)
}

pub fn deserialize_media_catalog(bytes: Vec<u8>, base_url: String) -> Result<MediaCatalog, Error> {
    let catalog = crate::catalog::MediaCatalog::deserialize(&bytes, &base_url).map_err(Error::from)?;
    Ok(MediaCatalog {
        table: catalog.get_table().clone(),
        base_url: catalog.get_base_url().to_string(),
    })
}

pub fn deserialize_table_catalog(bytes: Vec<u8>, base_url: String) -> Result<TableCatalog, Error> {
    let catalog = crate::catalog::TableCatalog::deserialize(&bytes, &base_url).map_err(Error::from)?;
    Ok(TableCatalog {
        table: catalog.get_table().clone(),
        base_url: catalog.get_base_url().to_string(),
    })
}

pub fn media_catalog_to_json(wrapper: MediaCatalog) -> Result<String, Error> {
    let catalog = crate::catalog::MediaCatalog::new(wrapper.table, &wrapper.base_url);
    catalog.to_json().map_err(Error::from)
}

pub fn table_catalog_to_json(wrapper: TableCatalog) -> Result<String, Error> {
    let catalog = crate::catalog::TableCatalog::new(wrapper.table, &wrapper.base_url);
    catalog.to_json().map_err(Error::from)
}

pub fn xor_str(value: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    crate::table_encryption::table_encryption_service::xor_str(&value, &key)
}

pub fn xor_data(name: String, data: Vec<u8>) -> Vec<u8> {
    crate::table_encryption::table_encryption_service::xor(&name, &data)
}

pub fn xor_bytes(value: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    crate::table_encryption::table_encryption_service::xor_bytes(&value, &key)
}

pub fn xor_int32(value: i32, key: Vec<u8>) -> i32 {
    crate::table_encryption::table_encryption_service::xor_int32(value, &key)
}

pub fn xor_int64(value: i64, key: Vec<u8>) -> i64 {
    crate::table_encryption::table_encryption_service::xor_int64(value, &key)
}

pub fn xor_uint32(value: u32, key: Vec<u8>) -> u32 {
    crate::table_encryption::table_encryption_service::xor_uint32(value, &key)
}

pub fn xor_uint64(value: u64, key: Vec<u8>) -> u64 {
    crate::table_encryption::table_encryption_service::xor_uint64(value, &key)
}

pub fn convert_int(value: i32, key: Vec<u8>) -> i32 {
    crate::table_encryption::table_encryption_service::convert_int(value, &key)
}

pub fn convert_long(value: i64, key: Vec<u8>) -> i64 {
    crate::table_encryption::table_encryption_service::convert_long(value, &key)
}

pub fn convert_uint(value: u32, key: Vec<u8>) -> u32 {
    crate::table_encryption::table_encryption_service::convert_uint(value, &key)
}

pub fn convert_ulong(value: u64, key: Vec<u8>) -> u64 {
    crate::table_encryption::table_encryption_service::convert_ulong(value, &key)
}

pub fn convert_float(value: f32, key: Vec<u8>) -> f32 {
    crate::table_encryption::table_encryption_service::convert_float(value, &key)
}

pub fn convert_double(value: f64, key: Vec<u8>) -> f64 {
    crate::table_encryption::table_encryption_service::convert_double(value, &key)
}

pub fn encrypt_float(value: f32, key: Vec<u8>) -> f32 {
    crate::table_encryption::table_encryption_service::encrypt_float(value, &key)
}

pub fn encrypt_double(value: f64, key: Vec<u8>) -> f64 {
    crate::table_encryption::table_encryption_service::encrypt_double(value, &key)
}

pub fn create_key(bytes: Vec<u8>) -> Vec<u8> {
    crate::table_encryption::table_encryption_service::create_key(&bytes).to_vec()
}

pub fn convert_string(value: String, key: Vec<u8>) -> Result<String, Error> {
    crate::table_encryption::table_encryption_service::convert_string(&value, &key).map_err(Error::from)
}

pub fn encrypt_string(value: String, key: Vec<u8>) -> Result<String, Error> {
    crate::table_encryption::table_encryption_service::new_encrypt_string(&value, &key).map_err(Error::from)
}

pub fn extract_zip_file(zip_data: Vec<u8>, filename: String, file_to_extract: String) -> Result<Vec<u8>, Error> {
    let mut zip_file = crate::table_zip::TableZipFile::new(zip_data, filename).map_err(Error::from)?;
    zip_file.get_by_name(file_to_extract).map_err(Error::from)
}

pub fn extract_all_zip_files(zip_data: Vec<u8>, filename: String) -> Result<Vec<ZipFileEntry>, Error> {
    let mut zip_file = crate::table_zip::TableZipFile::new(zip_data, filename).map_err(Error::from)?;
    let files = zip_file.extract_all().map_err(Error::from)?;
    
    Ok(files.into_iter().map(|(name, data)| ZipFileEntry { name, data }).collect())
}

pub fn use_encryption() -> bool {
    crate::table_encryption::table_encryption_service::use_encryption()
}

pub fn set_use_encryption(enabled: bool) {
    crate::table_encryption::table_encryption_service::set_use_encryption(enabled)
}

#[derive(Debug, Clone)]
pub struct ZipFileEntry {
    pub name: String,
    pub data: Vec<u8>,
}