mod args;
mod chunk;
mod chunk_type;
mod commands;
mod error;
mod png;

use crate::args::{Args, Commands};
use crate::chunk_type::ChunkType;
use crate::commands::*;
use crate::png::Png;
use clap::Parser;
use std::str::FromStr;

fn main() {
    let cli = Args::parse();

    match &cli.command {
        Some(Commands::Encode {
            png,
            chunk_type,
            secret_msg,
        }) => {
            encode(png.as_ref().unwrap(), chunk_type.as_ref().unwrap(), secret_msg.as_ref().unwrap())
                .expect("TODO: panic message");

            /*if let Some(path) = png.as_deref() {
                println!("Value for png: {}", path.display());
            }
            if let Some(chunk_type) = chunk_type.as_deref() {
                println!(
                    "Value for chunk_type: {}",
                    ChunkType::from_str(chunk_type).unwrap()
                );
            }
            if let Some(secret_msg) = secret_msg.as_deref() {
                println!("Value for secret_msg: {}", secret_msg);
            }*/
        }
        None => {
            println!("No subcommand was used");
        }
    }
}
