use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use crate::encoding::decode_to_utf8;
use anyhow::Result;
use super::validate_and_canonicalize;

pub fn safe_read(
    path: &str,
    range: Option<[usize; 2]>,
    tail: Option<usize>,
) -> Result<(String, &'static str)> {
    let path_buf = validate_and_canonicalize(path)?;
    let mut file = File::open(&path_buf).map_err(|e| {
        anyhow::anyhow!("Open Error: Failed to open '{}'. Details: {}", path, e)
    })?;
    let metadata = file.metadata().map_err(|e| {
        anyhow::anyhow!("Metadata Error: Failed to retrieve file size for '{}'. Details: {}", path, e)
    })?;
    let file_size = metadata.len() as usize;

    let mut buffer = Vec::new();

    if let Some(n) = tail {
        let start = if file_size > n { file_size - n } else { 0 };
        file.seek(SeekFrom::Start(start as u64)).map_err(|e| {
            anyhow::anyhow!("Seek Error: Failed to seek to tail position in '{}'. Details: {}", path, e)
        })?;
        file.read_to_end(&mut buffer).map_err(|e| {
            anyhow::anyhow!("Read Error: Failed to read tail content from '{}'. Details: {}", path, e)
        })?;
    } else if let Some([start, end]) = range {
        let actual_end = std::cmp::min(end, file_size);
        if start < actual_end {
            let len = actual_end - start;
            file.seek(SeekFrom::Start(start as u64)).map_err(|e| {
                anyhow::anyhow!("Seek Error: Failed to seek to position {} in '{}'. Details: {}", start, path, e)
            })?;
            buffer.resize(len, 0);
            file.read_exact(&mut buffer).map_err(|e| {
                anyhow::anyhow!("Read Error: Failed to read range content from '{}'. Details: {}", path, e)
            })?;
        }
    } else {
        file.read_to_end(&mut buffer).map_err(|e| {
            anyhow::anyhow!("Read Error: Failed to read full content from '{}'. Details: {}", path, e)
        })?;
    }

    let (content, encoding) = decode_to_utf8(&buffer);
    Ok((content, encoding))
}
