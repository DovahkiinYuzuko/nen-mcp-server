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
        .ok_or_else(|| anyhow!("拡張子 '.{}' はサポートされていません（ファイル: '{}'）。現在サポートされているのは .rs, .py, .cs です。", extension, path))?;

    let mut parser = Parser::new();
    parser.set_language(language)
        .with_context(|| format!("言語 '{}' 用のパーサーのセットアップに失敗しました。", extension))?;

    let tree = parser.parse(&content, None)
        .ok_or_else(|| anyhow!("ファイル '{}' の解析（パース）に失敗しました。ファイルが壊れているか、対応していない形式の可能性があります。", path))?;

    let mut outline = Vec::new();
    traverse(tree.root_node(), &content, &mut outline);

    serde_json::to_string(&outline)
        .with_context(|| format!("ファイル '{}' のアウトライン結果（JSON）の生成に失敗しました。", path))
}

fn traverse(node: Node, source: &str, outline: &mut Vec<OutlineItem>) {
    let kind = node.kind();
    
    let is_definition = match kind {
        // Rust
        "function_item" | "struct_item" | "enum_item" | "trait_item" | "mod_item" | "type_item" | "impl_item" => true,
        // Python
        "function_definition" | "class_definition" => true,
        // C#
        "class_declaration" | "method_declaration" | "struct_declaration" | "interface_declaration" | "enum_declaration" | "namespace_declaration" => true,
        _ => false,
    };

    if is_definition {
        let name_node = match kind {
            "function_item" | "struct_item" | "enum_item" | "trait_item" | "mod_item" | "type_item" => node.child_by_field_name("name"),
            "function_definition" | "class_definition" => node.child_by_field_name("name"),
            "method_declaration" | "class_declaration" | "struct_declaration" | "interface_declaration" | "enum_declaration" | "namespace_declaration" => node.child_by_field_name("name"),
            _ => None,
        };

        let name = if let Some(n) = name_node {
            n.utf8_text(source.as_bytes()).unwrap_or("unknown").to_string()
        } else if kind == "impl_item" {
            if let Some(type_node) = node.child_by_field_name("type") {
                type_node.utf8_text(source.as_bytes()).unwrap_or("unknown").to_string()
            } else {
                "impl".to_string()
            }
        } else {
            "unknown".to_string()
        };

        let line = node.start_position().row as u32 + 1;
        outline.push(OutlineItem {
            line,
            kind: kind.to_string(),
            name,
        });
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        traverse(child, source, outline);
    }
}
