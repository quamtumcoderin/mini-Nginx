use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Error de I/O: {0}")]
    Io(#[from] io::Error)
}