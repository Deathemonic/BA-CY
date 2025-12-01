const ENCRYPTION_KEY: u32 = 2948064217;

#[inline]
pub fn encrypt(data: &mut [u8], offset: usize, length: usize) {
    if offset >= data.len() || length == 0 {
        return;
    }

    let end = (offset + length).min(data.len());
    let key_byte = (ENCRYPTION_KEY & 0xFF) as u8;

    data[offset..end].iter_mut().for_each(|byte| *byte ^= key_byte);
}

#[inline]
pub fn encrypt_with_key(data: &[u8], key: &[u8]) -> Option<Vec<u8>> {
    if data.is_empty() || key.is_empty() {
        return None;
    }

    Some(
        data.iter()
            .zip(key.iter().cycle())
            .map(|(d, k)| d ^ k)
            .collect(),
    )
}

#[inline]
pub fn xor_exact(value: &[u8], key: &[u8]) -> Vec<u8> {
    value.iter().zip(key.iter()).map(|(a, b)| a ^ b).collect()
}

#[inline]
pub fn xor_inplace(data: &mut [u8], key: &[u8]) {
    data.iter_mut()
        .zip(key.iter().cycle())
        .for_each(|(d, k)| *d ^= k);
}
