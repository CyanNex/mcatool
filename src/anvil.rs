use std::fs::File;
use std::io;
use std::io::{BufReader, Error, Read};
use std::io::ErrorKind::{Other, UnexpectedEof};

use flate2::read::ZlibDecoder;

/// A Blob is a simple type for a Vector containing bytes
pub type Blob = Vec<u8>;

/// A struct allowing for the parsing and reading of data using
/// the [Anvil file format](https://minecraft.fandom.com/wiki/Anvil_file_format).
///
/// # Example usage
///
/// ```
/// fn main() {
///     let file = File::open("r.0.0.mca").unwrap();
///     let reader = AnvilReader::from_file(&file).unwrap();
///     let chunk_data = reader.read_chunk(4, 7).unwrap();
/// }
/// ```
pub struct AnvilReader {
    data_buffer: Blob,
}

struct ChunkHeader {
    data_offset: u32,
}

impl AnvilReader {
    /// Create a new AnvilReader and read the data from a given file.
    /// This function will attempt to read all the bytes from the file.
    /// The given file must be at least 8192 bytes long.
    pub fn from_file(file: &File) -> Result<AnvilReader, io::Error> {
        let file_length = file.metadata()?.len() as usize;
        let mut file_reader = BufReader::new(file);

        let mut data_buffer = Vec::with_capacity(file_length);
        let read_length = file_reader.read_to_end(&mut data_buffer)?;

        if read_length != file_length {
            return Err(Error::new(Other, "File length does not match read length"));
        }

        return Self::from_blob(data_buffer);
    }

    /// Create a new AnvilReader with the provided binary Blob.
    /// The provided Blob must be at least 8192 bytes long.
    pub fn from_blob(blob: Blob) -> Result<AnvilReader, io::Error> {
        if blob.len() < 8192 {
            return Err(Error::new(Other, "Anvil data must be at least 8192 bytes"));
        }

        return Ok(AnvilReader { data_buffer: blob });
    }

    /// Read a chunk from the Anvil data.
    /// This function will not check if the chunk coordinates are within the bounds
    /// of this region, instead the value will be wrapped to always be inside the bounds.
    /// It's recommended to first validate that the given coordinates are within the
    /// bounds of the region.
    /// This function will return a deflated binary Blob in NBT format.
    pub fn read_chunk(&self, chunk_x: u32, chunk_z: u32) -> Result<Blob, io::Error> {
        let chunk_idx = (chunk_x & 31) | ((chunk_z & 31) << 5);
        let header = &self.read_header(chunk_idx);

        let offset = (header.data_offset * 4096) as usize;
        let length = read_u32(&self.data_buffer[offset..offset + 4]) as usize;

        let chunk_data_start = offset + 5;
        let chunk_data_end = chunk_data_start + length;
        let raw_chunk_data = &self.data_buffer[chunk_data_start..chunk_data_end];

        let chunk_data = zlib_deflate(raw_chunk_data)?;

        return Ok(chunk_data);
    }

    fn read_header(&self, chunk_idx: u32) -> ChunkHeader {
        let offset = (chunk_idx << 2) as usize;
        let raw_chunk_header = &self.data_buffer[offset..offset + 4];

        return ChunkHeader {
            data_offset: read_u24(raw_chunk_header)
        };
    }
}

fn zlib_deflate(blob: &[u8]) -> Result<Blob, io::Error> {
    let mut zlib_decoder = ZlibDecoder::new(blob);
    let mut result_buffer = Vec::new();

    let length = zlib_decoder.read_to_end(&mut result_buffer)?;
    if length > 0 {
        return Ok(result_buffer);
    }

    return Err(Error::new(UnexpectedEof, "Zlib returned 0 bytes"));
}

fn read_u24(array: &[u8]) -> u32 {
    return ((array[0] as u32) << 16)
        | ((array[1] as u32) << 8)
        | ((array[2] as u32) << 0);
}

fn read_u32(array: &[u8]) -> u32 {
    return ((array[0] as u32) << 24)
        | ((array[1] as u32) << 16)
        | ((array[2] as u32) << 8)
        | ((array[3] as u32) << 0);
}