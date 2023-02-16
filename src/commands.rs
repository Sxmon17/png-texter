use std::io::Write;
use std::path::Path;
use std::str::FromStr;

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::error::{ChunkError, PngError};
use crate::png::Png;

use std::fs::File;

pub fn encode<S, T>(input: S, output: T, chunk_type: &str, secret_msg: &str) -> Result<(), PngError>
where
    S: AsRef<Path>,
    T: AsRef<Path>,
{
    let mut png = Png::from_file(input.as_ref())?;

    png.append_chunk(Chunk::new(
        ChunkType::from_str(chunk_type)?,
        secret_msg.as_bytes().to_vec(),
    ));

    let mut file = File::create(output)?;
    file.write_all(&png.as_bytes())?;
    Ok(())
}

pub fn decode<S: AsRef<Path>>(input: S, chunk_type: &str) -> Result<String, PngError> {
    let png = Png::from_file(input.as_ref())?;
    let chunk = png
        .chunk_by_type(chunk_type)
        .ok_or(ChunkError::ChunkNotFound(chunk_type.to_string()))?;
    let msg = String::from_utf8(chunk.data.clone()).map_err(|e| PngError::FromUtf8(e))?;

    Ok(msg)
}

pub fn remove<S: AsRef<Path>>(input: S, chunk_type: &str) -> Result<(), PngError> {
    let mut png = Png::from_file(input.as_ref())?;
    png.remove_chunk(chunk_type)
        .map_err(|e| PngError::Chunk(e))?;
    let mut file = File::create(input)?;
    file.write_all(&png.as_bytes())?;

    println!("Chunk removed");

    Ok(())
}

pub fn encode_from_url<S: AsRef<Path>>(
    url: &str,
    output: S,
    chunk_type: &str,
    secret_msg: &str,
) -> Result<(), PngError> {
    let resp = attohttpc::get(url).send().map_err(|e| PngError::Http(e))?;
    let mut png = Png::try_from(&*resp.bytes().unwrap())?;
    png.append_chunk(Chunk::new(
        ChunkType::from_str(chunk_type)?,
        secret_msg.as_bytes().to_vec(),
    ));
    let mut file = File::create(output)?;
    file.write_all(&png.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        let input = "png_tests/good_normal_tiny-rgb-gray.png";
        let output = "png_tests/test.png";
        let chunk_type = "test";
        let secret_msg = "Hello world!";

        encode(input, output, chunk_type, secret_msg).unwrap();
        let msg = decode(output, chunk_type).unwrap();

        assert_eq!(msg, secret_msg);

        remove(output, chunk_type).unwrap();
    }
}
