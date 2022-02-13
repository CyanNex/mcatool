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
        // read the file length from the file metadata
        let file_length = file.metadata()?.len() as usize;
        // create a new reader to read the file
        let mut file_reader = BufReader::new(file);

        // create a buffer for storing the file contents
        let mut data_buffer = Vec::with_capacity(file_length);
        // read the file into the buffer
        let read_length = file_reader.read_to_end(&mut data_buffer)?;

        // if the number of bytes read is not the same as the file
        // length, return an error
        if read_length != file_length {
            return Err(Error::new(Other, "File length does not match read length"));
        }

        // return a new AnvilReader created from the buffer
        return Self::from_blob(data_buffer);
    }

    /// Create a new AnvilReader with the provided binary Blob.
    /// The provided Blob must be at least 8192 bytes long.
    pub fn from_blob(blob: Blob) -> Result<AnvilReader, io::Error> {
        // if the length of the blob is less than 8kb, return an error
        if blob.len() < 8192 {
            return Err(Error::new(Other, "Anvil data must be at least 8192 bytes"));
        }

        // return a new AnvilReader created from the blob
        return Ok(AnvilReader { data_buffer: blob });
    }

    /// Read a chunk from the Anvil data.
    /// This function will not check if the chunk coordinates are within the bounds
    /// of this region, instead the value will be wrapped to always be inside the bounds.
    /// It's recommended to first validate that the given coordinates are within the
    /// bounds of the region.
    /// This function will return a deflated binary Blob in NBT format.
    pub fn read_chunk(&self, chunk_x: u32, chunk_z: u32) -> Result<Blob, io::Error> {
        // calculate the chunk index from the coordinates
        let chunk_idx = (chunk_x & 31) | ((chunk_z & 31) << 5);
        // read the chunk header at this index
        let header = &self.read_header(chunk_idx);

        // get the offset and length of the chunk data from the header
        let offset = (header.data_offset * 4096) as usize;
        let length = read_u32(&self.data_buffer[offset..offset + 4]) as usize;

        // get the raw chunk data from the buffer
        let chunk_data_start = offset + 5;
        let chunk_data_end = chunk_data_start + length;
        let raw_chunk_data = &self.data_buffer[chunk_data_start..chunk_data_end];

        // deflate (decompress) the chunk data
        let chunk_data = zlib_deflate(raw_chunk_data)?;

        // return the chunk data
        return Ok(chunk_data);
    }

    fn read_header(&self, chunk_idx: u32) -> ChunkHeader {
        // calculate the offset of the header (chunk_idx * 4)
        let offset = (chunk_idx << 2) as usize;
        // get the raw chunk header (4 bytes) from the buffer
        let raw_chunk_header = &self.data_buffer[offset..offset + 4];

        // return the chunk header
        return ChunkHeader {
            data_offset: read_u24(raw_chunk_header)
        };
    }
}

fn zlib_deflate(blob: &[u8]) -> Result<Blob, io::Error> {
    // create a new zlib decoder for the blob
    let mut zlib_decoder = ZlibDecoder::new(blob);
    // create a buffer for storing the decoded result
    let mut result_buffer = Vec::new();

    // let the decoder decode all the data
    let length = zlib_decoder.read_to_end(&mut result_buffer)?;

    // if the decoded returned more than 0 bytes, return the decoded data,
    // otherwise, return an error
    return if length > 0 {
        Ok(result_buffer)
    } else {
        Err(Error::new(UnexpectedEof, "Zlib returned 0 bytes"))
    };
}

/// read a 24-bit big-endian value from a buffer
fn read_u24(array: &[u8]) -> u32 {
    return ((array[0] as u32) << 16)
        | ((array[1] as u32) << 8)
        | ((array[2] as u32) << 0);
}

/// read a 32-bit big-endian value from a buffer
fn read_u32(array: &[u8]) -> u32 {
    return ((array[0] as u32) << 24)
        | ((array[1] as u32) << 16)
        | ((array[2] as u32) << 8)
        | ((array[3] as u32) << 0);
}
