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
        // Common patterns
        "class_declaration" | "method_declaration" | "interface_declaration" | "enum_declaration" | "struct_declaration" | "function_declaration" => true,
        // Rust
        "function_item" | "struct_item" | "enum_item" | "trait_item" | "mod_item" | "type_item" | "impl_item" => true,
        // Python / C / C++ / PHP / PowerShell
        "function_definition" | "class_definition" => true,
        // C#
        "namespace_declaration" => true,
        // JavaScript / TypeScript / PHP
        "method_definition" | "type_alias_declaration" | "trait_declaration" => true,
        // C / C++
        "class_specifier" | "struct_specifier" | "enum_specifier" | "namespace_definition" => true,       
        // Go / Swift
        "type_declaration" | "protocol_declaration" => true,
        // Ruby
        "method" | "class" | "module" => true,
        // JSON / TOML
        "pair" | "table" => true,
        // YAML
        "block_mapping_pair" => true,
        // SQL
        "create_table_statement" | "create_view_statement" | "create_index_statement" => true,
        // Docker
        "instruction" => true,
        // Markdown
        "atx_heading" => true,
        // Nix
        "binding" => true,
        _ => false,
    };

    if is_definition {
        let name_node = match kind {
            "function_item" | "struct_item" | "enum_item" | "trait_item" | "mod_item" | "type_item" => node.child_by_field_name("name"),
            "function_definition" | "class_definition" | "method" | "class" | "module" => node.child_by_field_name("name"),
            "method_declaration" | "class_declaration" | "struct_declaration" | "interface_declaration" | "enum_declaration" | "namespace_declaration" => node.child_by_field_name("name"),
            "function_declaration" | "method_definition" | "type_alias_declaration" | "namespace_definition" | "trait_declaration" | "protocol_declaration" => node.child_by_field_name("name"),
            "class_specifier" | "struct_specifier" | "enum_specifier" | "type_declaration" => node.child_by_field_name("name"),
            "pair" => node.child_by_field_name("key").or_else(|| node.child_by_field_name("name")),
            "table" => node.child_by_field_name("name"),
            "block_mapping_pair" => node.child_by_field_name("key"),
            "binding" => node.child_by_field_name("attrpath"),
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
        } else if kind == "instruction" || kind == "atx_heading" || kind.contains("statement") {
            node.utf8_text(source.as_bytes()).unwrap_or("unknown").trim().to_string()
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
