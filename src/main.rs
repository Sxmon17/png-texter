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

    let chunk_type = Some("rust");

    match &cli.command {
        Some(Commands::Encode {
            png,
            secret_msg,
        }) => {
            encode(
                png.as_ref().unwrap(),
                 chunk_type.unwrap(),
                secret_msg.as_ref().unwrap(),
            )
            .expect("Error while encoding");
            println!("Encoded successfully")
        }
        Some(Commands::Decode { png }) => {
            let msg = decode(png.as_ref().unwrap(), chunk_type.unwrap()).unwrap();
            println!("Decoded successfully with message: {}", msg);
        }
        Some(Commands::Remove { png }) => {
            remove(png.as_ref().unwrap(), chunk_type.unwrap()).unwrap();
        }
        None => {
            println!("No subcommand was used");
        }
    }
}
