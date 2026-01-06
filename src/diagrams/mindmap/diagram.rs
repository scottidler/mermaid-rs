use serde::{Deserialize, Serialize};

use crate::core::{Config, Diagram, MermaidError};

use super::{MindmapNode, MindmapNodeShape};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mindmap {
    pub root: MindmapNode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip)]
    raw_mermaid: Option<String>,
}

impl Mindmap {
    pub fn builder(root_text: impl Into<String>) -> MindmapBuilder {
        MindmapBuilder::new(root_text)
    }

    pub fn from_raw_mermaid(script: String) -> Self {
        Self {
            root: MindmapNode::new(""),
            title: None,
            raw_mermaid: Some(script),
        }
    }

    pub fn from_json(json: &str) -> Result<Self, MermaidError> {
        serde_json::from_str(json).map_err(|e| MermaidError::ParseError(e.to_string()))
    }

    pub fn from_yaml(yaml: &str) -> Result<Self, MermaidError> {
        serde_yaml::from_str(yaml).map_err(|e| MermaidError::ParseError(e.to_string()))
    }

    pub fn from_toml(toml: &str) -> Result<Self, MermaidError> {
        toml::from_str(toml).map_err(|e| MermaidError::ParseError(e.to_string()))
    }
}

impl Diagram for Mindmap {
    fn diagram_type(&self) -> &'static str {
        "mindmap"
    }

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn config(&self) -> Option<&Config> {
        None
    }

    fn to_mermaid(&self) -> String {
        if let Some(raw) = &self.raw_mermaid {
            return raw.clone();
        }

        let mut output = String::from("mindmap\n");
        output.push_str(&self.root.to_mermaid(1));
        output
    }
}

#[derive(Debug)]
pub struct MindmapBuilder {
    root: MindmapNode,
    title: Option<String>,
}

impl MindmapBuilder {
    pub fn new(root_text: impl Into<String>) -> Self {
        Self {
            root: MindmapNode::new(root_text),
            title: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn root_shape(mut self, shape: MindmapNodeShape) -> Self {
        self.root = self.root.with_shape(shape);
        self
    }

    pub fn root_icon(mut self, icon: impl Into<String>) -> Self {
        self.root = self.root.with_icon(icon);
        self
    }

    /// Add a child to the root node
    pub fn child(mut self, text: impl Into<String>) -> Self {
        self.root.children.push(MindmapNode::new(text));
        self
    }

    /// Add a child with shape to the root node
    pub fn child_with_shape(mut self, text: impl Into<String>, shape: MindmapNodeShape) -> Self {
        self.root
            .children
            .push(MindmapNode::new(text).with_shape(shape));
        self
    }

    /// Add a full node tree as child
    pub fn child_node(mut self, node: MindmapNode) -> Self {
        self.root.children.push(node);
        self
    }

    pub fn build(self) -> Mindmap {
        Mindmap {
            root: self.root,
            title: self.title,
            raw_mermaid: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mindmap_basic() {
        let mindmap = Mindmap::builder("Root")
            .child("Child1")
            .child("Child2")
            .build();

        let mermaid = mindmap.to_mermaid();
        assert!(mermaid.contains("mindmap"));
        assert!(mermaid.contains("Root"));
        assert!(mermaid.contains("Child1"));
        assert!(mermaid.contains("Child2"));
    }

    #[test]
    fn mindmap_with_title() {
        let mindmap = Mindmap::builder("Root").title("My Mindmap").build();

        assert_eq!(mindmap.title(), Some("My Mindmap"));
    }

    #[test]
    fn mindmap_from_json() {
        let json = r#"{
            "root": {
                "text": "Main",
                "children": [
                    {"text": "Sub1"},
                    {"text": "Sub2"}
                ]
            }
        }"#;

        let mindmap = Mindmap::from_json(json).unwrap();
        assert_eq!(mindmap.root.text, "Main");
        assert_eq!(mindmap.root.children.len(), 2);
    }

    #[test]
    fn mindmap_from_yaml() {
        let yaml = r#"
root:
  text: Main
  children:
    - text: Sub1
    - text: Sub2
"#;

        let mindmap = Mindmap::from_yaml(yaml).unwrap();
        assert_eq!(mindmap.root.text, "Main");
        assert_eq!(mindmap.root.children.len(), 2);
    }

    #[test]
    fn mindmap_raw_mermaid() {
        let raw = "mindmap\n    Root\n        Child";
        let mindmap = Mindmap::from_raw_mermaid(raw.to_string());
        assert_eq!(mindmap.to_mermaid(), raw);
    }
}
