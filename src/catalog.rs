use crate::error::CatalogError;

use hashbrown::HashMap;
use memorypack::{MemoryPackSerializer, MemoryPackable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Packing {
    pub milestone: String,
    pub patch_version: i64,
    pub full_patch_packs: Vec<Patch>,
    pub update_packs: Vec<Patch>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Patch {
    pub pack_name: String,
    pub pack_size: i64,
    pub crc: i64,
    pub is_prologue: bool,
    pub is_split_download: bool,
    pub bundle_files: Vec<Asset>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Asset {
    pub name: String,
    pub size: i64,
    pub is_prologue: bool,
    pub crc: i64,
    pub is_split_download: bool,
}

#[derive(MemoryPackable, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Media {
    pub path: String,
    pub file_name: String,
    pub bytes: i64,
    pub crc: i64,
    pub is_prologue: bool,
    pub is_split_download: bool,
    pub media_type: i32,
}

#[derive(MemoryPackable, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Table {
    pub name: String,
    pub size: i64,
    pub crc: i64,
    pub is_in_build: bool,
    pub is_changed: bool,
    pub is_prologue: bool,
    pub is_split_download: bool,
    pub includes: Vec<String>,
}

#[derive(MemoryPackable, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct MediaCatalog {
    pub table: HashMap<String, Media>,
}

#[derive(MemoryPackable, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct TableCatalog {
    pub table: HashMap<String, Table>,
}

#[inline]
pub fn deserialize_media_catalog(bytes: &[u8]) -> Result<MediaCatalog, CatalogError> {
    let catalog = MemoryPackSerializer::deserialize::<MediaCatalog>(bytes)?;
    Ok(catalog)
}

#[inline]
pub fn deserialize_table_catalog(bytes: &[u8]) -> Result<TableCatalog, CatalogError> {
    let catalog = MemoryPackSerializer::deserialize::<TableCatalog>(bytes)?;
    Ok(catalog)
}
