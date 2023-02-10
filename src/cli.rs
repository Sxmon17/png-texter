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
}
