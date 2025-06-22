use pyo3::prelude::*;

pub mod lib {
    pub mod catalog;
    pub mod hash;
    pub mod memorypack;
    pub mod table_encryption;
    pub mod table_zip;
}

use lib::catalog::{Media, Table, MediaCatalog, TableCatalog};
use lib::hash::{calculate_crc32 as rust_calculate_crc32, calculate_md5 as rust_calculate_md5, calculate_xxhash as rust_calculate_xxhash};
use lib::table_encryption::table_encryption_service::{xor as rust_xor, convert_string as rust_convert_string, new_encrypt_string as rust_new_encrypt_string, create_key as rust_create_key, convert_int as rust_convert_int, convert_long as rust_convert_long, convert_uint as rust_convert_uint, convert_ulong as rust_convert_ulong, convert_float as rust_convert_float, convert_double as rust_convert_double};
use lib::table_zip::TableZipFile;
use std::collections::HashMap;

#[pyclass(name = "Media")]
#[derive(Clone)]
pub struct PyMedia {
    #[pyo3(get, set)]
    pub path: String,
    #[pyo3(get, set)]
    pub file_name: String,
    #[pyo3(get, set)]
    pub bytes: i64,
    #[pyo3(get, set)]
    pub crc: i64,
    #[pyo3(get, set)]
    pub is_prologue: bool,
    #[pyo3(get, set)]
    pub is_split_download: bool,
    #[pyo3(get, set)]
    pub media_type: i32,
}

#[pyclass(name = "Table")]
#[derive(Clone)]
pub struct PyTable {
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub size: i64,
    #[pyo3(get, set)]
    pub crc: i64,
    #[pyo3(get, set)]
    pub is_in_build: bool,
    #[pyo3(get, set)]
    pub is_changed: bool,
    #[pyo3(get, set)]
    pub is_prologue: bool,
    #[pyo3(get, set)]
    pub is_split_download: bool,
    #[pyo3(get, set)]
    pub includes: Vec<String>,
}

#[pyclass(name = "MediaCatalog")]
pub struct PyMediaCatalog {
    inner: MediaCatalog,
}

#[pymethods]
impl PyMediaCatalog {
    #[new]
    pub fn new(table: HashMap<String, PyMedia>, base_url: String) -> Self {
        let table: HashMap<String, Media> = table.into_iter()
            .map(|(k, v)| (k, Media {
                path: v.path,
                file_name: v.file_name,
                bytes: v.bytes,
                crc: v.crc,
                is_prologue: v.is_prologue,
                is_split_download: v.is_split_download,
                media_type: v.media_type,
            }))
            .collect();
        Self {
            inner: MediaCatalog::new(table, &base_url)
        }
    }

