/// Module for parsing NBT data structures.
/// https://minecraft.fandom.com/wiki/NBT_format#NBT_file

use std::{fmt, io};
use std::any::Any;
use std::io::{Error, Read};
use std::io::ErrorKind::Other;

use crate::binutil::{buf_read_u16, buf_read_u8, parse_string};
use crate::nbt::tag_array::{NbtTagByteArray, NbtTagIntArray, NbtTagLongArray};
use crate::nbt::tag_compound::{NbtTagCompound, NbtTagEnd};
use crate::nbt::tag_list::NbtTagList;
use crate::nbt::tag_number::{NbtTagByte, NbtTagDouble, NbtTagFloat, NbtTagInt, NbtTagLong, NbtTagShort};
use crate::nbt::tag_string::NbtTagString;

pub mod tag_compound;
pub mod tag_number;
pub mod tag_string;
pub mod tag_array;
pub mod tag_list;
mod format;

type BoxedTag = Box<dyn NbtTag>;


/// Generic trait for all NBT tags
pub trait NbtTag: fmt::Display {
    /// Read this tag from a buffer into a struct
    fn new(header: TagHeader, buffer: &mut dyn Read) -> Box<Self> where Self: Sized;
    /// Returns the type byte (0-12) of this tag
    fn tag_type(&self) -> u8;
    /// Returns the name of this tag. May be empty
    fn name(&self) -> &str;

    fn as_any(&self) -> &dyn Any;
}

/// The header of any tag in the binary format. Contains a
/// type and a name. The name may be empty.
#[derive(Clone)]
pub struct TagHeader {
    tag_type: u8,
    name: String,
}

/// Read an NBT tag from a binary buffer.
/// If the tag has children, they will be parsed recursively.
/// <p>
/// The cursor of the buffer will be moved to the end of this tag.
pub fn read_tag(buffer: &mut dyn Read) -> Result<BoxedTag, io::Error> {
    let header = read_header(buffer);
    return read_tag_data(header, buffer);
}

/// Read a tag header from a binary buffer.
/// Since TAG_End never has a name, it's name field will always be empty.
/// <p>
/// The cursor of the buffer will be moved to the end of the header, which
/// is the start of the data of the tag.
fn read_header(buffer: &mut dyn Read) -> TagHeader {
    let tag_type = buf_read_u8(buffer);

    let name = match tag_type {
        // tag type 0 (TAG_End) does not have a name field
        0 => String::new(),
        // for all other tag types, read the name field
        _ => {
            // name length is a 16-bit value
            let name_len = buf_read_u16(buffer) as usize;

            // read the name from the buffer
            let mut raw_name = vec![0; name_len];
            buffer.read_exact(&mut raw_name).unwrap();
            parse_string(raw_name.as_slice())
        }
    };

    return TagHeader {
        tag_type,
        name,
    };
}

/// Read the data of an NBT tag from a binary buffer.
/// <p>
/// The cursor of the buffer will be moved to the end of the tag.
fn read_tag_data(header: TagHeader, buffer: &mut dyn Read) -> Result<BoxedTag, io::Error> {
    // read the correct tag based on the type in the header
    return match header.tag_type {
        0 => Ok(NbtTagEnd::new(header, buffer)),
        1 => Ok(NbtTagByte::new(header, buffer)),
        2 => Ok(NbtTagShort::new(header, buffer)),
        3 => Ok(NbtTagInt::new(header, buffer)),
        4 => Ok(NbtTagLong::new(header, buffer)),
        5 => Ok(NbtTagFloat::new(header, buffer)),
        6 => Ok(NbtTagDouble::new(header, buffer)),
        7 => Ok(NbtTagByteArray::new(header, buffer)),
        8 => Ok(NbtTagString::new(header, buffer)),
        9 => Ok(NbtTagList::new(header, buffer)),
        10 => Ok(NbtTagCompound::new(header, buffer)),
        11 => Ok(NbtTagIntArray::new(header, buffer)),
        12 => Ok(NbtTagLongArray::new(header, buffer)),

        _ => Err(Error::new(Other, format!("Unknown tag type {}", header.tag_type)))
    };
}
