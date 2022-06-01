use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Write};
use std::io::ErrorKind::UnexpectedEof;

use flate2::read::ZlibDecoder;

use crate::binutil::{buf_write_u24, buf_write_u32, buf_write_u8, parse_u24, parse_u32};
use crate::error::Error;
use crate::error::Error::{AnvilParseError, AnvilWriteError, ChunkNotFoundError};

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
pub struct AnvilData {
    pub data_buffer: Blob,
}

#[derive(Debug)]
struct ChunkHeader {
    data_offset: u32,
    sector_count: u8,
}

impl AnvilData {
    pub fn new() -> AnvilData {
        return Self {
            data_buffer: vec![0; 8 * 1024],
        };
    }

    /// Create a new AnvilReader and read the data from a given file.
    /// This function will attempt to read all the bytes from the file.
    /// The given file must be at least 8192 bytes long.
    pub fn from_file(file: &File) -> Result<AnvilData, Error> {
        // read the file length from the file metadata
        let file_length = file.metadata()?.len() as usize;

        let mut file_reader = BufReader::new(file);
        // create a buffer for storing the file contents
        let mut data_buffer = Vec::with_capacity(file_length);
        // read the file into the buffer
        let read_length = file_reader.read_to_end(&mut data_buffer)?;

        // if the number of bytes read is not the same as the file
        // length, return an error
        if read_length != file_length {
            return Err(AnvilParseError("File length does not match read length"));
        }

        return Self::from_blob(data_buffer);
    }

    /// Create a new AnvilReader with the provided binary Blob.
    /// The provided Blob must be at least 8192 bytes long.
    pub fn from_blob(blob: Blob) -> Result<AnvilData, Error> {
        // if the length of the blob is less than 8kb, return an error
        if blob.len() < 8192 {
            return Err(AnvilParseError("Anvil data must be at least 8192 bytes"));
        }

        return Ok(AnvilData { data_buffer: blob });
    }

    /// Read a chunk from the Anvil data.
    /// This function will not check if the chunk coordinates are within the bounds
    /// of this region, instead the value will be wrapped to always be inside the bounds.
    /// It's recommended to first validate that the given coordinates are within the
    /// bounds of the region.
    /// This function will return a decompressed binary Blob in NBT format.
    pub fn read_chunk(&self, chunk_x: u32, chunk_z: u32) -> Result<Blob, Error> {
        // calculate the chunk index from the coordinates
        let chunk_idx = (chunk_x & 31) | ((chunk_z & 31) << 5);

        let header = &self.read_header(chunk_idx);

        if header.data_offset > 0 || header.sector_count > 0 {
            // get the offset and length of the chunk data from the header
            let offset = (header.data_offset << 12) as usize;
            let length = parse_u32(&self.data_buffer[offset..offset + 4]) as usize;

            // get the raw chunk data from the buffer
            let chunk_data_start = offset + 5;
            let mut chunk_data_end = chunk_data_start + length;
            if chunk_data_start >= self.data_buffer.len() {
                panic!("Chunk at {}, {} should start at {} but region is only {} bytes",
                       chunk_x, chunk_z, chunk_data_start, self.data_buffer.len());
            }
            if chunk_data_end >= self.data_buffer.len() {
                // panic!("Chunk at {}, {} should end at {} but region is only {} bytes",
                //        chunk_x, chunk_z, chunk_data_end, self.data_buffer.len());
                chunk_data_end -= 1;
            }
            let raw_chunk_data = &self.data_buffer[chunk_data_start..chunk_data_end];

            // decompressed (decompress) the chunk data
            // let chunk_data = zlib_decompress(raw_chunk_data)?;

            return Ok(raw_chunk_data.to_vec());
        } else {
            return Err(ChunkNotFoundError);
        }
    }

    pub fn write_chunk(&mut self, chunk_x: u32, chunk_z: u32, data: &Blob, uncompressed_size: u32) -> Result<(), Error> {
        // let compressed_data = zlib_compress(data)?;
        // let data_len = compressed_data.len() + 5;
        let data_len = data.len() as u32;

        // chunk size is limited to 256 sectors of 4KiB
        let sector_count = uncompressed_size >> 12;
        if sector_count > 256 {
            return Err(AnvilWriteError("Chunk cannot be larger than 4096*256"));
        }

        // calculate the chunk index from the coordinates
        let chunk_idx = (chunk_x & 31) | ((chunk_z & 31) << 5);

        let header = ChunkHeader {
            sector_count: sector_count as u8,
            data_offset: (self.data_buffer.len() as u32) >> 12,
        };
        self.write_header(chunk_idx, &header);

        // the chunk data is prefixed with a small header containing
        // the compressed size and compression type
        let mut data_head = vec![0; 5];
        buf_write_u32(&mut data_head, data_len);
        data_head[4] = 2; // zlib compression type

        self.data_buffer.write_all(&data_head)?;
        self.data_buffer.write_all(data.as_slice())?;

        let total_sectors = (self.data_buffer.len() >> 12) + 1;
        self.data_buffer.resize(total_sectors << 12, 0);

        return Ok(());
    }

    fn read_header(&self, chunk_idx: u32) -> ChunkHeader {
        // calculate the offset of the header (chunk_idx * 4)
        let offset = (chunk_idx << 2) as usize;
        // get the raw chunk header (4 bytes) from the buffer
        let raw_chunk_header = &self.data_buffer[offset..offset + 4];

        return ChunkHeader {
            data_offset: parse_u24(raw_chunk_header),
            sector_count: raw_chunk_header[3],
        };
    }

    fn write_header(&mut self, chunk_idx: u32, header: &ChunkHeader) {
        // calculate the offset of the header (chunk_idx * 4)
        let offset = (chunk_idx << 2) as usize;

        buf_write_u24(&mut self.data_buffer[offset..offset + 3], header.data_offset);
        buf_write_u8(&mut self.data_buffer[offset + 3..offset + 4], header.sector_count);

        let offset = offset | 0x1000;
        buf_write_u32(&mut self.data_buffer[offset..offset + 4], 1653847548);
    }
}

pub fn zlib_decompress(blob: &[u8]) -> Result<Blob, Error> {
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
        Err(io::Error::new(UnexpectedEof, "Zlib returned 0 bytes").into())
    };
}
