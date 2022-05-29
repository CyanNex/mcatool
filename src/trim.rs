use std::{fs, thread};
use std::fs::{DirEntry, File, ReadDir};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;

use crate::{anvil, AnvilData, nbt, NbtTagCompound, NbtTagLong};
use crate::anvil::Blob;
use crate::error::Error;
use crate::error::Error::AnvilParseError;

#[derive(Copy, Clone)]
pub struct TrimOptions {
    min_inhabited_time: i64,
    thread_count: usize,
}

impl TrimOptions {
    pub fn new(min_inhabited_time: i64, thread_count: usize) -> Self {
        Self {
            min_inhabited_time,
            thread_count,
        }
    }
}

pub fn trim_world(world_dir: &str, options: TrimOptions) -> Result<(), Error> {
    let region_dirs = vec![
        format!("{}/region", world_dir),
        format!("{}/DIM1/region", world_dir),
        format!("{}/DIM-1/region", world_dir),
    ];

    let now = Instant::now();

    for region_dir in region_dirs {
        println!("Processing '{}'", region_dir);
        let region_dir = Path::new(&region_dir);
        if region_dir.exists() {
            trim_region_dir(&region_dir, options)?;
        }
    }

    println!("Done (took {:.2?})!", now.elapsed());

    return Ok(());
}

fn trim_region_dir(region_dir: &Path, options: TrimOptions) -> Result<(), Error> {
    let region_dir = fs::read_dir(region_dir)?;
    let region_files = get_region_files(region_dir)?;
    let mut grouped_region_files = vec![Vec::new(); options.thread_count];
    group_region_files(&region_files, &mut grouped_region_files);

    let mut threads = Vec::new();
    for _ in 0..options.thread_count {
        let group_files = grouped_region_files.remove(0);
        threads.push(thread::spawn(move || {
            for path in group_files {
                process_region_file(path, options).unwrap();
            }
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    return Ok(());
}

fn group_region_files(region_files: &Vec<DirEntry>,
                      grouped_region_files: &mut Vec<Vec<PathBuf>>) {
    let group_count = grouped_region_files.len();

    let mut region_file_idx = 0;
    while region_file_idx < region_files.len() {
        for group in 0..group_count {
            let region_file = region_files[region_file_idx].path();
            grouped_region_files[group].push(region_file);
            region_file_idx += 1;
            if region_file_idx >= region_files.len() {
                break;
            }
        }
    }
}

fn process_region_file(file_path: PathBuf, options: TrimOptions) -> Result<(), Error> {
    let file = File::open(&file_path)?;
    let reader = AnvilData::from_file(&file)?;
    let mut writer = AnvilData::new();

    let mut chunk_count = 0;
    for x in 0..32 {
        for z in 0..32 {
            let chunk_data = reader.read_chunk(x, z);
            if let Ok(chunk_data) = chunk_data {
                let decompressed_data = anvil::zlib_decompress(&chunk_data)?;
                let time = get_inhabited_time(&decompressed_data)?;
                if time > options.min_inhabited_time {
                    writer.write_chunk(x, z, &chunk_data)?;
                    chunk_count += 1;
                }
            }
        }
    }

    if chunk_count > 0 {
        let mut file = File::create(&file_path)?;
        file.write_all(&writer.data_buffer)?;
    } else {
        fs::remove_file(&file_path)?;
    }

    return Ok(());
}

fn get_region_files(region_dir: ReadDir) -> Result<Vec<DirEntry>, Error> {
    let mut region_files = Vec::new();
    for file in region_dir {
        let file = file?;
        let file_name = file.file_name().into_string().unwrap();
        if file_name.ends_with(".mca") {
            region_files.push(file);
        }
    }
    return Ok(region_files);
}

fn get_inhabited_time(chunk_data: &Blob) -> Result<i64, Error> {
    let tag = nbt::read_tag(&mut chunk_data.as_slice()).unwrap();
    let tag = match tag.as_any().downcast_ref::<NbtTagCompound>() {
        Some(tag) => tag,
        None => panic!("Chunk root tag must be a NbtTagCompound"),
    };

    let tag = if tag.children.len() > 2 {
        tag
    } else {
        let tag = if tag.children[0].name() == "Level" {
            &tag.children[0]
        } else {
            &tag.children[1]
        };

        match tag.as_any().downcast_ref::<NbtTagCompound>() {
            Some(tag) => tag,
            None => panic!("Level tag must be a NbtTagCompound"),
        }
    };

    for child in &tag.children {
        if child.name() == "InhabitedTime" {
            let child = match child.as_any().downcast_ref::<NbtTagLong>() {
                Some(b) => b,
                None => panic!("InhabitedTime field must be a NbtTagLong"),
            };
            return Ok(child.value);
        }
    }
    return Err(AnvilParseError("Cannot parse chunk data"));
}
