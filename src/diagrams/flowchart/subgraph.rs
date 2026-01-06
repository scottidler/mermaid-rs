use serde::{Deserialize, Serialize};

use crate::core::Direction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subgraph {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    pub nodes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    /// Nested subgraphs
    #[serde(default)]
    pub subgraphs: Vec<Subgraph>,
}

impl Subgraph {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: None,
            nodes: Vec::new(),
            direction: None,
            subgraphs: Vec::new(),
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_node(mut self, node_id: impl Into<String>) -> Self {
        self.nodes.push(node_id.into());
        self
    }

    pub fn with_nodes(mut self, node_ids: Vec<String>) -> Self {
        self.nodes = node_ids;
        self
    }

    pub fn with_direction(mut self, direction: Direction) -> Self {
        self.direction = Some(direction);
        self
    }

    /// Add a nested subgraph
    pub fn with_subgraph(mut self, subgraph: Subgraph) -> Self {
        self.subgraphs.push(subgraph);
        self
    }

    /// Renders the subgraph start in mermaid syntax
    pub fn to_mermaid_start(&self) -> String {
        let title = self.title.as_deref().unwrap_or(&self.id);
        let mut output = format!("subgraph {} [\"{}\"]\n", self.id, title);
        if let Some(dir) = &self.direction {
            output.push_str(&format!("    direction {}\n", dir));
        }
        output
    }

    /// Renders the subgraph end
    pub fn to_mermaid_end(&self) -> &'static str {
        "end"
    }

    /// Renders the complete subgraph with nested subgraphs (with indentation)
    pub fn to_mermaid_with_indent(&self, base_indent: &str) -> String {
        let title = self.title.as_deref().unwrap_or(&self.id);
        let mut output = format!("{}subgraph {} [\"{}\"]\n", base_indent, self.id, title);

        let inner_indent = format!("{}    ", base_indent);

        if let Some(dir) = &self.direction {
            output.push_str(&format!("{}direction {}\n", inner_indent, dir));
        }

        // Render nested subgraphs recursively
        for subgraph in &self.subgraphs {
            output.push_str(&subgraph.to_mermaid_with_indent(&inner_indent));
        }

        output.push_str(&format!("{}end\n", base_indent));
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subgraph_basic() {
        let sg = Subgraph::new("sg1").with_title("My Subgraph");
        assert!(sg.to_mermaid_start().contains("subgraph sg1"));
        assert!(sg.to_mermaid_start().contains("My Subgraph"));
    }

    #[test]
    fn subgraph_with_direction() {
        let sg = Subgraph::new("sg1")
            .with_title("Test")
            .with_direction(Direction::LeftRight);
        let output = sg.to_mermaid_start();
        assert!(output.contains("direction LR"));
    }

    #[test]
    fn nested_subgraph() {
        let inner = Subgraph::new("inner").with_title("Inner Group");
        let outer = Subgraph::new("outer")
            .with_title("Outer Group")
            .with_subgraph(inner);

        let output = outer.to_mermaid_with_indent("    ");
        assert!(output.contains("subgraph outer"));
        assert!(output.contains("subgraph inner"));
        assert!(output.contains("Inner Group"));
        assert!(output.contains("Outer Group"));
        // Check proper nesting with two end statements
        assert_eq!(output.matches("end").count(), 2);
    }

    #[test]
    fn deeply_nested_subgraph() {
        let level3 = Subgraph::new("level3").with_title("Level 3");
        let level2 = Subgraph::new("level2")
            .with_title("Level 2")
            .with_subgraph(level3);
        let level1 = Subgraph::new("level1")
            .with_title("Level 1")
            .with_subgraph(level2);

        let output = level1.to_mermaid_with_indent("    ");
        assert!(output.contains("Level 1"));
        assert!(output.contains("Level 2"));
        assert!(output.contains("Level 3"));
        assert_eq!(output.matches("end").count(), 3);
    }
}
