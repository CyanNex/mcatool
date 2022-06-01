use std::io;
use crate::error::Error::{FsError, IoError};

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    FsError(fs_extra::error::Error),
    AnvilParseError(&'static str),
    ChunkReadError(String),
    AnvilWriteError(&'static str),
    ChunkNotFoundError,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        IoError(err)
    }
}

impl From<fs_extra::error::Error> for Error {
    fn from(err: fs_extra::error::Error) -> Self {
        FsError(err)
    }
}
