//! # Plain C ABI
//!
//! Hand-written `extern "C"` wrappers exposing `bacy`'s API with no UniFFI
//! dependency. Always compiled — this is the default, unconditional layer.
//!
//! **For Rust users:** depend on `bacy` directly and use its native API.
//! **For UniFFI-based bindings (Kotlin, Swift, Python, etc.):** build with
//! the `uniffi` feature and use the generated bindings instead.
//!
//! Error convention: fallible functions return `i32` (`0` = success,
//! nonzero = error code, see `BacyErrorCode`). Functions producing an
//! owned value on success write it through an out-parameter and only do
//! so when the return code is `0`. Structured error detail (e.g. CRC
//! mismatch values) is available via a paired `_error` out-param of a
//! `#[repr(C)]` struct, avoiding string allocation for known-shape errors.

#![allow(unsafe_op_in_unsafe_fn)]

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::panic::catch_unwind;
use std::path::Path;
use std::ptr;

#[repr(i32)]
pub enum BacyErrorCode {
    Success = 0,
    Io = 1,
    InvalidPath = 2,
    Mismatch = 3,
    Base64Decode = 4,
    FromUtf16 = 5,
    StringConversion = 6,
    PanicUnwind = -1,
    NullPointer = -2
}

impl From<&bacy::error::HashError> for BacyErrorCode {
    fn from(e: &bacy::error::HashError) -> Self {
        match e {
            bacy::error::HashError::Io(_) => BacyErrorCode::Io,
            bacy::error::HashError::InvalidPath => BacyErrorCode::InvalidPath,
            bacy::error::HashError::Mismatch { .. } => BacyErrorCode::Mismatch
        }
    }
}

impl From<&bacy::error::TableEncryptionError> for BacyErrorCode {
    fn from(e: &bacy::error::TableEncryptionError) -> Self {
        match e {
            bacy::error::TableEncryptionError::Base64Decode(_) => BacyErrorCode::Base64Decode,
            bacy::error::TableEncryptionError::FromUtf16Error(_) => BacyErrorCode::FromUtf16,
            bacy::error::TableEncryptionError::StringConversionFailed => {
                BacyErrorCode::StringConversion
            }
        }
    }
}

#[repr(C)]
pub struct BacyCrcMismatch {
    pub expected: u32,
    pub actual: u32
}

#[repr(C)]
pub struct BacyBytes {
    pub ptr: *mut u8,
    pub len: usize,
    pub cap: usize
}

impl BacyBytes {
    fn from_vec(mut v: Vec<u8>) -> Self {
        let ptr = v.as_mut_ptr();
        let len = v.len();
        let cap = v.capacity();
        std::mem::forget(v);
        BacyBytes { ptr, len, cap }
    }

