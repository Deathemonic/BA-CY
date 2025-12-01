use crate::hash::xxhash;

use std::path::{Path, PathBuf};

pub fn get_file_path<P: AsRef<Path>>(
    path: P,
    crc: Option<i64>,
    no_hash: bool,
    to_lower: bool,
) -> PathBuf {
    let path = path.as_ref();
    let parent = path.parent();
    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

    if file_name.is_empty() {
        return PathBuf::new();
    }

    let hash_input = if to_lower {
        file_name.to_lowercase()
    } else {
        file_name.to_string()
    };
    let hash = xxhash::calculate_hash64_str(&hash_input);

    let result = match (crc, no_hash) {
        (Some(crc_value), _) => format!("{}_{}", hash, crc_value),
        (None, true) => file_name.to_string(),
        (None, false) => hash.to_string(),
    };

    if let Some(parent_path) = parent {
        parent_path.join(result)
    } else {
        PathBuf::from(result)
    }
}
