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
        // PHP / PowerShell
        "function_definition" | "class_definition" | "function_statement" | "class_statement" => true,
        // JavaScript / TypeScript / PHP
        "method_definition" | "type_alias_declaration" | "trait_declaration" => true,
        _ => false,
    };

    if !is_definition {
        return None;
    }

    let name_node = match kind {
        "function_definition" | "class_definition" => node.child_by_field_name("name"),
        "function_statement" | "class_statement" => {
            // PowerShell: "name" フィールドがない場合は、function_name ノードなどを探す
            node.child_by_field_name("name")
                .or_else(|| node.children(&mut node.walk()).find(|c| c.kind() == "function_name" || c.kind() == "class_name"))
        },
        "method_declaration"
        | "class_declaration"
        | "struct_declaration"
        | "interface_declaration"
        | "enum_declaration" => node.child_by_field_name("name"),
        "function_declaration"
        | "method_definition"
        | "type_alias_declaration"
        | "trait_declaration" => node.child_by_field_name("name"),
        _ => None,
    };

    let name = if let Some(n) = name_node {
        n.utf8_text(source).unwrap_or("unknown").to_string()
    } else {
        "unknown".to_string()
    };

    Some((kind.to_string(), name))
}
