/// Number tags are numeric primitive values

use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Read;

use crate::binutil::{buf_read_f32, buf_read_f64, buf_read_u16, buf_read_u32, buf_read_u64, buf_read_u8};
use crate::format_number;
use crate::nbt::{NbtTag, TagHeader};

pub struct NbtTagByte {
    name: String,
    value: i8,
}

impl NbtTag for NbtTagByte {
    fn new(header: TagHeader, buffer: &mut dyn Read) -> Box<Self> where Self: Sized {
        let value = buf_read_u8(buffer) as i8;

        return Box::new(
            NbtTagByte {
                name: header.name,
                value,
            }
        );
    }

    fn tag_type(&self) -> u8 {
        1
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Display for NbtTagByte {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        format_number!(self, formatter)
    }
}

pub struct NbtTagShort {
    name: String,
    value: i16,
}

impl NbtTag for NbtTagShort {
    fn new(header: TagHeader, buffer: &mut dyn Read) -> Box<Self> where Self: Sized {
        let value = buf_read_u16(buffer) as i16;

        return Box::new(
            NbtTagShort {
                name: header.name,
                value,
            }
        );
    }

    fn tag_type(&self) -> u8 {
        2
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Display for NbtTagShort {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        format_number!(self, formatter)
    }
}

pub struct NbtTagInt {
    name: String,
    value: i32,
}

impl NbtTag for NbtTagInt {
    fn new(header: TagHeader, buffer: &mut dyn Read) -> Box<Self> where Self: Sized {
        let value = buf_read_u32(buffer) as i32;

        return Box::new(
            NbtTagInt {
                name: header.name,
                value,
            }
        );
    }

    fn tag_type(&self) -> u8 {
        3
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Display for NbtTagInt {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        format_number!(self, formatter)
    }
}

pub struct NbtTagLong {
    name: String,
    value: i64,
}

impl NbtTag for NbtTagLong {
    fn new(header: TagHeader, buffer: &mut dyn Read) -> Box<Self> where Self: Sized {
        let value = buf_read_u64(buffer) as i64;

        return Box::new(
            NbtTagLong {
                name: header.name,
                value,
            }
        );
    }

    fn tag_type(&self) -> u8 {
        4
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Display for NbtTagLong {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        format_number!(self, formatter)
    }
}

pub struct NbtTagFloat {
    name: String,
    value: f32,
}

impl NbtTag for NbtTagFloat {
    fn new(header: TagHeader, buffer: &mut dyn Read) -> Box<Self> where Self: Sized {
        let value = buf_read_f32(buffer);

        return Box::new(
            NbtTagFloat {
                name: header.name,
                value,
            }
        );
    }

    fn tag_type(&self) -> u8 {
        5
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Display for NbtTagFloat {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        format_number!(self, formatter)
    }
}

pub struct NbtTagDouble {
    name: String,
    value: f64,
}

impl NbtTag for NbtTagDouble {
    fn new(header: TagHeader, buffer: &mut dyn Read) -> Box<Self> where Self: Sized {
        let value = buf_read_f64(buffer);

        return Box::new(
            NbtTagDouble {
                name: header.name,
                value,
            }
        );
    }

    fn tag_type(&self) -> u8 {
        6
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Display for NbtTagDouble {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        format_number!(self, formatter)
    }
}
