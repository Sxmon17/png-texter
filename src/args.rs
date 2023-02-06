use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "png-texter",
    about = "A simple png chunk message encoder/decoder"
)]
#[command(author = "Simon Guglberger <simonguglberger@gmail.com>")]
#[command(version = "v1.1.0-alpha")]
#[command(author, version, about, long_about = None)]
pub struct Args {
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
