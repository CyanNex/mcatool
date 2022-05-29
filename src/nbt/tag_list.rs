use std::any::Any;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Read;

use crate::binutil::{buf_read_u32, buf_read_u8};
use crate::format_list;
use crate::nbt::{BoxedTag, NbtTag, read_tag_data, TagHeader};

/// A list of tag payloads, without repeated tag IDs or any tag names.
pub struct NbtTagList {
    name: String,
    children: Vec<BoxedTag>,
}

impl NbtTag for NbtTagList {
    fn new(header: TagHeader, buffer: &mut dyn Read) -> Box<Self> {
        // the first byte in the list tag is the type of the list elements
        let child_type = buf_read_u8(buffer) as u8;
        // next is an integer value which is the number of elements in the list
        let child_count = buf_read_u32(buffer) as usize;

        // all elements in the list have the same header, so we can create that first
        let child_header = TagHeader {
            tag_type: child_type,
            name: String::new(),
        };

        // create a list to hold all the parsed elements of this list
        let mut children = Vec::with_capacity(child_count);

        // loop over all the children
        for _ in 0..child_count {
            // the children in a list do not have a header, so we only need to read the data
            // we can clone the header we made previously
            let child = read_tag_data(child_header.clone(), buffer).unwrap();

            children.push(child);
        }

        return Box::new(
            NbtTagList {
                name: header.name,
                children,
            }
        );
    }

    fn tag_type(&self) -> u8 {
        9
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for NbtTagList {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        format_list!(self, formatter)
    }
}
