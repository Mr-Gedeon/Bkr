//! protocol.rs
//! 
//! this file define the different message type that both the client and the server can transmit to each other

use crate::types::{ClientId, FileMetadata};
use serde::{Deserialize, Serialize};

/// allowed messages transmitted between client and server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {

    /// client regitration
    ClientRegister { client_id: ClientId },
    
    /// server confirmation of client registration
    RegisterAck { success: bool, message: String },
    
    /// file metadata sent from client
    FileMetadata { metadata: FileMetadata },
    
    /// server's request of file data
    RequestFileContent { hash: String },
    
    /// server's response if file already exist
    FileAlreadyExists { hash: String },
    
    /// client transmition of file data (compressed)
    FileContent { 
        hash: String, 
        compressed_data: Vec<u8>,
        original_size: u64,
    },
    
    /// server confirmation of client file transmition
    FileReceived { 
        hash: String, 
        version_number: u32,
        success: bool,
    },
    
    /// Error
    Error { message: String },
    
    /// Heartbeat to keep client-server connection
    Ping,
    Pong,
}

/// messages data type encaptulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub msg_type: MessageType,
}

impl Message {
    pub fn new(msg_type: MessageType) -> Self {
        Self { msg_type }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(bytes)
    }
}
