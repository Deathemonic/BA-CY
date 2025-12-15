use crate::crypto::xor::xor_inplace;
use crate::error::TableEncryptionError;
use crate::hash::xxhash;
use crate::math::mersenne::MersenneTwister;

use base64::{Engine, engine::general_purpose::STANDARD};

#[inline]
pub fn create_key(name: &str) -> [u8; 8] {
    let hash = xxhash::calculate_hash_str(name);
    let mut mt = MersenneTwister::new(hash);
    let mut key = [0u8; 8];
    mt.next_bytes(&mut key);
    key
}

pub fn create_password(key: &str, length: usize) -> String {
    let hash = xxhash::calculate_hash_str(key);
    let mut mt = MersenneTwister::new(hash);

    let byte_length = length * 3 / 4;
    let mut bytes = vec![0u8; byte_length];
    mt.next_bytes(&mut bytes);

    STANDARD.encode(&bytes)
}

pub fn xor(name: &str, bytes: &mut [u8]) {
    let hash = xxhash::calculate_hash_str(name);
    let mut mt = MersenneTwister::new(hash);

    let (chunks, remainder) = bytes.as_chunks_mut::<4>();
    for chunk in chunks {
        *chunk = (mt.next_u32() ^ u32::from_le_bytes(*chunk)).to_le_bytes();
    }

    if !remainder.is_empty() {
        let xor_key = mt.next_u32().to_le_bytes();
        for (i, byte) in remainder.iter_mut().enumerate() {
            *byte ^= xor_key[i];
        }
    }
}

#[inline]
pub fn decrypt_i32(value: i32, key: &[u8]) -> i32 {
    let mut bytes = value.to_le_bytes();
    xor_inplace(&mut bytes, key);
    i32::from_le_bytes(bytes)
}

#[inline]
pub fn decrypt_i64(value: i64, key: &[u8]) -> i64 {
    let mut bytes = value.to_le_bytes();
    xor_inplace(&mut bytes, key);
    i64::from_le_bytes(bytes)
}

#[inline]
pub fn decrypt_u32(value: u32, key: &[u8]) -> u32 {
    let mut bytes = value.to_le_bytes();
    xor_inplace(&mut bytes, key);
    u32::from_le_bytes(bytes)
}

#[inline]
pub fn decrypt_u64(value: u64, key: &[u8]) -> u64 {
    let mut bytes = value.to_le_bytes();
    xor_inplace(&mut bytes, key);
    u64::from_le_bytes(bytes)
}

#[inline]
pub fn decrypt_enum<T>(value: T, key: &[u8]) -> T
where
    T: Copy + flatbuffers::EndianScalar,
    T::Scalar: Into<i32> + From<i32> + Copy,
{
    let scalar_val: i32 = value.to_little_endian().into();
    let converted = if scalar_val != 0 {
        decrypt_i32(scalar_val, key)
    } else {
        0
    };
    T::from_little_endian(T::Scalar::from(converted))
}

#[inline]
pub fn decrypt_f32(value: f32, key: &[u8]) -> f32 {
    let divisor = calculate_multiplier(key[0]);
    value / divisor as f32 / 10000.0
}

#[inline]
pub fn decrypt_f64(value: f64, key: &[u8]) -> f64 {
    let divisor = calculate_multiplier(key[0]);
    value / divisor as f64 / 1000000.0
}

pub fn decrypt_string(value: &str, key: &[u8]) -> Result<String, TableEncryptionError> {
    if value.is_empty() {
        return Ok(String::new());
    }

    let mut bytes = STANDARD.decode(value)?;
    xor_inplace(&mut bytes, key);

    let utf16_values: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();

    Ok(String::from_utf16(&utf16_values)?)
}

#[inline]
pub fn encrypt_f32(value: f32, key: &[u8]) -> f32 {
    let multiplier = calculate_multiplier(key[0]);
    ((value * 10000.0) as i32 * multiplier) as f32
}

#[inline]
pub fn encrypt_f64(value: f64, key: &[u8]) -> f64 {
    let multiplier = calculate_multiplier(key[0]);
    ((value * 1000000.0) as i32 * multiplier) as f64
}

pub fn encrypt_string(value: &str, key: &[u8]) -> String {
    if value.is_empty() {
        return String::new();
    }

    let mut bytes: Vec<u8> = value.encode_utf16().flat_map(|c| c.to_le_bytes()).collect();
    xor_inplace(&mut bytes, key);
    STANDARD.encode(&bytes)
}

#[inline]
fn calculate_multiplier(key_byte: u8) -> i32 {
    let mod_value = key_byte.wrapping_sub(5 * ((key_byte / 5) & 0xFE));
    let multiplier = if mod_value >= 2 { mod_value } else { 7 };
    if (key_byte & 1) != 0 {
        -(multiplier as i32)
    } else {
        multiplier as i32
    }
}
