use std::any::Any;
/// Array tags are arrays of primitive values.

use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Read;

use crate::binutil::{buf_read_u32, buf_read_u64, buf_read_u8};
use crate::format_array;
use crate::nbt::{NbtTag, TagHeader};

pub struct NbtTagByteArray {
    name: String,
    values: Vec<i8>,
}

impl NbtTag for NbtTagByteArray {
    fn new(header: TagHeader, buffer: &mut dyn Read) -> Box<Self> where Self: Sized {
        // a byte array tag starts with a 32 bit length value
        let length = buf_read_u32(buffer) as usize;
        // create a list to hold the values
        let mut values = vec![0; length];
        // read all the values into the list
        for i in 0..length {
            values[i] = buf_read_u8(buffer) as i8;
        }

        return Box::new(
            NbtTagByteArray {
                name: header.name,
                values,
            }
        );
    }

    fn tag_type(&self) -> u8 {
        7
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for NbtTagByteArray {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        format_array!(self, formatter)
    }
}

pub struct NbtTagIntArray {
    name: String,
    values: Vec<i32>,
}

impl NbtTag for NbtTagIntArray {
    fn new(header: TagHeader, buffer: &mut dyn Read) -> Box<Self> where Self: Sized {
        // an int array tag starts with a 32 bit length value
        let length = buf_read_u32(buffer) as usize;
        // create a list to hold the values
        let mut values = vec![0; length];
        // read all the values into the list
        for i in 0..length {
            values[i] = buf_read_u32(buffer) as i32;
        }

        return Box::new(
            NbtTagIntArray {
                name: header.name,
                values,
            }
        );
    }

    fn tag_type(&self) -> u8 {
        11
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for NbtTagIntArray {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        format_array!(self, formatter)
    }
}

pub struct NbtTagLongArray {
    name: String,
    values: Vec<i64>,
}

impl NbtTag for NbtTagLongArray {
    fn new(header: TagHeader, buffer: &mut dyn Read) -> Box<Self> where Self: Sized {
        // a long array tag starts with a 32 bit length value
        let length = buf_read_u32(buffer) as usize;
        // create a list to hold the values
        let mut values = vec![0; length];
        // read all the values into the list
        for i in 0..length {
            values[i] = buf_read_u64(buffer) as i64;
        }

        return Box::new(
            NbtTagLongArray {
                name: header.name,
                values,
            }
        );
    }

    fn tag_type(&self) -> u8 {
        12
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for NbtTagLongArray {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        format_array!(self, formatter)
    }
}
