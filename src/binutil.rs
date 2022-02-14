use std::io::Read;

/// Parse a 16-bit big-endian value from a byte array
pub fn parse_u16(array: &[u8]) -> u16 {
    return ((array[0] as u16) << 8)
        | ((array[1] as u16) << 0);
}

/// Parse a 24-bit big-endian value from a byte array
pub fn parse_u24(array: &[u8]) -> u32 {
    return ((array[0] as u32) << 16)
        | ((array[1] as u32) << 8)
        | ((array[2] as u32) << 0);
}

/// Parse a 32-bit big-endian value from a byte array
pub fn parse_u32(array: &[u8]) -> u32 {
    return ((array[0] as u32) << 24)
        | ((array[1] as u32) << 16)
        | ((array[2] as u32) << 8)
        | ((array[3] as u32) << 0);
}

/// Parse a 64-bit big-endian value from a byte array
pub fn parse_u64(array: &[u8]) -> u64 {
    return ((array[0] as u64) << 56)
        | ((array[1] as u64) << 48)
        | ((array[2] as u64) << 40)
        | ((array[3] as u64) << 32)
        | ((array[4] as u64) << 24)
        | ((array[5] as u64) << 16)
        | ((array[6] as u64) << 8)
        | ((array[7] as u64) << 0);
}

/// Parse a 32-bit big-endian float from a byte array
pub fn parse_f32(array: &[u8]) -> f32 {
    return f32::from_bits(parse_u32(array));
}

/// Parse a 64-bit big-endian float from a byte array
pub fn parse_f64(array: &[u8]) -> f64 {
    return f64::from_bits(parse_u64(array));
}

/// Parse a UTF-8 string from a byte array
pub fn parse_string(array: &[u8]) -> String {
    String::from_utf8_lossy(array).to_string()
}

/// Read a 8-bit big-endian value from a buffer
pub fn buf_read_u8(buffer: &mut dyn Read) -> u8 {
    let mut value: [u8; 1] = [0; 1];
    buffer.read_exact(&mut value).expect("expected 1 byte");
    return value[0] as u8;
}

/// Read a 16-bit big-endian value from a buffer
pub fn buf_read_u16(buffer: &mut dyn Read) -> u16 {
    let mut value: [u8; 2] = [0; 2];
    buffer.read_exact(&mut value).expect("expected 2 bytes");
    return parse_u16(&value) as u16;
}

/// Read a 32-bit big-endian value from a buffer
pub fn buf_read_u32(buffer: &mut dyn Read) -> u32 {
    let mut value: [u8; 4] = [0; 4];
    buffer.read_exact(&mut value).expect("expected 4 bytes");
    return parse_u32(&value) as u32;
}

/// Read a 64-bit big-endian value from a buffer
pub fn buf_read_u64(buffer: &mut dyn Read) -> u64 {
    let mut value: [u8; 8] = [0; 8];
    buffer.read_exact(&mut value).expect("expected 8 bytes");
    return parse_u64(&value) as u64;
}

/// Read a 32-bit big-endian float from a buffer
pub fn buf_read_f32(buffer: &mut dyn Read) -> f32 {
    let mut value: [u8; 4] = [0; 4];
    buffer.read_exact(&mut value).expect("expected 4 bytes");
    return parse_f32(&value) as f32;
}

/// Read a 64-bit big-endian float from a buffer
pub fn buf_read_f64(buffer: &mut dyn Read) -> f64 {
    let mut value: [u8; 8] = [0; 8];
    buffer.read_exact(&mut value).expect("expected 8 bytes");
    return parse_f64(&value) as f64;
}
