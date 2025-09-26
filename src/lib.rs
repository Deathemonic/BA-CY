pub mod bindings;
pub mod hash;
pub mod catalog;
pub mod memorypack;
pub mod table_encryption;
pub mod table_zip;
pub mod error;

pub use hash::CrcResult;
pub use catalog::{Media, Table, Asset, Packing, Patch};

#[cfg(feature = "uniffi")]
pub use bindings::*;

#[cfg(feature = "uniffi")]
uniffi::include_scaffolding!("bacy");
