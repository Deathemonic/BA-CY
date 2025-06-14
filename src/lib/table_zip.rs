use std::io::{Cursor, Read};
use std::path::Path;
use anyhow::Result;
use base64::{Engine, engine::general_purpose};
use zip::ZipArchive;

use crate::lib::hash::calculate_xxhash;
use crate::lib::table_encryption::table_encryption_service::next_bytes;
use rand_mt::Mt;

pub struct TableZipFile<R: Read + std::io::Seek> {
    archive: ZipArchive<R>,
    password: Vec<u8>,
}

impl<R: Read + std::io::Seek> TableZipFile<R> {
    pub fn new(reader: R, password: Option<Vec<u8>>, file_name: Option<&str>) -> Result<Self> {
        let archive = ZipArchive::new(reader)?;
        let password = match password {
            Some(pwd) => pwd,
            None => {
                let name = file_name.unwrap_or("").to_lowercase();
                Self::generate_password(&name)
            }
        };

        Ok(Self { archive, password })
    }

    fn generate_password(file_name: &str) -> Vec<u8> {
        let hash_value = calculate_xxhash(file_name.as_bytes());
        let mut twister = Mt::new(hash_value);
        let mut next_bytes_buf = vec![0u8; 15];
        next_bytes(&mut twister, &mut next_bytes_buf);
        general_purpose::STANDARD.encode(&next_bytes_buf).into_bytes()
    }

    pub fn open(&mut self, name: &str) -> Result<Vec<u8>> {
        let mut file = self.archive.by_name_decrypt(name, &self.password)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}

impl TableZipFile<std::fs::File> {
    pub fn from_path<P: AsRef<Path>>(path: P, password: Option<Vec<u8>>) -> Result<Self> {
        let file = std::fs::File::open(&path)?;
        let file_name = path.as_ref().file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        Self::new(file, password, Some(file_name))
    }
}

impl TableZipFile<Cursor<Vec<u8>>> {
    pub fn from_bytes(bytes: Vec<u8>, password: Option<Vec<u8>>, file_name: Option<&str>) -> Result<Self> {
        let cursor = Cursor::new(bytes);
        Self::new(cursor, password, file_name)
    }
}
