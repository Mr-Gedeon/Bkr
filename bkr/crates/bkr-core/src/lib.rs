//! lib.rs
//! 
//! This library file expose the modules accessible outside of the crate and help using the
//! different data types wihout needing to write the whole module path
//! 
//! ex: instead of 'use bkr_core::error::BkrError;'
//! we can now use 'use bkr_core::{BkrError};'


pub mod protocol;
pub mod types;
pub mod hash;
pub mod error;

pub use error::{BkrError, Result};
pub use hash::compute_file_hash;
pub use protocol::{Message, MessageType};
pub use types::{ClientId, FileMetadata, FileVersion};
