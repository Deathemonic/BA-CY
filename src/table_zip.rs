use crate::error::TableZipError;
use crate::hash::calculate_xxhash;
use crate::table_encryption::table_encryption_service::next_bytes;

use base64::{engine::general_purpose, Engine};
use rand_mt::Mt;
use std::io::{Cursor, Read};
use zip::ZipArchive;

pub struct TableZipFile {
    archive: ZipArchive<Cursor<Vec<u8>>>,
    password: String,
}

impl TableZipFile {
    pub fn new<S: AsRef<str>>(buf: Vec<u8>, filename: S) -> Result<Self, TableZipError> {
        let hash = calculate_xxhash(filename.as_ref().as_bytes(), false, false) as u32;
        let mut rng = Mt::new(hash);
        let mut next_buf = [0u8; 15];
        next_bytes(&mut rng, &mut next_buf);
        let password = general_purpose::STANDARD.encode(next_buf);
        let archive = ZipArchive::new(Cursor::new(buf))?;

        Ok(Self { archive, password })
    }

    pub fn get_by_name<S: AsRef<str>>(&mut self, name: S) -> Result<Vec<u8>, TableZipError> {
        let mut file = self
            .archive
            .by_name_decrypt(name.as_ref(), self.password.as_bytes())?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(buf)
    }

    pub fn extract_all(&mut self) -> Result<Vec<(String, Vec<u8>)>, TableZipError> {
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
