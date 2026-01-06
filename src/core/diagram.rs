use crate::core::{Config, MermaidError};

/// Trait implemented by all diagram types
pub trait Diagram: Send + Sync {
    /// Returns the mermaid syntax string for this diagram
    fn to_mermaid(&self) -> String;

    /// Returns the diagram type identifier (e.g., "flowchart", "sequenceDiagram")
    fn diagram_type(&self) -> &'static str;

    /// Returns optional title
    fn title(&self) -> Option<&str>;

    /// Returns optional configuration
    fn config(&self) -> Option<&Config>;

    /// Builds the complete mermaid script including frontmatter
    fn build_script(&self) -> String {
        let mut script = String::new();

        // Add YAML frontmatter if title or config present
        if self.title().is_some() || self.config().is_some() {
            script.push_str("---\n");
            if let Some(title) = self.title() {
                script.push_str(&format!("title: {}\n", title));
            }
            if let Some(config) = self.config() {
                script.push_str(&config.to_yaml());
            }
            script.push_str("---\n\n");
        }

        script.push_str(&self.to_mermaid());
        script
    }
}

/// Trait for diagram types that can be deserialized from config files
pub trait FromConfig: Diagram + Sized {
    fn from_json(json: &str) -> Result<Self, MermaidError>;
    fn from_yaml(yaml: &str) -> Result<Self, MermaidError>;
    fn from_toml(toml: &str) -> Result<Self, MermaidError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestDiagram {
        title: Option<String>,
        config: Option<Config>,
    }

    impl Diagram for TestDiagram {
        fn to_mermaid(&self) -> String {
            "graph TD\n  A --> B".to_string()
        }

        fn diagram_type(&self) -> &'static str {
            "flowchart"
        }

        fn title(&self) -> Option<&str> {
            self.title.as_deref()
        }

        fn config(&self) -> Option<&Config> {
            self.config.as_ref()
        }
    }

    #[test]
    fn build_script_without_frontmatter() {
        let diagram = TestDiagram {
            title: None,
            config: None,
        };
        let script = diagram.build_script();
        assert_eq!(script, "graph TD\n  A --> B");
    }

    #[test]
    fn build_script_with_title() {
        let diagram = TestDiagram {
            title: Some("My Diagram".to_string()),
            config: None,
        };
        let script = diagram.build_script();
        assert!(script.starts_with("---\n"));
        assert!(script.contains("title: My Diagram"));
        assert!(script.contains("graph TD"));
    }
}
