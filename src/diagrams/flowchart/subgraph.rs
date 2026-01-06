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
}

impl Subgraph {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: None,
            nodes: Vec::new(),
            direction: None,
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
}
