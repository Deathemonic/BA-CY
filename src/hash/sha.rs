use sha2::{Digest, Sha256};

pub fn compute(source: &[u8]) -> [u8; 32] { Sha256::digest(source).into() }

pub fn compute_str(source: &str) -> [u8; 32] { compute(source.as_bytes()) }
