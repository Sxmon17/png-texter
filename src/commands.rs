use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::error::{ChunkTypeError, PngError};
use crate::png::Png;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;

pub fn encode<S: AsRef<Path>>(
    input: S,
    chunk_type: &str,
    secret_msg: &str,
) -> Result<(), PngError> {
    let mut png = Png::from_file(input.as_ref()).expect("TODO: panic message");
    png.append_chunk(Chunk::new(
        ChunkType::from_str(chunk_type).unwrap(),
        secret_msg.as_bytes().to_vec(),
    ));

    let mut file = std::fs::File::create(input).unwrap();
    file.write_all(&png.as_bytes()).unwrap();
    Ok(())
}

