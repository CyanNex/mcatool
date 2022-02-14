use std::fs::File;
use std::io::prelude::*;

use clap::{app_from_crate, arg};

use crate::anvil::AnvilReader;

mod anvil;
mod nbt;
mod binutil;


fn main() {
    let matches = app_from_crate!()
        .arg(arg!(<mca_file> "The .mca file to read from"))
        .arg(arg!(<x> "the chunk X coordinate").validator(|s| { s.parse::<u32>() }))
        .arg(arg!(<z> "the chunk Z coordinate").validator(|s| { s.parse::<u32>() }))
        .arg(arg!([output_file] "The file to write to"))
        .get_matches();

    let mca_file = matches.value_of("mca_file").unwrap();

    let x_coord = matches.value_of_t("x").unwrap();
    let z_coord = matches.value_of_t("z").unwrap();

    let default_output_file = format!("c.{}.{}.dat", x_coord, z_coord);
    let output_file = matches.value_of("output_file")
        .unwrap_or(default_output_file.as_str());

    export_chunk_to_file(mca_file, x_coord, z_coord, output_file);
}

fn export_chunk_to_file(input_file: &str, x_coord: u32, z_coord: u32, output_file: &str) {
    let file = File::open(input_file).unwrap();
    let reader = AnvilReader::from_file(&file).unwrap();
    let chunk_data = reader.read_chunk(x_coord, z_coord).unwrap();
    let mut new_file = File::create(output_file).unwrap();
    new_file.write_all(&chunk_data).unwrap();
}
