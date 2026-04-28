pub fn get_definition_info(
    _lang: &str,
    node: tree_sitter::Node,
    source: &[u8],
) -> Option<(String, String)> {
    let kind = node.kind();

    let is_definition = match kind {
        // JSON / TOML
        "pair" | "table" => true,
        // YAML
        "block_mapping_pair" => true,
        // SQL
        "create_table_statement" | "create_view_statement" | "create_index_statement" => true,
        // Docker
        "instruction" => true,
        _ => false,
    };

    if !is_definition {
        return None;
    }

    let name_node = match kind {
        "pair" => node
            .child_by_field_name("key")
            .or_else(|| node.child_by_field_name("name")),
        "table" => node.child_by_field_name("name"),
        "block_mapping_pair" => node.child_by_field_name("key"),
        _ => None,
    };

    let name = if let Some(n) = name_node {
        n.utf8_text(source).unwrap_or("unknown").to_string()
    } else if kind == "instruction" || kind.contains("statement") {
        let text = node.utf8_text(source).unwrap_or("unknown");
        text.trim().to_string()
    } else {
        "unknown".to_string()
    };

    Some((kind.to_string(), name))
}
