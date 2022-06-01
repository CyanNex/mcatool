use std::fs::File;
use std::io::prelude::*;

use clap::{arg, command, Command};

use crate::anvil::AnvilData;
use crate::trim::TrimOptions;

mod anvil;
mod trim;
mod binutil;
mod error;


fn main() {
    let matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("extract")
            .about("Extract a single chunk from a .mca file")
            .arg(arg!(<mca_file> "The .mca file to read from"))
            .arg(arg!(<x> "the chunk X coordinate").validator(|s| { s.parse::<u32>() }))
            .arg(arg!(<z> "the chunk Z coordinate").validator(|s| { s.parse::<u32>() }))
            .arg(arg!([output_file] "The file to write to"))
        )
        .subcommand(Command::new("trim")
            .about("Trim all inactive chunks from a world")
            .arg(arg!(<world> "Directory of the world to trim"))
            .arg(arg!(<min_inhabited_time> "The minimum inhabited time for a chunk to be kept (in ticks)"))
            .arg(arg!([threads] "The number of trimming threads (defaults to number of cores)"))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("extract", matches)) => {
            let mca_file = matches.value_of("mca_file").unwrap();

            let x_coord = matches.value_of_t("x").unwrap();
            let z_coord = matches.value_of_t("z").unwrap();

            let default_output_file = format!("c.{}.{}.dat", x_coord, z_coord);
            let output_file = matches.value_of("output_file")
                .unwrap_or(default_output_file.as_str());

            export_chunk_to_file(mca_file, x_coord, z_coord, output_file);
        }
        Some(("trim", matches)) => {
            let world_dir = matches.value_of("world").unwrap();
            let min_inhabited_time = matches.value_of_t("min_inhabited_time").unwrap();
            let threads = matches.value_of_t("threads")
                .unwrap_or(num_cpus::get());

            trim::trim_world(world_dir, TrimOptions::new(
                min_inhabited_time, threads,
            )).unwrap();
        }
        _ => { panic!("Unknown subcommand"); }
    }
}

fn export_chunk_to_file(input_file: &str, x_coord: u32, z_coord: u32, output_file: &str) {
    let file = File::open(input_file).unwrap();
    let reader = AnvilData::from_file(&file).unwrap();
    let chunk_data = reader.read_chunk(x_coord, z_coord).unwrap();
    let mut new_file = File::create(output_file).unwrap();
    new_file.write_all(&chunk_data).unwrap();
}
