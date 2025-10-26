use memorypack::{MemoryPackSerializer, MemoryPackSerialize, MemoryPackDeserialize, MemoryPackable};
use serde::{Deserialize, Serialize};
use hashbrown::HashMap;

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
pub struct Catalog<T: MemoryPackSerialize + MemoryPackDeserialize + Default> {
    pub table: HashMap<String, T>,

    #[serde(skip)]
    #[memorypack(skip)]
    pub base_url: String,
}

impl<T> Catalog<T>
where
    T: MemoryPackSerialize + MemoryPackDeserialize + Serialize + for<'de> Deserialize<'de> + Clone + Default,
{
    pub fn new(table: HashMap<String, T>, base_url: &str) -> Self {
        Self {
            table,
            base_url: base_url.to_string(),
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json_data: &str, base_url: &str) -> Result<Self, serde_json::Error> {
        let mut catalog: Self = serde_json::from_str(json_data)?;
        catalog.base_url = base_url.to_string();
        Ok(catalog)
    }

    pub fn deserialize(bytes: &[u8], base_url: &str) -> Result<Self, memorypack::MemoryPackError> {
        let mut catalog = MemoryPackSerializer::deserialize::<Self>(bytes)?;
        catalog.base_url = base_url.to_string();
        Ok(catalog)
    }

    pub fn serialize(&self) -> Result<Vec<u8>, memorypack::MemoryPackError> {
        MemoryPackSerializer::serialize(self)
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
