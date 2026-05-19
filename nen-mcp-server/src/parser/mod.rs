use tree_sitter::Language;

pub mod languages;

pub fn get_language(extension: &str) -> Option<Language> {
    match extension.to_lowercase().as_str() {
        "rs" => Some(tree_sitter_rust::language()),
        "py" => Some(tree_sitter_python::language()),
        "cs" => Some(tree_sitter_c_sharp::language()),
        "js" => Some(tree_sitter_javascript::language()),
        "ts" => Some(tree_sitter_typescript::language_typescript()),
        "java" => Some(tree_sitter_java::language()),
        "c" => Some(tree_sitter_c::language()),
        "cpp" | "cc" | "cxx" | "hpp" | "h" => Some(tree_sitter_cpp::language()),
        "go" => Some(tree_sitter_go::language()),
        // "kt" | "kts" => Some(tree_sitter_kotlin::language()),
        "html" | "htm" => Some(tree_sitter_html::language()),
        "css" => Some(tree_sitter_css::language()),
        "json" => Some(tree_sitter_json::language()),
        // "toml" => Some(tree_sitter_toml::language()),
        // "yaml" | "yml" => Some(tree_sitter_yaml::language()),
        // "sql" => Some(tree_sitter_sql::language()),
        // "php" => Some(tree_sitter_php::language()),
        // "rb" => Some(tree_sitter_ruby::language()),
        // "swift" => Some(tree_sitter_swift::language()),
        // "dockerfile" | "docker" => Some(tree_sitter_dockerfile::language()),
        // "nix" => Some(tree_sitter_nix::language()),
        // "md" | "markdown" => Some(tree_sitter_markdown::language()),
        "ps1" | "psm1" => Some(tree_sitter_powershell::language()),
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
        assert!(get_language("js").is_some());
        assert!(get_language("ts").is_some());
        assert!(get_language("java").is_some());
        assert!(get_language("c").is_some());
        assert!(get_language("cpp").is_some());
        assert!(get_language("go").is_some());
        // assert!(get_language("kt").is_some());
        assert!(get_language("html").is_some());
        assert!(get_language("css").is_some());
        assert!(get_language("json").is_some());
        // assert!(get_language("toml").is_some());
        // assert!(get_language("yaml").is_some());
        // assert!(get_language("sql").is_some());
        // assert!(get_language("php").is_some());
        // assert!(get_language("rb").is_some());
        // assert!(get_language("swift").is_some());
        // assert!(get_language("dockerfile").is_some());
        // assert!(get_language("nix").is_some());
        // assert!(get_language("md").is_some());
        assert!(get_language("ps1").is_some());
        assert!(get_language("unknown").is_none());
    }
}
