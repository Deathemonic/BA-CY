pub mod error;
pub mod bindings;

pub mod lib {
    pub mod catalog;
    pub mod hash;
    pub mod memorypack;
    pub mod table_encryption;
    pub mod table_zip;
    pub mod crc_service;
}

pub use error::*;
pub use bindings::*;

pub use lib::hash;
pub use lib::hash::CrcResult;
pub use lib::crc_service;
pub use lib::catalog;
pub use lib::catalog::{Media, Table};
pub use lib::memorypack;
pub use lib::table_encryption;
pub use lib::table_zip;

uniffi::include_scaffolding!("bacy");