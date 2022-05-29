extern crate core;

use std::fs;
use std::fs::File;
use std::time::Instant;
// use std::io::prelude::*;

use clap::{app_from_crate, arg};

use crate::anvil::AnvilData;
use crate::nbt::{NbtTag, read_tag};
use crate::nbt::tag_compound::NbtTagCompound;
use crate::nbt::tag_number::NbtTagLong;
use crate::trim::{trim_world, TrimOptions};

mod anvil;
mod nbt;
mod binutil;
mod trim;
mod error;


fn main() {
    // let matches = app_from_crate!()
    //     .arg(arg!(<mca_file> "The .mca file to read from"))
    //     .arg(arg!(<x> "the chunk X coordinate").validator(|s| { s.parse::<u32>() }))
    //     .arg(arg!(<z> "the chunk Z coordinate").validator(|s| { s.parse::<u32>() }))
    //     .arg(arg!([output_file] "The file to write to"))
    //     .get_matches();
    //
    // let mca_file = matches.value_of("mca_file").unwrap();
    //
    // let file = File::open(&mca_file).unwrap();
    // let reader = AnvilReader::from_file(&file).unwrap();
    // for x in 10..32 {
    //     for z in 0..32 {
    //         let chunk_data = reader.read_chunk(x, z).unwrap();
    //         if chunk_data.len() > 0 {
    //             let tag = read_tag(&mut chunk_data.as_slice()).unwrap();
    //             let tag = match tag.as_any().downcast_ref::<NbtTagCompound>() {
    //                 Some(b) => b,
    //                 None => panic!("tag isn't a compound!"),
    //             };
    //
    //             for child in &tag.children {
    //                 if child.name() == "InhabitedTime" {
    //                     let child = match child.as_any().downcast_ref::<NbtTagLong>() {
    //                         Some(b) => b,
    //                         None => panic!("child isn't a long!"),
    //                     };
    //                     if child.value > 0 {
    //                         println!("{}: {}", child.name(), child.value);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    trim_world("test", TrimOptions::new(6000, 24)).unwrap();

    // let mut anvil = AnvilReader::new();
    // let mut test = Vec::new();
    // test.resize(100 * 1024, 0xAB);
    // anvil.write_chunk(2, 5, &test).unwrap();
    // let test = vec![1, 2, 3, 4, 5, 6];
    // anvil.write_chunk(30, 8, &test).unwrap();
    // let chunk = anvil.read_chunk(2, 5).unwrap();
    //
    // let now = Instant::now();
    // for _ in 0..(32 * 32) {
    //     anvil.write_chunk(2, 5, &test).unwrap();
    // }
    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);

    // let x_coord = matches.value_of_t("x").unwrap();
    // let z_coord = matches.value_of_t("z").unwrap();
    //
    // let default_output_file = format!("c.{}.{}.dat", x_coord, z_coord);
    // let output_file = matches.value_of("output_file")
    //     .unwrap_or(default_output_file.as_str());
    //
    // export_chunk_to_file(mca_file, x_coord, z_coord, output_file);
}

// fn export_chunk_to_file(input_file: &str, x_coord: u32, z_coord: u32, output_file: &str) {
//     let file = File::open(input_file).unwrap();
//     let reader = AnvilReader::from_file(&file).unwrap();
//     let chunk_data = reader.read_chunk(x_coord, z_coord).unwrap();
//     let mut new_file = File::create(output_file).unwrap();
//     new_file.write_all(&chunk_data).unwrap();
// }
