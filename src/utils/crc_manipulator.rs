use crate::error::HashError;
use crate::hash::crc;

use std::fs;
use std::path::Path;

const POLY_NORMAL: u64 = 0x104C11DB7;
const POLY_DEGREE: i32 = 32;
const GF2_INVERSE_X32: u64 = 0xCBF1ACDA;

pub struct CrcManipulator {
    pub file_path: Box<Path>,
}

impl CrcManipulator {
    pub fn new(file_path: impl AsRef<Path>) -> Self {
        Self {
            file_path: file_path.as_ref().into(),
        }
    }

    #[inline]
    fn gf2_multiply_mod(mut a: u64, mut b: u64) -> u64 {
        let mut result = 0u64;

        while b != 0 {
            if b & 1 != 0 {
                result ^= a;
            }

            b >>= 1;
            a <<= 1;

            if a >> POLY_DEGREE != 0 {
                a ^= POLY_NORMAL;
            }
        }

        result
    }

    #[inline]
    fn reverse_bits(bytes: &[u8; 4]) -> [u8; 4] {
        u32::from_be_bytes(*bytes).reverse_bits().to_be_bytes()
    }

    fn forge_bytes(data: &[u8], target_crc: u32) -> [u8; 4] {
        let mut data_with_padding = data.to_vec();
        data_with_padding.extend_from_slice(&[0, 0, 0, 0]);
        let padded_crc = crc::compute_bytes(&data_with_padding);

        let xor_result = target_crc ^ padded_crc;
        let xor_bytes = xor_result.to_be_bytes();

        let reversed_bytes = Self::reverse_bits(&xor_bytes);

        let k = u32::from_be_bytes(reversed_bytes) as u64;

        let p_value = Self::gf2_multiply_mod(k, GF2_INVERSE_X32);
        let p_bytes = (p_value as u32).to_be_bytes();

        p_bytes.map(|b| b.reverse_bits())
    }

    pub fn forge_crc(&self, target_crc: u32) -> Result<(), HashError> {
        let data = fs::read(&self.file_path)?;

        if crc::compute_bytes(&data) == target_crc {
            return Ok(());
        }

        let patch = Self::forge_bytes(&data, target_crc);

        let mut new_data = data;
        new_data.extend_from_slice(&patch);

        fs::write(&self.file_path, &new_data)?;

        let new_crc = crc::compute_bytes(&new_data);

        match new_crc == target_crc {
            true => Ok(()),
            false => Err(HashError::Mismatch {
                expected: target_crc,
                actual: new_crc,
            }),
        }
    }

    pub fn match_file(&self, target_file: &Path) -> Result<(), HashError> {
        if !target_file.exists() {
            return Err(HashError::InvalidPath);
        }
        let target_crc = crc::compute_streaming(target_file, 0x2000)?;
        self.forge_crc(target_crc)
    }
}
