use tree_sitter::Language;

pub fn get_language(extension: &str) -> Option<Language> {
    match extension.to_lowercase().as_str() {
        "rs" => Some(tree_sitter_rust::language()),
        "py" => Some(tree_sitter_python::language()),
        "cs" => Some(tree_sitter_c_sharp::language()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_language() {
        assert!(get_language("rs").is_some());
        assert!(get_language("py").is_some());
        assert!(get_language("cs").is_some());
        assert!(get_language("unknown").is_none());
    }
}
