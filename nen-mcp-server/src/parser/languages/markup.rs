pub fn get_definition_info(
    _lang: &str,
    node: tree_sitter::Node,
    source: &[u8],
) -> Option<(String, String)> {
    let kind = node.kind();

    let is_definition = match kind {
        // Markdown
        "atx_heading" => true,
        _ => false,
    };

    if !is_definition {
        return None;
    }

    let name = if kind == "atx_heading" {
        let text = node.utf8_text(source).unwrap_or("unknown");
        text.trim().to_string()
    } else {
        "unknown".to_string()
    };

    Some((kind.to_string(), name))
}
