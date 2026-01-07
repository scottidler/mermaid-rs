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

    /// Builds the complete mermaid script including init directive
    fn build_script(&self) -> String {
        let mut script = String::new();

        // Add %%{init}%% directive for config (mermaid.ink compatible)
        if let Some(config) = self.config() {
            script.push_str(&config.to_init_directive());
            script.push('\n');
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
    fn build_script_with_config() {
        let diagram = TestDiagram {
            title: None,
            config: Some(Config::new().with_theme(crate::core::Theme::Dark)),
        };
        let script = diagram.build_script();
        assert!(script.starts_with("%%{init:"));
        assert!(script.contains("'theme': 'dark'"));
        assert!(script.contains("graph TD"));
    }
}
