use std::path::PathBuf;

use crate::commands::{decode, encode, remove};
use clap::{Parser, Subcommand};

use clap::error::{DefaultFormatter, Error, ErrorFormatter, ErrorKind};

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Sets the chunk type to use
    #[arg(short, long, value_name = "CHUNK_TYPE")]
    pub chunk_type: Option<String>,

    #[arg(short, long, value_name = "URL")]
    pub url: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    /// encode the png with a secret message
    Encode {
        #[arg(index = 1, value_name = "FILE")]
        /// Png to encode
        png: Option<PathBuf>,

        /// Secret message to encode within the png
        #[arg(index = 2, value_name = "SECRET_MSG")]
        secret_msg: Option<String>,

        /// internet url as png source
        #[arg(short, long, value_name = "URL")]
        url: Option<String>,

        /// Output png
        #[arg(short, long, value_name = "output")]
        output: Option<PathBuf>,
    },

    #[command(arg_required_else_help = true)]
    /// decode the png to get the secret message
    Decode {
        /// Png to decode
        #[arg(index = 1, value_name = "FILE")]
        png: Option<PathBuf>,
    },

    /// remove the chunk from the png
    Remove {
        /// Png to use
        #[arg(index = 1, value_name = "FILE")]
        png: Option<PathBuf>,
    },

    /// create an encrypted example png
    Example {
        /// output png
        #[arg(index = 1, value_name = "FILE")]
        png: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    let chunk_type = Some("rust");

    let output: Option<&PathBuf> = None;

    let url: Option<String> = None;

    match &cli.command {
        Some(Commands::Encode {
            png,
            secret_msg,
            url,
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
