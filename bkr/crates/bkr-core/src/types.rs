//! types.rs
//! 
//! this file define the structure of the two main components of bkr: the user identifier and the file needing to be versioned.
//! the file structure is divided in a file metadata, witch is sent firstly to ensure the hash is new, and the actual
//! file structure containing the actual data, the version of the file and its storage path

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


/// unique user indentifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClientId(pub Uuid); // this tuple field help us distinguish a client uuid among other uuid

impl ClientId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }

    pub fn as_str(&self) -> String {
        self.0.to_string()
    }
}

impl Default for ClientId {
    fn default() -> Self {
        Self::new()
    }
}

// base file metadata sent before the actual data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub client_id: ClientId,
    pub relative_path: String,
    pub hash: String,
    pub size: u64,
    pub modified_at: DateTime<Utc>,
}

// structure of a file stored in the server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileVersion {
    pub id: Uuid,
    pub client_id: ClientId,
    pub relative_path: String,
    pub hash: String,
    pub size: u64,
    pub version_number: u32,
    pub created_at: DateTime<Utc>,
    pub storage_path: String,
}

impl FileVersion {
    pub fn new(
        client_id: ClientId,
        relative_path: String,
        hash: String,
        size: u64,
        version_number: u32,
        storage_path: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            client_id,
            relative_path,
            hash,
            size,
            version_number,
            created_at: Utc::now(),
            storage_path,
        }
    }
}
