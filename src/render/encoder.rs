use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

/// Encode a mermaid diagram script for use in mermaid.ink URLs
pub fn encode_diagram(script: &str) -> String {
    URL_SAFE_NO_PAD.encode(script.as_bytes())
}

/// Decode a base64-encoded mermaid diagram script
#[allow(dead_code)]
pub fn decode_diagram(encoded: &str) -> Result<String, base64::DecodeError> {
    let bytes = URL_SAFE_NO_PAD.decode(encoded)?;
    Ok(String::from_utf8_lossy(&bytes).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_roundtrip() {
        let script = "graph TD\n  A --> B";
        let encoded = encode_diagram(script);
        let decoded = decode_diagram(&encoded).unwrap();
        assert_eq!(decoded, script);
    }

    #[test]
    fn encode_produces_url_safe_string() {
        let script = "graph TD\n  A[Start] --> B{Decision}\n  B -->|Yes| C[End]";
        let encoded = encode_diagram(script);
        // URL-safe base64 should not contain +, /, or =
        assert!(!encoded.contains('+'));
        assert!(!encoded.contains('/'));
    }
}
