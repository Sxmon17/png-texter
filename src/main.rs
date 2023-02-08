#![allow(warnings)]

extern crate core;

mod chunk;
mod chunk_type;
mod cli;
mod commands;
mod error;
mod png;

use crate::cli::{Cli, Commands};
use crate::commands::*;
use clap::error::{DefaultFormatter, Error, ErrorFormatter, ErrorKind};
use clap::Parser;
use reqwest::blocking::get;
use std::path::PathBuf;

fn main() {
    let cli = Cli::parse();
}