    #[staticmethod]
    pub fn deserialize(_py: Python<'_>, bytes: &[u8], base_url: &str) -> PyResult<Self> {
        MediaCatalog::deserialize(bytes, base_url)
            .map(|inner| Self { inner })
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    pub fn to_json(&self) -> PyResult<String> {
        self.inner.to_json()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    #[staticmethod]
    pub fn from_json(json_data: &str, base_url: &str) -> PyResult<Self> {
        MediaCatalog::from_json(json_data, base_url)
            .map(|inner| Self { inner })
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    pub fn get_table(&self) -> HashMap<String, PyMedia> {
        self.inner.get_table().iter()
            .map(|(k, v)| (k.clone(), PyMedia {
                path: v.path.clone(),
                file_name: v.file_name.clone(),
                bytes: v.bytes,
                crc: v.crc,
                is_prologue: v.is_prologue,
                is_split_download: v.is_split_download,
                media_type: v.media_type,
            }))
            .collect()
    }

    pub fn get_base_url(&self) -> String {
        self.inner.get_base_url().to_string()
    }
}

#[pyclass(name = "TableCatalog")]
pub struct PyTableCatalog {
    inner: TableCatalog,
}

#[pymethods]
impl PyTableCatalog {
    #[new]
    pub fn new(table: HashMap<String, PyTable>, base_url: String) -> Self {
        let table: HashMap<String, Table> = table.into_iter()
            .map(|(k, v)| (k, Table {
                name: v.name,
                size: v.size,
                crc: v.crc,
                is_in_build: v.is_in_build,
                is_changed: v.is_changed,
                is_prologue: v.is_prologue,
                is_split_download: v.is_split_download,
                includes: v.includes,
            }))
            .collect();
        Self {
            inner: TableCatalog::new(table, &base_url)
        }
    }

    #[staticmethod]
    pub fn deserialize(_py: Python<'_>, bytes: &[u8], base_url: &str) -> PyResult<Self> {
        TableCatalog::deserialize(bytes, base_url)
            .map(|inner| Self { inner })
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    pub fn to_json(&self) -> PyResult<String> {
        self.inner.to_json()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    #[staticmethod]
    pub fn from_json(json_data: &str, base_url: &str) -> PyResult<Self> {
        TableCatalog::from_json(json_data, base_url)
            .map(|inner| Self { inner })
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    pub fn get_table(&self) -> HashMap<String, PyTable> {
        self.inner.get_table().iter()
            .map(|(k, v)| (k.clone(), PyTable {
                name: v.name.clone(),
                size: v.size,
                crc: v.crc,
                is_in_build: v.is_in_build,
                is_changed: v.is_changed,
                is_prologue: v.is_prologue,
                is_split_download: v.is_split_download,
                includes: v.includes.clone(),
            }))
            .collect()
    }

    pub fn get_base_url(&self) -> String {
        self.inner.get_base_url().to_string()
    }
}

#[pyclass(name = "TableZipFile")]
pub struct PyTableZipFile {
    inner: TableZipFile,
}

#[pymethods]
impl PyTableZipFile {
    #[new]
    pub fn new(bytes: Vec<u8>, file_name: String) -> PyResult<Self> {
        let inner = TableZipFile::new(bytes, file_name)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(Self { inner })
    }

    pub fn get_by_name(&mut self, name: &str) -> PyResult<Vec<u8>> {
        self.inner.get_by_name(name)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    pub fn extract_all(&mut self) -> PyResult<Vec<(String, Vec<u8>)>> {
        self.inner.extract_all()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }
}

#[pymethods]
impl PyMedia {
    #[new]
    pub fn new(path: String, file_name: String, bytes: i64, crc: i64, is_prologue: bool, is_split_download: bool, media_type: i32) -> Self {
        Self {
            path,
            file_name,
            bytes,
            crc,
            is_prologue,
            is_split_download,
            media_type,
        }
    }
}

#[pymethods]
impl PyTable {
    #[new]
    pub fn new(name: String, size: i64, crc: i64, is_in_build: bool, is_changed: bool, is_prologue: bool, is_split_download: bool, includes: Vec<String>) -> Self {
        Self {
            name,
            size,
            crc,
            is_in_build,
            is_changed,
            is_prologue,
            is_split_download,
            includes,
        }
    }
}

#[pyfunction]
fn calculate_crc32(path: String) -> PyResult<u32> {
    let path_buf = std::path::PathBuf::from(path);
    rust_calculate_crc32(path_buf).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

#[pyfunction]
fn calculate_md5(path: String) -> PyResult<String> {
    let path_buf = std::path::PathBuf::from(path);
    rust_calculate_md5(path_buf).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

#[pyfunction]
fn calculate_xxhash(bytes: &[u8]) -> u32 {
    rust_calculate_xxhash(bytes)
}

#[pyfunction]
fn xor(name: &str, data: &[u8]) -> Vec<u8> {
    rust_xor(name, data)
}

#[pyfunction]
fn convert_string(value: &str, key: &[u8]) -> PyResult<String> {
    rust_convert_string(value, key)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

#[pyfunction]
fn new_encrypt_string(value: &str, key: &[u8]) -> PyResult<String> {
    rust_new_encrypt_string(value, key)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

#[pyfunction]
fn create_key(bytes: &[u8]) -> Vec<u8> {
    rust_create_key(bytes).to_vec()
}

#[pyfunction]
fn convert_int(value: i32, key: &[u8]) -> i32 {
    rust_convert_int(value, key)
}

#[pyfunction]
fn convert_long(value: i64, key: &[u8]) -> i64 {
    rust_convert_long(value, key)
}

#[pyfunction]
fn convert_uint(value: u32, key: &[u8]) -> u32 {
    rust_convert_uint(value, key)
}

#[pyfunction]
fn convert_ulong(value: u64, key: &[u8]) -> u64 {
    rust_convert_ulong(value, key)
}

#[pyfunction]
fn convert_float(value: f32, key: &[u8]) -> f32 {
    rust_convert_float(value, key)
}

#[pyfunction]
fn convert_double(value: f64, key: &[u8]) -> f64 {
    rust_convert_double(value, key)
}

#[pymodule]
fn bacy(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyMedia>()?;
    m.add_class::<PyTable>()?;
    m.add_class::<PyMediaCatalog>()?;
    m.add_class::<PyTableCatalog>()?;
    m.add_class::<PyTableZipFile>()?;
    m.add_function(wrap_pyfunction!(calculate_crc32, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_md5, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_xxhash, m)?)?;
    m.add_function(wrap_pyfunction!(xor, m)?)?;
    m.add_function(wrap_pyfunction!(convert_string, m)?)?;
    m.add_function(wrap_pyfunction!(new_encrypt_string, m)?)?;
    m.add_function(wrap_pyfunction!(create_key, m)?)?;
    m.add_function(wrap_pyfunction!(convert_int, m)?)?;
    m.add_function(wrap_pyfunction!(convert_long, m)?)?;
    m.add_function(wrap_pyfunction!(convert_uint, m)?)?;
    m.add_function(wrap_pyfunction!(convert_ulong, m)?)?;
    m.add_function(wrap_pyfunction!(convert_float, m)?)?;
    m.add_function(wrap_pyfunction!(convert_double, m)?)?;
    Ok(())
}