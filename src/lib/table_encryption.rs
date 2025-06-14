pub mod table_encryption_service {
    use anyhow::Result;
    use base64::{Engine, engine::general_purpose};
    use byteorder::{ByteOrder, LittleEndian};
    use rand_mt::Mt;

    use crate::lib::hash::calculate_xxhash;

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
        let len: usize = (buf.len() + 3) / 4;
        for i in 0..len {
            let num: u32 = gen_int31(rng);
            let offset: usize = i * 4;
            let end: usize = (offset + 4).min(buf.len());
            let chunk: &mut [u8] = &mut buf[offset..end];
            for (j, x) in chunk.iter_mut().enumerate() {
                *x = ((num >> (j * 8)) & 0xFF) as u8;
            }
        }
    }

    fn strxor(value: &[u8], key: &[u8]) -> Vec<u8> {
        value.iter().zip(key.iter()).map(|(a, b)| a ^ b).collect()
    }

    fn xor_with_key(value: &mut [u8], key: &[u8]) -> Vec<u8> {
        match value.len().cmp(&key.len()) {
            std::cmp::Ordering::Equal => strxor(value, key),
            std::cmp::Ordering::Less => strxor(value, &key[..value.len()]),
            std::cmp::Ordering::Greater => {
                let mut result: Vec<u8> = Vec::with_capacity(value.len());
                let key_len: usize = key.len();
                let full_chunks: usize = value.len() / key_len;
                let remainder: usize = value.len() % key_len;

                for i in 0..full_chunks {
                    let start: usize = i * key_len;
                    let end: usize = start + key_len;
                    result.extend(strxor(&value[start..end], key));
                }

                if remainder > 0 {
                    let start: usize = full_chunks * key_len;
                    result.extend(strxor(&value[start..], &key[..remainder]));
                }

                result
            }
        }
    }

    pub fn xor(name: &str, data: &[u8]) -> Vec<u8> {
        let seed: u32 = calculate_xxhash(name.as_bytes());
        let mut rng: Mt = Mt::new(seed);
        let mut key: Vec<u8> = vec![0u8; data.len()];
        next_bytes(&mut rng, &mut key);
        xor_with_key(&mut key, data)
    }

    pub fn xor_bytes(value: &[u8], key: &[u8]) -> Vec<u8> {
        value.iter().zip(key.iter().cycle()).map(|(v, k)| v ^ k).collect()
    }

    pub fn xor_int32(value: i32, key: &[u8]) -> i32 {
        let mut bytes: [u8; 4] = [0u8; 4];
        LittleEndian::write_i32(&mut bytes, value);
        let xored_bytes: Vec<u8> = xor_bytes(&bytes, key);
        LittleEndian::read_i32(&xored_bytes)
    }

    pub fn xor_int64(value: i64, key: &[u8]) -> i64 {
        let mut bytes: [u8; 8] = [0u8; 8];
        LittleEndian::write_i64(&mut bytes, value);
        let xored_bytes: Vec<u8> = xor_bytes(&bytes, key);
        LittleEndian::read_i64(&xored_bytes)
    }

    pub fn xor_uint32(value: u32, key: &[u8]) -> u32 {
        let mut bytes: [u8; 4] = [0u8; 4];
        LittleEndian::write_u32(&mut bytes, value);
        let xored_bytes: Vec<u8> = xor_bytes(&bytes, key);
        LittleEndian::read_u32(&xored_bytes)
    }

    pub fn xor_uint64(value: u64, key: &[u8]) -> u64 {
        let mut bytes: [u8; 8] = [0u8; 8];
        LittleEndian::write_u64(&mut bytes, value);
        let xored_bytes: Vec<u8> = xor_bytes(&bytes, key);
        LittleEndian::read_u64(&xored_bytes)
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

    pub fn convert_float(value: f32, key: &[u8]) -> f32 {
        if value == 0.0 {
            return 0.0;
        }

        let modulus: i32 = calculate_modulus(key);
        if modulus == 1 {
            return value;
        }
        (value / modulus as f32) / 10000.0
    }

    pub fn convert_double(value: f64, key: &[u8]) -> f64 {
        convert_float(value as f32, key) as f64
    }

    pub fn encrypt_float(value: f32, key: &[u8]) -> f32 {
        if value == 0.0 {
            return 0.0;
        }

        let modulus: i32 = calculate_modulus(key);
        if modulus == 1 {
            return value;
        }
        (value * 10000.0) * modulus as f32
    }

    pub fn encrypt_double(value: f64, key: &[u8]) -> f64 {
        encrypt_float(value as f32, key) as f64
    }

    pub fn create_key(bytes: &[u8]) -> [u8; 8] {
        let seed: u32 = calculate_xxhash(bytes);
        let mut rng: Mt = Mt::new(seed);
        let mut buf: [u8; 8] = [0u8; 8];
        next_bytes(&mut rng, &mut buf);
        buf
    }

    pub fn convert_string(value: &str, key: &[u8]) -> Result<String> {
        let mut raw: Vec<u8> = general_purpose::STANDARD.decode(value.as_bytes())?;
        let bytes: Vec<u8> = xor_with_key(&mut raw, key);
        let utf16_bytes: Vec<u16> = bytes.chunks_exact(2).map(|x| u16::from_le_bytes([x[0], x[1]])).collect::<Vec<u16>>();
        match String::from_utf16(&utf16_bytes) {
            Ok(s) => Ok(s),
            Err(_) => Ok(bytes.iter().map(|x| *x as char).collect::<String>()),
        }
    }

    pub fn new_encrypt_string(value: &str, key: &[u8]) -> Result<String> {
        if value.is_empty() || value.len() < 8 {
            return Ok(value.to_string());
        }
        let mut raw: Vec<u8> = value.encode_utf16().flat_map(|x| x.to_le_bytes()).collect::<Vec<u8>>();
        let xor: Vec<u8> = xor_with_key(&mut raw, key);
        Ok(general_purpose::STANDARD.encode(&xor))
    }
}