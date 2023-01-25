mod args;
mod chunk;
mod chunk_type;
mod commands;
mod error;
mod png;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    todo!()
}
