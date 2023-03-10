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
    InvalidCrc { expected: String, found: String },

    #[error("{0} not found")]
    ChunkNotFound(String),
}

#[derive(Error, Debug)]
pub enum PngError {
    #[error("invalid PNG signature (expected {expected:?}, found {found:?})")]
    InvalidSignature {
        expected: Box<[u8]>,
        found: Box<[u8]>,
    },

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Chunk(#[from] ChunkError),

    #[error(transparent)]
    ChunkType(#[from] ChunkTypeError),

    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),

    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    Http(#[from] attohttpc::Error),
}
