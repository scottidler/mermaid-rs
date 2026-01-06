/// Normalize an ID for mermaid syntax.
///
/// Converts to snake_case by:
/// - Replacing non-alphanumeric characters (except `_`, `.`, `-`) with underscores
/// - Converting to lowercase
///
/// This matches the behavior of mermaid-py's `text_to_snake_case()` function.
pub fn normalize_id(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' || c == '.' || c == '-' {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_simple_text() {
        assert_eq!(normalize_id("Hello"), "hello");
        assert_eq!(normalize_id("UPPERCASE"), "uppercase");
    }

    #[test]
    fn normalize_with_spaces() {
        assert_eq!(normalize_id("First Node"), "first_node");
        assert_eq!(normalize_id("My Cool Node"), "my_cool_node");
    }

    #[test]
    fn normalize_preserves_valid_chars() {
        assert_eq!(normalize_id("node_1"), "node_1");
        assert_eq!(normalize_id("node-2"), "node-2");
        assert_eq!(normalize_id("node.3"), "node.3");
    }

    #[test]
    fn normalize_special_chars() {
        assert_eq!(normalize_id("node@#$%"), "node____");
        assert_eq!(normalize_id("hello!world"), "hello_world");
    }

    #[test]
    fn normalize_mixed_case_with_spaces() {
        assert_eq!(normalize_id("User Authentication"), "user_authentication");
        assert_eq!(normalize_id("API Gateway"), "api_gateway");
    }
}
