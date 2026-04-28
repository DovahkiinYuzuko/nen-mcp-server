use crate::tools::safe_read::safe_read;
use crate::parser::get_language;
use tree_sitter::{Parser, Node};
use anyhow::{Result, anyhow};
use std::path::Path;
use serde::Serialize;

#[derive(Serialize)]
struct OutlineItem {
    line: u32,
    kind: String,
    name: String,
}

pub fn get_outline(path: &str) -> Result<String> {
    let (content, _) = safe_read(path, None, None)?;
    let extension = Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow!("Outline Error: No file extension found for '{}'. Extension is required to determine the language parser.", path))?;

    let language = get_language(extension)
        .ok_or_else(|| anyhow!("Outline Error: Unsupported file extension '.{}' for path '{}'. Supported extensions are: .rs, .py, .cs.", extension, path))?;

    let mut parser = Parser::new();
    parser.set_language(language)
        .map_err(|e| anyhow!("Outline Error: Internal error setting up parser for language '{}': {}", extension, e))?;

    let tree = parser.parse(&content, None)
        .ok_or_else(|| anyhow!("Outline Error: Failed to parse content of '{}'. The file might be corrupted or in an unexpected format.", path))?;

    let mut outline = Vec::new();
    traverse(tree.root_node(), &content, &mut outline);

    serde_json::to_string(&outline).map_err(|e| anyhow!("Outline Error: Internal error serializing outline JSON for '{}': {}", path, e))
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
