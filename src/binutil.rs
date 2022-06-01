pub fn parse_u24(array: &[u8]) -> u32 {
    return ((array[0] as u32) << 16)
        | ((array[1] as u32) << 8)
        | ((array[2] as u32) << 0);
}

pub fn parse_u32(array: &[u8]) -> u32 {
    return ((array[0] as u32) << 24)
        | ((array[1] as u32) << 16)
        | ((array[2] as u32) << 8)
        | ((array[3] as u32) << 0);
}

pub fn buf_write_u8(buffer: &mut [u8], value: u8) {
    buffer[0] = value as u8;
}

pub fn buf_write_u24(buffer: &mut [u8], value: u32) {
    buffer[0] = ((value >> 16) & 0xFF) as u8;
    buffer[1] = ((value >> 8) & 0xFF) as u8;
    buffer[2] = ((value >> 0) & 0xFF) as u8;
}

pub fn buf_write_u32(buffer: &mut [u8], value: u32) {
    buffer[0] = ((value >> 24) & 0xFF) as u8;
    buffer[1] = ((value >> 16) & 0xFF) as u8;
    buffer[2] = ((value >> 8) & 0xFF) as u8;
    buffer[3] = ((value >> 0) & 0xFF) as u8;
}
