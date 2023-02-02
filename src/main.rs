mod args;
mod chunk;
mod chunk_type;
mod commands;
mod error;
mod png;

use crate::args::{Args, Commands};
use crate::commands::*;
use clap::Parser;

fn main() {
    let cli = Args::parse();

    match &cli.command {
        Some(Commands::Encode {
            png,
            chunk_type,
            secret_msg,
        }) => {
            encode(
                png.as_ref().unwrap(),
                chunk_type.as_ref().unwrap(),
                secret_msg.as_ref().unwrap(),
            )
            .expect("Error while encoding");
        }
        Some(Commands::Decode { png, chunk_type }) => {
            decode(png.as_ref().unwrap(), chunk_type.as_ref().unwrap()).unwrap();
        }
        Some(Commands::Remove { png, chunk_type }) => {
            remove(png.as_ref().unwrap(), chunk_type.as_ref().unwrap()).unwrap();
        }
        None => {
            println!("No subcommand was used");
        }
    }
}
