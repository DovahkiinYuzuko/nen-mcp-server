pub mod safe_read;
pub mod get_outline;
pub mod inspect_file;
pub mod read_hex;

use std::path::PathBuf;
use anyhow::{Result, Context};

/// Validates that the path exists, is a file, and returns its canonicalized form.
pub fn validate_and_canonicalize(path: &str) -> Result<PathBuf> {
    let path_buf = dunce::canonicalize(path)
        .with_context(|| format!("指定されたパス '{}' の正規化（存在確認）に失敗しました。パスが正しいか、権限があるか確認してください。", path))?;
    
    if !path_buf.is_file() {
        return Err(anyhow::anyhow!("パス '{}' は存在しますが、ファイルではありません。このツールは単一のファイルのみを対象としています。", path_buf.display()));
    }
    Ok(path_buf)
}
