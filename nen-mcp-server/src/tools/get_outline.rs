use crate::tools::safe_read::safe_read;
use crate::parser::get_language;
use tree_sitter::{Parser, Node};
use anyhow::{Result, anyhow, Context};
use std::path::Path;
use serde::Serialize;

#[derive(Serialize)]
struct OutlineItem {
    line: u32,
    kind: String,
    name: String,
}

pub fn get_outline(path: &str) -> Result<String> {
    let (content, _) = safe_read(path, None, None)
        .with_context(|| format!("アウトライン取得のためのファイル読み取り（'{}'）に失敗しました。", path))?;
    
    let extension = Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow!("ファイル '{}' に拡張子がありません。パーサーを決定するために拡張子が必要です。", path))?;

    let language = get_language(extension)
        .ok_or_else(|| anyhow!("拡張子 '.{}' はサポートされていません（ファイル: '{}'）。現在サポートされているのは .rs, .py, .cs, .js, .ts, .java, .c, .cpp, .go, .kt, .html, .css, .json, .toml, .yaml, .sql, .php, .rb, .swift, .dockerfile, .nix, .md, .ps1 です。", extension, path))?;

    let mut parser = Parser::new();
    parser.set_language(&language)
        .with_context(|| format!("言語 '{}' 用のパーサーのセットアップに失敗しました。", extension))?;    

    let tree = parser.parse(&content, None)
        .ok_or_else(|| anyhow!("ファイル '{}' の解析（パース）に失敗しました。ファイルが壊れているか、対応していない形式の可能性があります。", path))?;

    let mut outline = Vec::new();
    traverse(extension, tree.root_node(), &content, &mut outline);

    serde_json::to_string(&outline)
        .with_context(|| format!("ファイル '{}' のアウトライン結果（JSON）の生成に失敗しました。", path)) 
}

fn traverse(lang: &str, node: Node, source: &str, outline: &mut Vec<OutlineItem>) {
    if let Some((kind, name)) = crate::parser::languages::get_definition_info(lang, node, source.as_bytes()) {
        let line = node.start_position().row as u32 + 1;
        outline.push(OutlineItem {
            line,
            kind,
            name,
        });
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        traverse(lang, child, source, outline);
    }
}
