mod args;
mod chunk;
mod chunk_type;
mod commands;
mod error;
mod png;

use crate::args::{Args, Commands};
use crate::commands::*;
use clap::error::{DefaultFormatter, Error, ErrorFormatter, ErrorKind};
use clap::Parser;
use std::path::PathBuf;

fn main() {
    let cli = Args::parse();

    let chunk_type = Some("rust");

    let output: Option<&PathBuf> = None;

    match &cli.command {
        Some(Commands::Encode {
            png,
            secret_msg,
            output,
        }) => {
            match output {
                Some(T) => encode(
                    png.as_ref().unwrap(),
                    output.as_ref().unwrap(),
                    chunk_type.unwrap(),
                    secret_msg.as_ref().unwrap(),
                )
                .expect("Error while encoding"),
                None => encode(
                    png.as_ref().unwrap(),
                    png.as_ref().unwrap(),
                    chunk_type.unwrap(),
                    secret_msg.as_ref().unwrap(),
                )
                .expect("Error while encoding"),
            }
            println!("Encoded successfully");
        }
        Some(Commands::Decode { png }) => {
            let msg = decode(png.as_ref().unwrap(), chunk_type.unwrap());
            if msg.is_err() {
                println!("No chunk found");
                return;
            }
            println!("Decoded successfully with message: {}", msg.unwrap());
        }
        Some(Commands::Remove { png }) => {
            remove(png.as_ref().unwrap(), chunk_type.unwrap()).unwrap();
        }
        Some(Commands::Example { png }) => {
            encode(
                "png_tests/good_normal_tiny-rgb-gray.png",
                png.as_ref().unwrap(),
                chunk_type.unwrap(),
                "Hello World!",
            )
            .expect("Error while encoding");
            println!("Encoded successfully")
        }
        None => {
            println!("No subcommand was used");
        }
    }
}