    fn null() -> Self {
        BacyBytes {
            ptr: ptr::null_mut(),
            len: 0,
            cap: 0
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_free_bytes(b: BacyBytes) {
    if !b.ptr.is_null() {
        drop(Vec::from_raw_parts(b.ptr, b.len, b.cap));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_free_string(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}

unsafe fn str_from_ptr<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() {
        return None;
    }
    CStr::from_ptr(ptr).to_str().ok()
}

fn to_c_string(s: String) -> *mut c_char {
    CString::new(s).map(CString::into_raw).unwrap_or(ptr::null_mut())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_crc_compute_streaming(
    path: *const c_char,
    buffer_size: u64,
    out_value: *mut u32
) -> i32 {
    let Some(path) = str_from_ptr(path) else {
        return BacyErrorCode::NullPointer as i32;
    };
    let result = catch_unwind(|| {
        bacy::hash::crc::compute_streaming(Path::new(path), buffer_size as usize, None)
    });
    match result {
        Ok(Ok(value)) => {
            *out_value = value;
            BacyErrorCode::Success as i32
        }
        Ok(Err(e)) => BacyErrorCode::from(&e) as i32,
        Err(_) => BacyErrorCode::PanicUnwind as i32
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_crc_compute_bytes(data: *const u8, len: usize) -> u32 {
    if data.is_null() || len == 0 {
        return 0;
    }
    let slice = std::slice::from_raw_parts(data, len);
    catch_unwind(|| bacy::hash::crc::compute_bytes(slice, None)).unwrap_or(0)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_crc_compare(
    path: *const c_char,
    expected_crc: u32,
    out_mismatch: *mut BacyCrcMismatch
) -> i32 {
    let Some(path) = str_from_ptr(path) else {
        return BacyErrorCode::NullPointer as i32;
    };
    let result = catch_unwind(|| bacy::hash::crc::compare(Path::new(path), expected_crc));
    match result {
        Ok(Ok(())) => BacyErrorCode::Success as i32,
        Ok(Err(e)) => {
            if let bacy::error::HashError::Mismatch { expected, actual } = e {
                if !out_mismatch.is_null() {
                    *out_mismatch = BacyCrcMismatch { expected, actual };
                }
            }
            BacyErrorCode::from(&e) as i32
        }
        Err(_) => BacyErrorCode::PanicUnwind as i32
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_crc_evaluate(data: *const u8, len: usize) -> u32 {
    bacy_crc_compute_bytes(data, len)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_crc_forge(file_path: *const c_char, target_crc: u32) -> i32 {
    let Some(file_path) = str_from_ptr(file_path) else {
        return BacyErrorCode::NullPointer as i32;
    };
    let result = catch_unwind(|| {
        let manipulator = bacy::utils::crc_manipulator::CrcManipulator::new(file_path);
        manipulator.forge_crc(target_crc)
    });
    match result {
        Ok(Ok(())) => BacyErrorCode::Success as i32,
        Ok(Err(e)) => BacyErrorCode::from(&e) as i32,
        Err(_) => BacyErrorCode::PanicUnwind as i32
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_crc_match_file(
    file_path: *const c_char,
    target_file_path: *const c_char
) -> i32 {
    let (Some(file_path), Some(target_file_path)) =
        (str_from_ptr(file_path), str_from_ptr(target_file_path))
    else {
        return BacyErrorCode::NullPointer as i32;
    };
    let result = catch_unwind(|| {
        let manipulator = bacy::utils::crc_manipulator::CrcManipulator::new(file_path);
        manipulator.match_file(Path::new(target_file_path))
    });
    match result {
        Ok(Ok(())) => BacyErrorCode::Success as i32,
        Ok(Err(e)) => BacyErrorCode::from(&e) as i32,
        Err(_) => BacyErrorCode::PanicUnwind as i32
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_md5_to_hex_string(data: *const u8, len: usize) -> *mut c_char {
    if data.is_null() {
        return ptr::null_mut();
    }
    let slice = std::slice::from_raw_parts(data, len);
    match catch_unwind(|| bacy::crypto::md5::to_hex_string(slice)) {
        Ok(s) => to_c_string(s),
        Err(_) => ptr::null_mut()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_md5_compute_hash(data: *const u8, len: usize) -> BacyBytes {
    if data.is_null() {
        return BacyBytes::null();
    }
    let slice = std::slice::from_raw_parts(data, len);
    match catch_unwind(|| bacy::crypto::md5::compute_hash(slice).to_vec()) {
        Ok(v) => BacyBytes::from_vec(v),
        Err(_) => BacyBytes::null()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_md5_compute_hash_hmac(
    data: *const u8,
    data_len: usize,
    key: *const u8,
    key_len: usize
) -> BacyBytes {
    if data.is_null() || key.is_null() {
        return BacyBytes::null();
    }
    let data = std::slice::from_raw_parts(data, data_len);
    let key = std::slice::from_raw_parts(key, key_len);
    match catch_unwind(|| bacy::crypto::md5::compute_hash_hmac(data, key).to_vec()) {
        Ok(v) => BacyBytes::from_vec(v),
        Err(_) => BacyBytes::null()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_md5_compute_hash_str(source: *const c_char) -> *mut c_char {
    let Some(source) = str_from_ptr(source) else { return ptr::null_mut() };
    match catch_unwind(|| bacy::crypto::md5::compute_hash_str(source)) {
        Ok(s) => to_c_string(s),
        Err(_) => ptr::null_mut()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_md5_compute_hash_str_hmac(
    source: *const c_char,
    key: *const c_char
) -> *mut c_char {
    let (Some(source), Some(key)) = (str_from_ptr(source), str_from_ptr(key)) else {
        return ptr::null_mut();
    };
    match catch_unwind(|| bacy::crypto::md5::compute_hash_str_hmac(source, key)) {
        Ok(s) => to_c_string(s),
        Err(_) => ptr::null_mut()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_md5_compute_digest(source: *const c_char) -> u32 {
    let Some(source) = str_from_ptr(source) else { return 0 };
    catch_unwind(|| bacy::crypto::md5::compute_digest(source)).unwrap_or(0)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_md5_compute_digest_hmac(
    source: *const c_char,
    key: *const c_char
) -> u32 {
    let (Some(source), Some(key)) = (str_from_ptr(source), str_from_ptr(key)) else {
        return 0;
    };
    catch_unwind(|| bacy::crypto::md5::compute_digest_hmac(source, key)).unwrap_or(0)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_md5_compute_digest64(source: *const c_char) -> u64 {
    let Some(source) = str_from_ptr(source) else { return 0 };
    catch_unwind(|| bacy::crypto::md5::compute_digest64(source)).unwrap_or(0)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_md5_compute_digest64_hmac(
    source: *const c_char,
    key: *const c_char
) -> u64 {
    let (Some(source), Some(key)) = (str_from_ptr(source), str_from_ptr(key)) else {
        return 0;
    };
    catch_unwind(|| bacy::crypto::md5::compute_digest64_hmac(source, key)).unwrap_or(0)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_md5_compute_head(source: *const c_char) -> *mut c_char {
    let Some(source) = str_from_ptr(source) else { return ptr::null_mut() };
    match catch_unwind(|| bacy::crypto::md5::compute_head(source)) {
        Ok(s) => to_c_string(s),
        Err(_) => ptr::null_mut()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_sha_compute(data: *const u8, len: usize) -> BacyBytes {
    if data.is_null() {
        return BacyBytes::null();
    }
    let slice = std::slice::from_raw_parts(data, len);
    match catch_unwind(|| bacy::hash::sha::compute(slice).to_vec()) {
        Ok(v) => BacyBytes::from_vec(v),
        Err(_) => BacyBytes::null()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_sha_compute_str(source: *const c_char) -> BacyBytes {
    let Some(source) = str_from_ptr(source) else { return BacyBytes::null() };
    match catch_unwind(|| bacy::hash::sha::compute_str(source).to_vec()) {
        Ok(v) => BacyBytes::from_vec(v),
        Err(_) => BacyBytes::null()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_xxhash_calculate_hash(data: *const u8, len: usize) -> u32 {
    if data.is_null() {
        return 0;
    }
    let slice = std::slice::from_raw_parts(data, len);
    catch_unwind(|| bacy::hash::xxhash::calculate_hash(slice)).unwrap_or(0)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_xxhash_calculate_hash_str(s: *const c_char) -> u32 {
    let Some(s) = str_from_ptr(s) else { return 0 };
    catch_unwind(|| bacy::hash::xxhash::calculate_hash_str(s)).unwrap_or(0)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_xxhash_calculate_hash64(data: *const u8, len: usize) -> u64 {
    if data.is_null() {
        return 0;
    }
    let slice = std::slice::from_raw_parts(data, len);
    catch_unwind(|| bacy::hash::xxhash::calculate_hash64(slice)).unwrap_or(0)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_xxhash_calculate_hash64_str(s: *const c_char) -> u64 {
    let Some(s) = str_from_ptr(s) else { return 0 };
    catch_unwind(|| bacy::hash::xxhash::calculate_hash64_str(s)).unwrap_or(0)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_xor_encrypt(
    data: *mut u8,
    len: usize,
    offset: u64,
    length: u64
) -> i32 {
    if data.is_null() {
        return BacyErrorCode::NullPointer as i32;
    }
    let slice = std::slice::from_raw_parts_mut(data, len);
    let result = catch_unwind(std::panic::AssertUnwindSafe(|| {
        bacy::crypto::xor::encrypt(slice, offset as usize, length as usize);
    }));
    match result {
        Ok(()) => BacyErrorCode::Success as i32,
        Err(_) => BacyErrorCode::PanicUnwind as i32
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_xor_encrypt_with_key(
    data: *const u8,
    data_len: usize,
    key: *const u8,
    key_len: usize
) -> BacyBytes {
    if data.is_null() || key.is_null() {
        return BacyBytes::null();
    }
    let data = std::slice::from_raw_parts(data, data_len);
    let key = std::slice::from_raw_parts(key, key_len);
    match catch_unwind(|| bacy::crypto::xor::encrypt_with_key(data, key)) {
        Ok(Some(v)) => BacyBytes::from_vec(v),
        Ok(None) | Err(_) => BacyBytes::null()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_xor_exact(
    value: *const u8,
    value_len: usize,
    key: *const u8,
    key_len: usize
) -> BacyBytes {
    if value.is_null() || key.is_null() {
        return BacyBytes::null();
    }
    let value = std::slice::from_raw_parts(value, value_len);
    let key = std::slice::from_raw_parts(key, key_len);
    match catch_unwind(|| bacy::crypto::xor::xor_exact(value, key)) {
        Ok(v) => BacyBytes::from_vec(v),
        Err(_) => BacyBytes::null()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_xor_inplace_bytes(
    data: *mut u8,
    len: usize,
    key: *const u8,
    key_len: usize
) -> i32 {
    if data.is_null() || key.is_null() {
        return BacyErrorCode::NullPointer as i32;
    }
    let slice = std::slice::from_raw_parts_mut(data, len);
    let key = std::slice::from_raw_parts(key, key_len);
    let result =
        catch_unwind(std::panic::AssertUnwindSafe(|| bacy::crypto::xor::xor_inplace(slice, key)));
    match result {
        Ok(()) => BacyErrorCode::Success as i32,
        Err(_) => BacyErrorCode::PanicUnwind as i32
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_table_create_key(name: *const c_char) -> BacyBytes {
    let Some(name) = str_from_ptr(name) else { return BacyBytes::null() };
    match catch_unwind(|| bacy::crypto::table::create_key(name).to_vec()) {
        Ok(v) => BacyBytes::from_vec(v),
        Err(_) => BacyBytes::null()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_table_create_password(
    key: *const c_char,
    length: u64
) -> *mut c_char {
    let Some(key) = str_from_ptr(key) else { return ptr::null_mut() };
    match catch_unwind(|| bacy::crypto::table::create_password(key, length as usize)) {
        Ok(s) => to_c_string(s),
        Err(_) => ptr::null_mut()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_table_xor(name: *const c_char, data: *mut u8, len: usize) -> i32 {
    let Some(name) = str_from_ptr(name) else {
        return BacyErrorCode::NullPointer as i32;
    };
    if data.is_null() {
        return BacyErrorCode::NullPointer as i32;
    }
    let slice = std::slice::from_raw_parts_mut(data, len);
    let result =
        catch_unwind(std::panic::AssertUnwindSafe(|| bacy::crypto::table::xor(name, slice)));
    match result {
        Ok(()) => BacyErrorCode::Success as i32,
        Err(_) => BacyErrorCode::PanicUnwind as i32
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_table_decrypt_i32(value: i32, key: *const u8, key_len: usize) -> i32 {
    if key.is_null() {
        return value;
    }
    let key = std::slice::from_raw_parts(key, key_len);
    catch_unwind(|| bacy::crypto::table::decrypt_i32(value, key)).unwrap_or(value)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_table_decrypt_i64(value: i64, key: *const u8, key_len: usize) -> i64 {
    if key.is_null() {
        return value;
    }
    let key = std::slice::from_raw_parts(key, key_len);
    catch_unwind(|| bacy::crypto::table::decrypt_i64(value, key)).unwrap_or(value)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_table_decrypt_u32(value: u32, key: *const u8, key_len: usize) -> u32 {
    if key.is_null() {
        return value;
    }
    let key = std::slice::from_raw_parts(key, key_len);
    catch_unwind(|| bacy::crypto::table::decrypt_u32(value, key)).unwrap_or(value)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_table_decrypt_u64(value: u64, key: *const u8, key_len: usize) -> u64 {
    if key.is_null() {
        return value;
    }
    let key = std::slice::from_raw_parts(key, key_len);
    catch_unwind(|| bacy::crypto::table::decrypt_u64(value, key)).unwrap_or(value)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_table_decrypt_f32(value: f32, key: *const u8, key_len: usize) -> f32 {
    if key.is_null() {
        return value;
    }
    let key = std::slice::from_raw_parts(key, key_len);
    catch_unwind(|| bacy::crypto::table::decrypt_f32(value, key)).unwrap_or(value)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_table_decrypt_f64(value: f64, key: *const u8, key_len: usize) -> f64 {
    if key.is_null() {
        return value;
    }
    let key = std::slice::from_raw_parts(key, key_len);
    catch_unwind(|| bacy::crypto::table::decrypt_f64(value, key)).unwrap_or(value)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_table_decrypt_string(
    value: *const c_char,
    key: *const u8,
    key_len: usize,
    out: *mut *mut c_char
) -> i32 {
    let Some(value) = str_from_ptr(value) else {
        return BacyErrorCode::NullPointer as i32;
    };
    if key.is_null() || out.is_null() {
        return BacyErrorCode::NullPointer as i32;
    }
    let key = std::slice::from_raw_parts(key, key_len);
    let result = catch_unwind(|| bacy::crypto::table::decrypt_string(value, key));
    match result {
        Ok(Ok(s)) => {
            *out = to_c_string(s);
            BacyErrorCode::Success as i32
        }
        Ok(Err(e)) => BacyErrorCode::from(&e) as i32,
        Err(_) => BacyErrorCode::PanicUnwind as i32
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_table_encrypt_f32(value: f32, key: *const u8, key_len: usize) -> f32 {
    if key.is_null() {
        return value;
    }
    let key = std::slice::from_raw_parts(key, key_len);
    catch_unwind(|| bacy::crypto::table::encrypt_f32(value, key)).unwrap_or(value)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_table_encrypt_f64(value: f64, key: *const u8, key_len: usize) -> f64 {
    if key.is_null() {
        return value;
    }
    let key = std::slice::from_raw_parts(key, key_len);
    catch_unwind(|| bacy::crypto::table::encrypt_f64(value, key)).unwrap_or(value)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_table_encrypt_string(
    value: *const c_char,
    key: *const u8,
    key_len: usize
) -> *mut c_char {
    let Some(value) = str_from_ptr(value) else { return ptr::null_mut() };
    if key.is_null() {
        return ptr::null_mut();
    }
    let key = std::slice::from_raw_parts(key, key_len);
    match catch_unwind(|| bacy::crypto::table::encrypt_string(value, key)) {
        Ok(s) => to_c_string(s),
        Err(_) => ptr::null_mut()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bacy_get_file_path(
    path: *const c_char,
    crc: i64,
    has_crc: bool,
    no_hash: bool,
    to_lower: bool
) -> *mut c_char {
    let Some(path) = str_from_ptr(path) else { return ptr::null_mut() };
    let crc = if has_crc { Some(crc) } else { None };
    let result = catch_unwind(|| {
        bacy::utils::strategy::get_file_path(path, crc, no_hash, to_lower)
            .to_string_lossy()
            .to_string()
    });
    match result {
        Ok(s) => to_c_string(s),
        Err(_) => ptr::null_mut()
    }
}
