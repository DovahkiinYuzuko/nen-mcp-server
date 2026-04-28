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
    let mut file = File::open(&path_buf)?;
    let metadata = file.metadata()?;
    let file_size = metadata.len() as usize;

    let mut buffer = Vec::new();

    if let Some(n) = tail {
        let start = if file_size > n { file_size - n } else { 0 };
        file.seek(SeekFrom::Start(start as u64))?;
        file.read_to_end(&mut buffer)?;
    } else if let Some([start, end]) = range {
        let actual_end = std::cmp::min(end, file_size);
        if start < actual_end {
            let len = actual_end - start;
            file.seek(SeekFrom::Start(start as u64))?;
            buffer.resize(len, 0);
            file.read_exact(&mut buffer)?;
        }
    } else {
        file.read_to_end(&mut buffer)?;
    }

    let (content, encoding) = decode_to_utf8(&buffer);
    Ok((content, encoding))
}
