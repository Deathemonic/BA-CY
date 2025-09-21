use crate::error::MemoryPackError;

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};

pub fn read_string(cursor: &mut Cursor<&[u8]>) -> Result<String, MemoryPackError> {
    let length_i32 = read_i32(cursor)?;

    if length_i32 < 0 {
        return Err(MemoryPackError::InvalidLength(length_i32));
    }

    let length: usize = length_i32 as usize;
    let mut buffer: Vec<u8> = vec![0; length];
    cursor.read_exact(&mut buffer)?;

    let string = String::from_utf8(buffer)?;
    Ok(string)
}

pub fn read_bool(cursor: &mut Cursor<&[u8]>) -> Result<bool, MemoryPackError> {
    Ok(cursor.read_u8()? == 1)
}

pub fn read_i8(cursor: &mut Cursor<&[u8]>) -> Result<i8, MemoryPackError> {
    Ok(cursor.read_i8()?)
}

pub fn read_i16(cursor: &mut Cursor<&[u8]>) -> Result<i16, MemoryPackError> {
    Ok(cursor.read_i16::<LittleEndian>()?)
}

pub fn read_i32(cursor: &mut Cursor<&[u8]>) -> Result<i32, MemoryPackError> {
    Ok(cursor.read_i32::<LittleEndian>()?)
}

pub fn read_i64(cursor: &mut Cursor<&[u8]>) -> Result<i64, MemoryPackError> {
    Ok(cursor.read_i64::<LittleEndian>()?)
}
