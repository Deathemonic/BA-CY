use crate::error::TableEncryptionError;
use crate::hash::calculate_xxhash;

use base64::{engine::general_purpose, Engine};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use rand_mt::Mt;

static USE_ENCRYPTION: Lazy<RwLock<bool>> = Lazy::new(|| RwLock::new(false));

pub fn use_encryption() -> bool {
    *USE_ENCRYPTION.read()
}

pub fn set_use_encryption(enabled: bool) {
    *USE_ENCRYPTION.write() = enabled;
}

fn gen_int31(rng: &mut Mt) -> u32 {
    rng.next_u32() >> 1
}

fn calculate_modulus(key: &[u8]) -> i32 {
    if key.is_empty() {
        return 1;
    }

    let mut modulus: i32 = (key[0] % 10) as i32;
    if modulus <= 1 {
        modulus = 7;
    }
    if key[0] & 1 != 0 {
        modulus = -modulus;
    }
    modulus
}

pub fn next_bytes(rng: &mut Mt, buf: &mut [u8]) {
    let (chunks, remainder) = buf.as_chunks_mut::<4>();

    for chunk in chunks {
        let num = gen_int31(rng);
        *chunk = num.to_le_bytes();
    }

    if !remainder.is_empty() {
        let num = gen_int31(rng);
        remainder.copy_from_slice(&num.to_le_bytes()[..remainder.len()]);
    }
}

pub fn xor_str(value: &[u8], key: &[u8]) -> Vec<u8> {
    value.iter().zip(key.iter()).map(|(a, b)| a ^ b).collect()
}

pub fn xor_bytes(value: &[u8], key: &[u8]) -> Vec<u8> {
    value
        .iter()
        .zip(key.iter().cycle())
        .map(|(v, k)| v ^ k)
        .collect()
}

pub fn xor_inplace(data: &mut [u8], key: &[u8]) {
    data.iter_mut()
        .zip(key.iter().cycle())
        .for_each(|(d, k)| *d ^= k);
}

pub fn xor(name: &str, data: &[u8]) -> Vec<u8> {
    let seed = calculate_xxhash(name.as_bytes(), false, false) as u32;
    let mut rng = Mt::new(seed);
    let mut result = vec![0u8; data.len()];
    next_bytes(&mut rng, &mut result);
    xor_inplace(&mut result, data);
    result
}

pub fn xor_int32(value: i32, key: &[u8]) -> i32 {
    let mut bytes = value.to_le_bytes();
    xor_inplace(&mut bytes, key);
    i32::from_le_bytes(bytes)
}

pub fn xor_int64(value: i64, key: &[u8]) -> i64 {
    let mut bytes = value.to_le_bytes();
    xor_inplace(&mut bytes, key);
    i64::from_le_bytes(bytes)
}

pub fn xor_uint32(value: u32, key: &[u8]) -> u32 {
    let mut bytes = value.to_le_bytes();
    xor_inplace(&mut bytes, key);
    u32::from_le_bytes(bytes)
}

pub fn xor_uint64(value: u64, key: &[u8]) -> u64 {
    let mut bytes = value.to_le_bytes();
    xor_inplace(&mut bytes, key);
    u64::from_le_bytes(bytes)
}

pub fn convert_int(value: i32, key: &[u8]) -> i32 {
    if value != 0 { xor_int32(value, key) } else { 0 }
}

pub fn convert_long(value: i64, key: &[u8]) -> i64 {
    if value != 0 { xor_int64(value, key) } else { 0 }
}

pub fn convert_uint(value: u32, key: &[u8]) -> u32 {
    if value != 0 { xor_uint32(value, key) } else { 0 }
}

pub fn convert_ulong(value: u64, key: &[u8]) -> u64 {
    if value != 0 { xor_uint64(value, key) } else { 0 }
}

pub fn convert_enum<T>(value: T, key: &[u8]) -> T
where
    T: Copy + flatbuffers::EndianScalar,
    T::Scalar: Into<i32> + From<i32> + Copy,
{
    let scalar_val: i32 = value.to_little_endian().into();
    let converted = if scalar_val != 0 { convert_int(scalar_val, key) } else { 0 };
    T::from_little_endian(T::Scalar::from(converted))
}

fn apply_modulus<F>(value: F, key: &[u8], op: impl FnOnce(F, i32) -> F) -> F
where
    F: PartialEq + From<f32>,
{
    if value == F::from(0.0) {
        return value;
    }
    let modulus = calculate_modulus(key);
    if modulus == 1 {
        return value;
    }
    op(value, modulus)
}

pub fn convert_float(value: f32, key: &[u8]) -> f32 {
    apply_modulus(value, key, |v, m| v / (m as f32 * 10000.0))
}

pub fn convert_double(value: f64, key: &[u8]) -> f64 {
    apply_modulus(value, key, |v, m| v / (m as f64 * 10000.0))
}

pub fn encrypt_float(value: f32, key: &[u8]) -> f32 {
    apply_modulus(value, key, |v, m| v * (m as f32 * 10000.0))
}

pub fn encrypt_double(value: f64, key: &[u8]) -> f64 {
    apply_modulus(value, key, |v, m| v * (m as f64 * 10000.0))
}

pub fn create_key(bytes: &[u8]) -> [u8; 8] {
    let seed: u32 = calculate_xxhash(bytes, false, false) as u32;
    let mut rng: Mt = Mt::new(seed);
    let mut buf: [u8; 8] = [0u8; 8];
    next_bytes(&mut rng, &mut buf);
    buf
}

pub fn convert_string(value: &str, key: &[u8]) -> Result<String, TableEncryptionError> {
    let mut raw = general_purpose::STANDARD.decode(value.as_bytes())?;

    xor_inplace(&mut raw, key);

    if raw.len() % 2 == 0 {
        let utf16_bytes: Vec<u16> = raw
            .chunks_exact(2)
            .map(|x| u16::from_le_bytes([x[0], x[1]]))
            .collect();

        if let Ok(s) = String::from_utf16(&utf16_bytes) {
            return Ok(s);
        }
    }

    Ok(raw.iter().map(|&x| x as char).collect())
}

pub fn new_encrypt_string(value: &str, key: &[u8]) -> Result<String, TableEncryptionError> {
    if value.is_empty() || value.len() < 8 {
        return Ok(value.to_string());
    }

    let mut raw: Vec<u8> = value.encode_utf16().flat_map(|x| x.to_le_bytes()).collect();

    xor_inplace(&mut raw, key);

    Ok(general_purpose::STANDARD.encode(&raw))
}
