use std::io::Write;
use std::path::Path;
use std::str::FromStr;

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::error::{ChunkError, PngError};
use crate::png::Png;

pub fn encode<S, T>(input: S, output: T, chunk_type: &str, secret_msg: &str) -> Result<(), PngError>
where
    S: AsRef<Path>,
    T: AsRef<Path>,
{
    let mut png = Png::from_file(input.as_ref()).expect("Png invalid");

    png.append_chunk(Chunk::new(
        ChunkType::from_str(chunk_type).unwrap(),
        secret_msg.as_bytes().to_vec(),
    ));

    let mut file = std::fs::File::create(output).unwrap();
    file.write_all(&png.as_bytes()).unwrap();
    Ok(())
}

pub fn decode<S: AsRef<Path>>(input: S, chunk_type: &str) -> Result<String, PngError> {
    let mut png = Png::from_file(input.as_ref()).expect("Png invalid");
    let chunk = png.chunk_by_type(chunk_type);

    if chunk.is_none() {
        return Err(ChunkError::ChunkNotFound(chunk_type.to_string()).into());
    }

    let msg = String::from_utf8(chunk.unwrap().data.clone()).unwrap();

    Ok(msg)
}

pub fn remove<S: AsRef<Path>>(input: S, chunk_type: &str) -> Result<(), PngError> {
    let mut png = Png::from_file(input.as_ref()).expect("Png invalid");
    png.remove_chunk(chunk_type).expect("Failed removing chunk");
    let mut file = std::fs::File::create(input)?;
    file.write_all(&png.as_bytes())?;

    println!("Chunk removed");

    Ok(())
}
