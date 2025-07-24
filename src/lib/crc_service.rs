use crate::lib::hash::{evaluate_crc32, CrcResult};
use crate::table_encryption_service::xor_str;

use anyhow::{anyhow, Context, Result};
use crc32fast::Hasher;
use std::convert::TryInto;
use std::fs;
use std::path::PathBuf;

fn bytes_to_u32_be(bytes: &[u8]) -> u32 {
    u32::from_be_bytes(bytes.try_into().expect("Input slice must be 4 bytes long"))
}

fn u32_to_bytes_be(value: u32) -> Vec<u8> {
    value.to_be_bytes().to_vec()
}

fn reverse_bits_in_bytes(byte_array: &[u8]) -> Vec<u8> {
    let num = bytes_to_u32_be(byte_array);
    let reversed_num = num.reverse_bits();
    u32_to_bytes_be(reversed_num)
}

fn hex_string_to_bytes(hex: &str) -> Result<Vec<u8>> {
    if hex.len() % 2 != 0 {
        return Err(anyhow!("Hex string must have an even length"));
    }

    (0..hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex[i..i + 2], 16)
                .with_context(|| format!("Invalid hex string: {}", hex))
        })
        .collect()
}

fn gf_multiply(mut a: u64, mut b: u64) -> u64 {
    let mut result = 0u64;

    while b != 0 {
        if b & 1 != 0 {
            result ^= a;
        }
        a <<= 1;
        b >>= 1;
    }

    result
}

fn gf_divide(dividend: u64, divisor: u64) -> u64 {
    if divisor == 0 {
        return 0;
    }

    let mut quotient = 0u64;
    let mut remainder = dividend;

    let divisor_bits = 64 - divisor.leading_zeros() as usize;

    while remainder != 0 {
        let remainder_bits = 64 - remainder.leading_zeros() as usize;
        if remainder_bits < divisor_bits {
            break;
        }

        let shift = remainder_bits - divisor_bits;
        quotient |= 1u64 << shift;
        remainder ^= divisor << shift;
    }

    quotient
}

fn gf_mod(mut dividend: u64, divisor: u64, n: usize) -> u64 {
    if divisor == 0 {
        return dividend;
    }

    let mask = if n >= 64 { u64::MAX } else { (1u64 << n) - 1 };
    let divisor_bits = 64 - divisor.leading_zeros() as usize;

    while dividend != 0 {
        let dividend_bits = 64 - dividend.leading_zeros() as usize;
        if dividend_bits < divisor_bits {
            break;
        }

        let shift = dividend_bits - divisor_bits;
        dividend ^= divisor << shift;
    }

    dividend & mask
}

fn gf_multiply_modular(a: u64, b: u64, modulus: u64, n: usize) -> u64 {
    let product = gf_multiply(a, b);
    gf_mod(product, modulus, n)
}

fn gf_modular_inverse(a: u64, m: u64) -> Result<u64> {
    if a == 0 {
        return Err(anyhow!("Cannot find inverse of zero"));
    }

    let (mut old_r, mut r) = (m, a);
    let (mut old_s, mut s) = (0u64, 1u64);

    while r != 0 {
        let quotient = gf_divide(old_r, r);
        let temp = r;
        r = old_r ^ gf_multiply(quotient, r);
        old_r = temp;

        let temp = s;
        s = old_s ^ gf_multiply(quotient, s);
        old_s = temp;
    }

    if old_r != 1 {
        return Err(anyhow!("Modular inverse does not exist"));
    }

    Ok(old_s)
}

fn gf_inverse(k: u32, poly: u64) -> Result<u32> {
    let x32 = 0x100000000u64;
    let inverse = gf_modular_inverse(x32, poly)?;
    let result = gf_multiply_modular(k as u64, inverse, poly, 32);
    Ok(result as u32)
}

fn calculate_gf_modular_inverse(k: u32) -> Result<u32> {
    let crc32_poly = 0x104C11DB7u64;
    gf_inverse(k, crc32_poly)
}

fn save_bytes_to_file(byte_array: &[u8], file_path: &PathBuf) -> Result<()> {
    fs::write(file_path, byte_array).with_context(|| format!("Error saving file: {:?}", file_path))?;
    Ok(())
}

pub fn manipulate_crc(original: &PathBuf, modified: &PathBuf) -> Result<bool> {
    let original_data = fs::read(original).with_context(|| format!("Error reading file '{:?}'", original))?;
    let modified_data = fs::read(modified).with_context(|| format!("Error reading file '{:?}'", modified))?;

    let original_crc = evaluate_crc32(&original_data);
    
    let mut modified_crc_hasher = Hasher::new();
    modified_crc_hasher.update(&modified_data);
    modified_crc_hasher.update(&[0, 0, 0, 0]);
    let modified_crc = CrcResult::new(modified_crc_hasher.finalize());

    let original_bytes = hex_string_to_bytes(&original_crc.hex)?;
    let modified_bytes = hex_string_to_bytes(&modified_crc.hex)?;
    let xor_result = xor_str(&original_bytes, &modified_bytes);

    let reversed_bytes = reverse_bits_in_bytes(&xor_result);
    let k = bytes_to_u32_be(&reversed_bytes) as u64;

    let correction_value = calculate_gf_modular_inverse(k as u32)?;

    let correction_bytes_raw = u32_to_bytes_be(correction_value);
    let final_result: Vec<u8> = correction_bytes_raw.iter().map(|b| b.reverse_bits()).collect();

    let mut final_data = modified_data;
    final_data.extend_from_slice(&final_result);
    
    let final_crc = evaluate_crc32(&final_data);

    let is_crc_match = final_crc.value == original_crc.value;
    if is_crc_match {
        save_bytes_to_file(&final_data, modified)?;
    }
    
    Ok(is_crc_match)
}