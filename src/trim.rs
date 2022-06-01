use std::{fs, thread};
use std::fs::{DirEntry, File, ReadDir};
use std::io::Write;
use std::path::{Path, PathBuf};
use fs_extra::dir;

use serde::Deserialize;

use crate::{anvil, AnvilData};
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

    println!("Processing '{}' with {} threads", world_dir, options.thread_count);

    let now = std::time::Instant::now();
    let original_size = dir::get_size(world_dir)?;

    for region_dir in region_dirs {
        println!("Processing '{}'", region_dir);
        let region_dir = Path::new(&region_dir);
        if region_dir.exists() {
            trim_region_dir(&region_dir, options)?;
        }
    }

    let trimmed_size = dir::get_size(world_dir)?;
    let trim_percent = 100.0 - ((trimmed_size as f64) / (original_size as f64) * 100.0);

    println!("Trimmed from {:.2} GB to {:.2} GB ({:.0}%), took {:.2?}!",
             bytes_to_gb(original_size), bytes_to_gb(trimmed_size),
             trim_percent, now.elapsed());

    return Ok(());
}

fn trim_region_dir(region_dir: &Path, options: TrimOptions) -> Result<(), Error> {
    let region_dir = fs::read_dir(region_dir)?;
    let region_files = find_region_files(region_dir)?;
    let mut grouped_region_files = vec![Vec::new(); options.thread_count];
    group_region_files(&region_files, &mut grouped_region_files);

    let mut threads = Vec::new();
    for _ in 0..options.thread_count {
        let group_files = grouped_region_files.remove(0);
        threads.push(thread::spawn(move || {
            for path in group_files {
                trim_region_file(path, options).unwrap();
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

fn trim_region_file(file_path: PathBuf, options: TrimOptions) -> Result<(), Error> {
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
                    writer.write_chunk(x, z, &chunk_data, decompressed_data.len() as u32)?;
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

fn find_region_files(region_dir: ReadDir) -> Result<Vec<DirEntry>, Error> {
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
    let section: ChunkData = fastnbt::from_bytes(chunk_data).unwrap();
    return if let Some(inhabited_time) = section.inhabited_time {
        Ok(inhabited_time)
    } else if let Some(level) = section.level {
        Ok(level.inhabited_time)
    } else {
        Err(AnvilParseError("Unable to find chunk inhabited time"))
    };
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ChunkData {
    inhabited_time: Option<i64>,
    level: Option<Level>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Level {
    inhabited_time: i64,
}

fn bytes_to_gb(bytes: u64) -> f64 {
    (bytes as f64) / 1000.0 / 1000.0 / 1000.0
}
