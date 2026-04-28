pub fn get_definition_info(
    _lang: &str,
    node: tree_sitter::Node,
    source: &[u8],
) -> Option<(String, String)> {
    let kind = node.kind();

    let is_definition = match kind {
        // Common patterns
        "class_declaration"
        | "method_declaration"
        | "interface_declaration"
        | "enum_declaration"
        | "struct_declaration"
        | "function_declaration" => true,
        // Rust
        "function_item"
        | "struct_item"
        | "enum_item"
        | "trait_item"
        | "mod_item"
        | "type_item"
        | "impl_item" => true,
        // Python / C / C++
        "function_definition" | "class_definition" => true,
        // C#
        "namespace_declaration" => true,
        // C / C++
        "class_specifier" | "struct_specifier" | "enum_specifier" | "namespace_definition" => true,
        // Go / Swift
        "type_declaration" | "protocol_declaration" => true,
        // Ruby
        "method" | "class" | "module" => true,
        // Nix
        "binding" => true,
        _ => false,
    };

    if !is_definition {
        return None;
    }

    let name_node = match kind {
        "function_item" | "struct_item" | "enum_item" | "trait_item" | "mod_item" | "type_item" => {
            node.child_by_field_name("name")
        }
        "function_definition" | "class_definition" | "method" | "class" | "module" => {
            node.child_by_field_name("name")
        }
        "method_declaration"
        | "class_declaration"
        | "struct_declaration"
        | "interface_declaration"
        | "enum_declaration"
        | "namespace_declaration" => node.child_by_field_name("name"),
        "function_declaration" | "namespace_definition" | "protocol_declaration" => {
            node.child_by_field_name("name")
        }
        "class_specifier" | "struct_specifier" | "enum_specifier" | "type_declaration" => {
            node.child_by_field_name("name")
        }
        "binding" => node.child_by_field_name("attrpath"),
        _ => None,
    };

    let name = if let Some(n) = name_node {
        n.utf8_text(source).unwrap_or("unknown").to_string()
    } else if kind == "impl_item" {
        if let Some(type_node) = node.child_by_field_name("type") {
            type_node.utf8_text(source).unwrap_or("unknown").to_string()
        } else {
            "impl".to_string()
        }
    } else {
        "unknown".to_string()
    };

    Some((kind.to_string(), name))
}
