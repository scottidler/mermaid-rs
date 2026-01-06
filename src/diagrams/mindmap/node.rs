use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindmapNode {
    pub text: String,
    #[serde(default)]
    pub shape: NodeShape,
    #[serde(default)]
    pub children: Vec<MindmapNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
}

impl MindmapNode {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            shape: NodeShape::default(),
            children: Vec::new(),
            icon: None,
            class: None,
        }
    }

    pub fn with_shape(mut self, shape: NodeShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn with_child(mut self, child: MindmapNode) -> Self {
        self.children.push(child);
        self
    }

    pub fn with_children(mut self, children: Vec<MindmapNode>) -> Self {
        self.children = children;
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    pub fn to_mermaid(&self, indent: usize) -> String {
        let mut output = String::new();
        let spaces = "    ".repeat(indent);

        // Node with shape
        let node_text = self.shape.wrap(&self.text);
        output.push_str(&format!("{}{}\n", spaces, node_text));

        // Icon if present
        if let Some(icon) = &self.icon {
            output.push_str(&format!("{}::icon({})\n", spaces, icon));
        }

        // Class if present
        if let Some(class) = &self.class {
            output.push_str(&format!("{}::::{}\n", spaces, class));
        }

        // Children
        for child in &self.children {
            output.push_str(&child.to_mermaid(indent + 1));
        }

        output
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeShape {
    #[default]
    Default, // plain text
    Square,  // [text]
    Rounded, // (text)
    Circle,  // ((text))
    Bang,    // ))text((
    Cloud,   // )text(
    Hexagon, // {{text}}
}

impl NodeShape {
    pub fn wrap(&self, text: &str) -> String {
        match self {
            Self::Default => text.to_string(),
            Self::Square => format!("[{}]", text),
            Self::Rounded => format!("({})", text),
            Self::Circle => format!("(({}))", text),
            Self::Bang => format!(")){}((", text),
            Self::Cloud => format!("){}(", text),
            Self::Hexagon => format!("{{{{{}}}}}", text),
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "default" | "plain" => Some(Self::Default),
            "square" | "rect" => Some(Self::Square),
            "rounded" => Some(Self::Rounded),
            "circle" => Some(Self::Circle),
            "bang" | "explosion" => Some(Self::Bang),
            "cloud" => Some(Self::Cloud),
            "hexagon" => Some(Self::Hexagon),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_basic() {
        let node = MindmapNode::new("Root");
        let mermaid = node.to_mermaid(0);
        assert!(mermaid.contains("Root"));
    }

    #[test]
    fn node_with_shape() {
        let node = MindmapNode::new("Test").with_shape(NodeShape::Square);
        let mermaid = node.to_mermaid(0);
        assert!(mermaid.contains("[Test]"));
    }

    #[test]
    fn node_with_children() {
        let node = MindmapNode::new("Root")
            .with_child(MindmapNode::new("Child1"))
            .with_child(MindmapNode::new("Child2"));
        let mermaid = node.to_mermaid(0);
        assert!(mermaid.contains("Root"));
        assert!(mermaid.contains("Child1"));
        assert!(mermaid.contains("Child2"));
    }

    #[test]
    fn node_shape_parse() {
        assert_eq!(NodeShape::parse("square"), Some(NodeShape::Square));
        assert_eq!(NodeShape::parse("circle"), Some(NodeShape::Circle));
        assert_eq!(NodeShape::parse("invalid"), None);
    }
}
