pub mod configs_data;
pub mod markup;
pub mod programming;
pub mod web_scripts;

pub fn get_definition_info(
    lang: &str,
    node: &tree_sitter::Node,
    source: &[u8],
) -> Option<(String, String)> {
    match lang.to_lowercase().as_str() {
        // Programming languages
        "rs" | "py" | "cs" | "java" | "c" | "cpp" | "cc" | "cxx" | "hpp" | "h" | "go" | "kt"
        | "kts" | "rb" | "swift" | "nix" => programming::get_definition_info(node, source),

        // Web scripts
        "js" | "ts" | "php" => web_scripts::get_definition_info(node, source),

        // Configs and Data
        "json" | "toml" | "yaml" | "yml" | "sql" | "dockerfile" | "docker" => {
            configs_data::get_definition_info(node, source)
        }

        // Markup and others
        "html" | "htm" | "css" | "md" | "markdown" | "ps1" | "psm1" => {
            markup::get_definition_info(node, source)
        }

        _ => None,
    }
}
