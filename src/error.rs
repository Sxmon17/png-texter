use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChunkTypeError {
    #[error("invalid byte length (expected {expected:?}, found {found:?})")]
    ByteLength { expected: String, found: String },

    #[error("`{0}` is not a ascii uppercase or lowercase character")]
    NotAscii(String),
}

#[derive(Error, Debug)]
pub enum ChunkError {
    #[error("Checksums do not match (expected {expected:?}, found {found:?})")]
    InvalidCrc{ expected: String, found: String },

    #[error("{0} not found")]
    ChunkNotFound(String),
}
