use std::io;
use crate::error::Error::IoError;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    AnvilParseError(&'static str),
    AnvilWriteError(&'static str),
    ChunkNotFoundError
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        IoError(err)
    }
}
