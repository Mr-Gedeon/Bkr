//! error.rs
//! 
//! this file enumerate all the error type used in thi project, and the result type giving thos erro in case of failure

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BkrError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] bincode::Error),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Hash mismatch: expected {expected}, got {actual}")]
    HashMismatch { expected: String, actual: String },

    #[error("Client not registered")]
    ClientNotRegistered,

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, BkrError>;