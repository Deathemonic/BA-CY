use anyhow::Result;
use base64::{Engine, engine::general_purpose};
use rand_mt::Mt;
use std::io::{Cursor, Read};
use zip::ZipArchive;

use crate::lib::hash::calculate_xxhash;
use crate::lib::table_encryption::table_encryption_service::next_bytes;

pub struct TableZipFile {
    archive: ZipArchive<Cursor<Vec<u8>>>,
    password: String,
}

impl TableZipFile {
    pub fn new<S: AsRef<str>>(buf: Vec<u8>, filename: S) -> Result<Self> {
        let hash = calculate_xxhash(filename.as_ref().as_bytes());
        let mut rng = Mt::new(hash);
        let mut next_buf = [0u8; 15];
        next_bytes(&mut rng, &mut next_buf);
        let password = general_purpose::STANDARD.encode(&next_buf);
        let archive = ZipArchive::new(Cursor::new(buf))?;

        Ok(Self { archive, password })
    }

    pub fn get_by_name<S: AsRef<str>>(&mut self, name: S) -> Result<Vec<u8>> {
        let mut file = self
            .archive
            .by_name_decrypt(name.as_ref(), self.password.as_bytes())?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(buf)
    }

    pub fn extract_all(&mut self) -> Result<Vec<(String, Vec<u8>)>> {
        let mut files = Vec::new();
        for i in 0..self.archive.len() {
            let mut file = self.archive.by_index_decrypt(i, self.password.as_bytes())?;
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;
            files.push((file.name().to_string(), buf));
        }
        Ok(files)
    }
}