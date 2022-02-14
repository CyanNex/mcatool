use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Read;

use crate::binutil::{buf_read_u16, parse_string};
use crate::format_string;
use crate::nbt::{NbtTag, TagHeader};

/// A UTF-8 string. It has a size, rather than being null terminated.
pub struct NbtTagString {
    name: String,
    value: String,
}

impl NbtTag for NbtTagString {
    fn new(header: TagHeader, buffer: &mut dyn Read) -> Box<Self> where Self: Sized {
        // a string tag starts with a 16-bit length value
        let length = buf_read_u16(buffer) as usize;

        // create a buffer for holding the raw value
        let mut value = vec![0; length];
        // read the value into the new buffer
        buffer.read_exact(&mut value).unwrap();
        // convert the value to a string
        let value = parse_string(value.as_slice());

        return Box::new(
            NbtTagString {
                name: header.name,
                value,
            }
        );
    }

    fn tag_type(&self) -> u8 {
        8
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Display for NbtTagString {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        format_string!(self, formatter)
    }
}