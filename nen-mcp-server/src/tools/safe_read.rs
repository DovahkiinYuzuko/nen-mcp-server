use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use crate::encoding::decode_to_utf8;
use anyhow::{Result, Context};
use super::validate_and_canonicalize;

pub fn safe_read(
    path: &str,
    range: Option<[usize; 2]>,
    tail: Option<usize>,
) -> Result<(String, &'static str)> {
    let path_buf = validate_and_canonicalize(path)?;
    let mut file = File::open(&path_buf)
        .with_context(|| format!("ファイル '{}' を開くことができませんでした。", path))?;
    let metadata = file.metadata()
        .with_context(|| format!("ファイル '{}' のメタデータ（サイズ等）を取得できませんでした。", path))?;
    let file_size = metadata.len() as usize;

    let mut buffer = Vec::new();

    if let Some(n) = tail {
        let start = if file_size > n { file_size - n } else { 0 };
        file.seek(SeekFrom::Start(start as u64))
            .with_context(|| format!("ファイル '{}' の末尾 {} バイトへのシークに失敗しました。", path, n))?;
        file.read_to_end(&mut buffer)
            .with_context(|| format!("ファイル '{}' の末尾内容の読み取りに失敗しました。", path))?;
    } else if let Some([start, end]) = range {
        let actual_end = std::cmp::min(end, file_size);
        if start < actual_end {
            let len = actual_end - start;
            file.seek(SeekFrom::Start(start as u64))
                .with_context(|| format!("ファイル '{}' の指定位置（{}）へのシークに失敗しました。", path, start))?;
            buffer.resize(len, 0);
            file.read_exact(&mut buffer)
                .with_context(|| format!("ファイル '{}' の指定範囲（{} バイト）の読み取りに失敗しました。", path, len))?;
        }
    } else {
        file.read_to_end(&mut buffer)
            .with_context(|| format!("ファイル '{}' の全内容の読み取りに失敗しました。", path))?;
    }

    let (content, encoding) = decode_to_utf8(&buffer);
    Ok((content, encoding))
}
