use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub name: Option<String>,

    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode {
        #[arg(index = 1, value_name = "FILE")]
        png: Option<PathBuf>,

        #[arg(index = 2, value_name = "CHUNK_TYPE")]
        chunk_type: Option<String>,

        #[arg(index = 3, value_name = "SECRET_MSG")]
        secret_msg: Option<String>,
    },
    Decode {
        #[arg(index = 1, value_name = "FILE")]
        png: Option<PathBuf>,

        #[arg(index = 2, value_name = "CHUNK_TYPE")]
        chunk_type: Option<String>,
    },
    Remove {
        #[arg(index = 1, value_name = "FILE")]
        png: Option<PathBuf>,

        #[arg(index = 2, value_name = "CHUNK_TYPE")]
        chunk_type: Option<String>,
    },
}
