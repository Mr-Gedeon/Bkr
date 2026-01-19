//! hash.rs
//! 
//! this file expose us a function able to perform a hash computation of a given file content using BLAKE3

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/// hash computation of a file using BLAKE3
pub fn compute_file_hash<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file: File = File::open(path)?;
    let mut hasher: blake3::Hasher = blake3::Hasher::new();
    let mut buffer: [u8; 8192] = [0; 8192];

    loop {
        let bytes_read: usize = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hasher.finalize().to_hex().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_hash_consistency() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"test content").unwrap();
        temp_file.flush().unwrap();

        let hash1 = compute_file_hash(temp_file.path()).unwrap();
        let hash2 = compute_file_hash(temp_file.path()).unwrap();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_different_content_different_hash() {
        let mut temp1 = NamedTempFile::new().unwrap();
        temp1.write_all(b"content 1").unwrap();
        temp1.flush().unwrap();

        let mut temp2 = NamedTempFile::new().unwrap();
        temp2.write_all(b"content 2").unwrap();
        temp2.flush().unwrap();

        let hash1 = compute_file_hash(temp1.path()).unwrap();
        let hash2 = compute_file_hash(temp2.path()).unwrap();

        assert_ne!(hash1, hash2);
    }
}