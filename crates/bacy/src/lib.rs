pub mod crypto;
pub mod error;
pub mod hash;
pub mod math;
pub mod utils;

pub mod table_encryption_service {
    pub use crate::crypto::table::{
        create_key,
        decrypt_enum as convert_enum,
        decrypt_f32 as convert_float,
        decrypt_f64 as convert_double,
        decrypt_i32 as convert_int,
        decrypt_i64 as convert_long,
        decrypt_string as convert_string,
        decrypt_u32 as convert_uint,
        decrypt_u64 as convert_ulong,
        encrypt_f32 as encrypt_float,
        encrypt_f64 as encrypt_double,
        encrypt_string
    };
}
