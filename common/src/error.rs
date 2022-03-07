use thiserror::Error;
use prost;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("prost encode error")]
    ProstEncodeError(#[from] prost::EncodeError),
    #[error("prost decode error")]
    ProstDecodeError(#[from] prost::DecodeError),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}