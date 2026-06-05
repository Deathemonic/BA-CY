use aes::cipher::block_padding::Pkcs7;
use aes::cipher::{BlockModeDecrypt, KeyIvInit};

use crate::error::AesError;

type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

pub fn decrypt(data: &[u8], key: &[u8; 32], iv: &[u8; 16]) -> Result<Vec<u8>, AesError> {
    Aes256CbcDec::new(key.into(), iv.into())
        .decrypt_padded_vec::<Pkcs7>(data)
        .map_err(|_| AesError::Decrypt)
}
