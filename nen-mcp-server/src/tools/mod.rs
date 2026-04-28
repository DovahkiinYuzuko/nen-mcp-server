pub mod safe_read;
pub mod get_outline;
pub mod inspect_file;
pub mod read_hex;

use std::path::PathBuf;
use anyhow::Result;

/// Validates that the path exists, is a file, and returns its canonicalized form.
pub fn validate_and_canonicalize(path: &str) -> Result<PathBuf> {
    let path_buf = std::fs::canonicalize(path)?;
    if !path_buf.is_file() {
        return Err(anyhow::anyhow!("Not a file: {}", path_buf.display()));
    }
    Ok(path_buf)
}
