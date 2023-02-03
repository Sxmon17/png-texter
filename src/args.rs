use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode {
        #[arg(index = 1, value_name = "FILE")]
        // Png to encode
        png: Option<PathBuf>,

        // Chunk type to use
        #[arg(index = 2, value_name = "CHUNK_TYPE")]
        chunk_type: Option<String>,

        // Secret message to encode within the png
        #[arg(index = 3, value_name = "SECRET_MSG")]
        secret_msg: Option<String>,
    },
    Decode {
        // Png to decode
        #[arg(index = 1, value_name = "FILE")]
        png: Option<PathBuf>,

        // Chunk type to use
        #[arg(index = 2, value_name = "CHUNK_TYPE")]
        chunk_type: Option<String>,
    },
    Remove {
        // Png to use
        #[arg(index = 1, value_name = "FILE")]
        png: Option<PathBuf>,

        // Chunk type to remove
        #[arg(index = 2, value_name = "CHUNK_TYPE")]
        chunk_type: Option<String>,
    },
}
