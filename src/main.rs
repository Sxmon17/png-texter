#![allow(dead_code)]

extern crate core;

mod chunk;
mod chunk_type;
mod cli;
mod commands;
mod error;
mod png;

use crate::cli::{Cli, Commands};
use crate::commands::*;
use ansi_term::Colour;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    let chunk_type = Some("rust");

    match &cli.command {
        Some(Commands::Encode {
            png,
            secret_msg,
            output,
            url,
        }) => {
            let mut output_path = png;
            if url.as_ref().is_some() {
                encode_from_url(
                    url.as_ref().unwrap(),
                    output_path.as_ref().unwrap(),
                    chunk_type.unwrap(),
                    secret_msg.as_ref().unwrap(),
                )
                .expect(
                    Colour::Red
                        .paint("Error while encoding")
                        .to_string()
                        .as_str(),
                );
            } else {
                output_path = if output.is_none() { png } else { output };
                encode(
                    png.as_ref().unwrap(),
                    output_path.as_ref().unwrap(),
                    chunk_type.unwrap(),
                    secret_msg.as_ref().unwrap(),
                )
                .expect(
                    Colour::Red
                        .paint("Error while encoding")
                        .to_string()
                        .as_str(),
                );
            };
            println!("{}", Colour::Green.paint("Encoded successfully"));
        }
        Some(Commands::Decode { png }) => {
            let msg = decode(png.as_ref().unwrap(), chunk_type.unwrap());
            if msg.is_err() {
                println!("{} No secret message found", Colour::Red.paint("Error:"));
                return;
            }
            println!(
                "{} {}",
                Colour::Green.paint("Decoded successfully with message:\n"),
                msg.unwrap()
            );
        }
        Some(Commands::Remove { png }) => {
            remove(png.as_ref().unwrap(), chunk_type.unwrap()).unwrap();
        }
        None => {
            println!("No subcommand was used");
        }
    }
}
