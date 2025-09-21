use super::memorypack;
use crate::error::CatalogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Cursor;
use std::marker::PhantomData;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Packing {
    pub milestone: String,
    pub patch_version: i64,
    pub full_patch_packs: Vec<Patch>,
    pub update_packs: Vec<Patch>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Patch {
    pub pack_name: String,
    pub pack_size: i64,
    pub crc: i64,
    pub is_prologue: bool,
    pub is_split_download: bool,
    pub bundle_files: Vec<Asset>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Asset {
    pub name: String,
    pub size: i64,
    pub is_prologue: bool,
    pub crc: i64,
    pub is_split_download: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Catalog<T> {
    pub table: HashMap<String, T>,

    #[serde(skip)]
    pub base_url: String,

    #[serde(skip)]
    _phantom: PhantomData<T>,
}

impl<T: Serialize + for<'de> Deserialize<'de> + Clone> Catalog<T> {
    pub fn new(table: HashMap<String, T>, base_url: &str) -> Self {
        Self {
            table,
            base_url: base_url.to_string(),
            _phantom: PhantomData,
        }
    }

    pub fn to_json(&self) -> Result<String, CatalogError> {
        serde_json::to_string_pretty(self).map_err(|_| CatalogError::SerializationFailed)
    }

    pub fn from_json(json_data: &str, base_url: &str) -> Result<Self, CatalogError> {
        let mut catalog: Self = serde_json::from_str(json_data).map_err(|_| CatalogError::DeserializationFailed)?;
        catalog.base_url = base_url.to_string();
        Ok(catalog)
    }

    pub fn get_table(&self) -> &HashMap<String, T> {
        &self.table
    }

    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }
}

pub type MediaCatalog = Catalog<Media>;
pub type TableCatalog = Catalog<Table>;

fn deserialize_catalog<T, F>(bytes: &[u8], base_url: &str, reader_fn: F) -> Result<Catalog<T>, CatalogError>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
    F: Fn(&mut Cursor<&[u8]>) -> Result<(String, T), CatalogError>,
{
    let mut cursor: Cursor<&[u8]> = Cursor::new(bytes);
    let _ = memorypack::read_i8(&mut cursor)?;

    let table_size: i32 = memorypack::read_i32(&mut cursor)?;
    let table: HashMap<String, T> = (0..table_size)
        .map(|_| reader_fn(&mut cursor))
        .collect::<Result<HashMap<String, T>, CatalogError>>()?;

    Ok(Catalog::new(table, base_url))
}

impl MediaCatalog {
    pub fn deserialize(bytes: &[u8], base_url: &str) -> Result<Self, CatalogError> {
        deserialize_catalog(bytes, base_url, read_media)
    }
}

impl TableCatalog {
    pub fn deserialize(bytes: &[u8], base_url: &str) -> Result<Self, CatalogError> {
        deserialize_catalog(bytes, base_url, read_table)
    }
}

fn read_media(cursor: &mut Cursor<&[u8]>) -> Result<(String, Media), CatalogError> {
    let _ = memorypack::read_i32(cursor);
    let key: String = memorypack::read_string(cursor)?;
    let _ = memorypack::read_i8(cursor);
    let _ = memorypack::read_i32(cursor);
    let path: String = memorypack::read_string(cursor)?;
    let _ = memorypack::read_i32(cursor);

    let file_name: String = memorypack::read_string(cursor)?;
    let bytes: i64 = memorypack::read_i64(cursor)?;
    let crc: i64 = memorypack::read_i64(cursor)?;
    let is_prologue: bool = memorypack::read_bool(cursor)?;
    let is_split_download: bool = memorypack::read_bool(cursor)?;
    let media_type: i32 = memorypack::read_i32(cursor)?;

    Ok((
        key,
        Media {
            path,
            file_name,
            bytes,
            crc,
            is_prologue,
            is_split_download,
            media_type,
        },
    ))
}

fn read_table(cursor: &mut Cursor<&[u8]>) -> Result<(String, Table), CatalogError> {
    let _ = memorypack::read_i32(cursor);
    let key: String = memorypack::read_string(cursor)?;
    let _ = memorypack::read_i8(cursor);
    let _ = memorypack::read_i32(cursor);

    let name: String = memorypack::read_string(cursor)?;
    let size: i64 = memorypack::read_i64(cursor)?;
    let crc: i64 = memorypack::read_i64(cursor)?;
    let is_in_build: bool = memorypack::read_bool(cursor)?;
    let is_changed: bool = memorypack::read_bool(cursor)?;
    let is_prologue: bool = memorypack::read_bool(cursor)?;
    let is_split_download: bool = memorypack::read_bool(cursor)?;

    let includes: Vec<String> = read_includes(cursor)?;

    Ok((
        key,
        Table {
            name,
            size,
            crc,
            is_in_build,
            is_changed,
            is_prologue,
            is_split_download,
            includes,
        },
    ))
}

fn read_includes(cursor: &mut Cursor<&[u8]>) -> Result<Vec<String>, CatalogError> {
    let size: i32 = memorypack::read_i32(cursor)?;
    if size == -1 {
        return Ok(vec![]);
    }

    let _ = memorypack::read_i32(cursor);

    (0..size)
        .map(|i| {
            let s: String = memorypack::read_string(cursor)?;
            if i != size - 1 {
                let _ = memorypack::read_i32(cursor)?;
            }
            Ok(s)
        })
        .collect::<Result<Vec<String>, CatalogError>>()
}