use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Read;

use crate::format_compound;
use crate::nbt::{BoxedTag, NbtTag, read_tag, TagHeader};

/// A list of fully formed tags, including their IDs, names, and payloads.
/// No two tags may have the same name.
pub struct NbtTagCompound {
    name: String,
    children: Vec<BoxedTag>,
}

impl NbtTag for NbtTagCompound {
    fn new(header: TagHeader, buffer: &mut dyn Read) -> Box<Self> {
        // create a list to hold all the parsed children of this compound
        let mut children = Vec::new();

        // we loop until we see a TAG_End
        loop {
            // read a single tag
            let child = read_tag(buffer).unwrap();
            // if the type is not 0, add it to the children
            // if the type is 0, it is a TAG_End marking the end of this compound
            let child_type = child.tag_type();
            if child_type != 0 {
                children.push(child);
            } else {
                break;
            }
        }

        return Box::new(
            NbtTagCompound {
                name: header.name,
                children,
            }
        );
    }

    fn tag_type(&self) -> u8 {
        10
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Display for NbtTagCompound {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        format_compound!(self, formatter)
    }
}

/// The NbtTagEnd is used to mark the end of an NbtTagCompound.
/// Note that the NbtTagEnd is only present in the binary data, and
/// the parser will not add any to the final parsed tree.
pub struct NbtTagEnd {}

impl NbtTag for NbtTagEnd {
    fn new(_header: TagHeader, _buffer: &mut dyn Read) -> Box<Self> {
        Box::new(NbtTagEnd {})
    }

    fn tag_type(&self) -> u8 {
        0
    }

    fn name(&self) -> &str {
        ""
    }
}

impl Display for NbtTagEnd {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "")
    }
}
