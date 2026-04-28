use std::fs;
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use anyhow::{Result, Context};
use serde::Serialize;
use crate::encoding::decode_to_utf8;
use super::validate_and_canonicalize;

#[derive(Serialize)]
pub struct InspectResult {
    pub size: u64,
    pub created: String,
    pub modified: String,
    pub total_lines: usize,
    pub estimated_encoding: String,
    pub search_results: Vec<SearchResult>,
}

#[derive(Serialize)]
pub struct SearchResult {
    pub line_number: usize,
    pub context: Vec<String>,
}

pub fn inspect_file(path: &str, search_query: Option<String>) -> Result<String> {
    let path_buf = validate_and_canonicalize(path)?;
    let metadata = fs::metadata(&path_buf)
        .with_context(|| format!("ファイル '{}' のメタデータ（作成日時やサイズなど）を取得できませんでした。", path))?;
    let size = metadata.len();
    
    let created = metadata.created().unwrap_or(SystemTime::UNIX_EPOCH);
    let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    
    let created_dt: DateTime<Utc> = created.into();
    let modified_dt: DateTime<Utc> = modified.into();
    
    let bytes = fs::read(&path_buf)
        .with_context(|| format!("ファイル '{}' の内容を読み取ることができませんでした。", path))?;
    let (content, encoding) = decode_to_utf8(&bytes);
    
    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len();
    
    let mut search_results = Vec::new();
    if let Some(query) = search_query {
        let query_lower = query.to_lowercase();
        for (i, line) in lines.iter().enumerate() {
            if line.to_lowercase().contains(&query_lower) {
                let start = if i >= 2 { i - 2 } else { 0 };
                let end = std::cmp::min(i + 3, total_lines);
                let context = lines[start..end].iter().map(|s| s.to_string()).collect();
                
                search_results.push(SearchResult {
                    line_number: i + 1,
                    context,
                });
                
                // Limit search results to avoid huge output
                if search_results.len() >= 10 {
                    break;
                }
            }
        }
    }
    
    let result = InspectResult {
        size,
        created: created_dt.to_rfc3339(),
        modified: modified_dt.to_rfc3339(),
        total_lines,
        estimated_encoding: encoding.to_string(),
        search_results,
    };
    
    serde_json::to_string_pretty(&result)
        .with_context(|| format!("ファイル '{}' の解析結果をJSON形式に変換できませんでした。", path))
}
