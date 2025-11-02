pub mod bindings;
pub mod catalog;
pub mod crc;
pub mod error;
pub mod hash;
pub mod table_encryption;
pub mod table_zip;

#[cfg(not(feature = "uniffi"))]
pub use catalog::{Asset, Media, MediaCatalog, Packing, Patch, Table, TableCatalog};

pub use hash::CrcResult;

#[cfg(feature = "uniffi")]
pub use bindings::*;

#[cfg(feature = "uniffi")]
uniffi::include_scaffolding!("bacy");
