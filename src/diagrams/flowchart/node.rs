use serde::{Deserialize, Serialize};

use crate::core::{normalize_id, Style};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub shape: NodeShape,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,
    /// Optional hyperlink for clickable nodes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// How the hyperlink opens (default: Blank for new tab)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href_type: Option<HrefType>,
}

impl Node {
    pub fn new(id: impl Into<String>, label: impl Into<String>, shape: NodeShape) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            shape,
            style: None,
            href: None,
            href_type: None,
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    /// Add a clickable hyperlink to this node
    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    /// Set how the hyperlink opens (default: Blank)
    pub fn with_href_type(mut self, href_type: HrefType) -> Self {
        self.href_type = Some(href_type);
        self
    }

    /// Renders the node in mermaid syntax
    pub fn to_mermaid(&self) -> String {
        // Normalize ID to match mermaid-py's text_to_snake_case()
        let normalized_id = normalize_id(&self.id);
        let mut output = format!("{}{}", normalized_id, self.shape.wrap(&self.label));

        // Add click directive if href is set
        if let Some(href) = &self.href {
            let href_type = self.href_type.unwrap_or_default();
            output.push_str(&format!(
                "\n    click {} \"{}\" {}",
                normalized_id,
                href,
                href_type.as_str()
            ));
        }

        output
    }
}

/// How a hyperlink should open when clicked
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HrefType {
    /// Open in new tab/window (_blank)
    #[default]
    Blank,
    /// Open in same frame (_self)
    #[serde(rename = "self")]
    Self_,
    /// Open in parent frame (_parent)
    Parent,
    /// Open in full window (_top)
    Top,
}

impl HrefType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Blank => "_blank",
            Self::Self_ => "_self",
            Self::Parent => "_parent",
            Self::Top => "_top",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "blank" | "_blank" => Some(Self::Blank),
            "self" | "_self" => Some(Self::Self_),
            "parent" | "_parent" => Some(Self::Parent),
            "top" | "_top" => Some(Self::Top),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NodeShape {
    #[default]
    Rectangle, // [label]
    Rounded,          // (label)
    Stadium,          // ([label])
    Subroutine,       // [[label]]
    Cylinder,         // [(label)]
    Circle,           // ((label))
    Asymmetric,       // >label]
    Rhombus,          // {label}
    Hexagon,          // {{label}}
    Parallelogram,    // [/label/]
    ParallelogramAlt, // [\label\]
    Trapezoid,        // [/label\]
    TrapezoidAlt,     // [\label/]
    DoubleCircle,     // (((label)))
}

impl NodeShape {
    pub fn wrap(&self, label: &str) -> String {
        match self {
            Self::Rectangle => format!("[\"{}\"]", label),
            Self::Rounded => format!("(\"{}\")", label),
            Self::Stadium => format!("([\"{}\"])", label),
            Self::Subroutine => format!("[[\"{}\"]]", label),
            Self::Cylinder => format!("[(\"{}\")]", label),
            Self::Circle => format!("((\"{}\")", label),
            Self::Asymmetric => format!(">\"{}\"]", label),
            Self::Rhombus => format!("{}{}{}{}{}", "{", "\"", label, "\"", "}"),
            Self::Hexagon => format!("{}{}{}{}{}", "{{", "\"", label, "\"", "}}"),
            Self::Parallelogram => format!("[/\"{}\"/]", label),
            Self::ParallelogramAlt => format!("[\\\"{}\"\\]", label),
            Self::Trapezoid => format!("[/\"{}\"\\]", label),
            Self::TrapezoidAlt => format!("[\\\"{}\"//]", label),
            Self::DoubleCircle => format!("(((\"{}\")))", label),
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "rectangle" | "rect" => Some(Self::Rectangle),
            "rounded" | "round" => Some(Self::Rounded),
            "stadium" => Some(Self::Stadium),
            "subroutine" => Some(Self::Subroutine),
            "cylinder" | "db" | "database" => Some(Self::Cylinder),
            "circle" => Some(Self::Circle),
            "asymmetric" | "flag" => Some(Self::Asymmetric),
            "rhombus" | "diamond" | "decision" => Some(Self::Rhombus),
            "hexagon" | "hex" => Some(Self::Hexagon),
            "parallelogram" | "para" => Some(Self::Parallelogram),
            "parallelogram-alt" | "para-alt" => Some(Self::ParallelogramAlt),
            "trapezoid" | "trap" => Some(Self::Trapezoid),
            "trapezoid-alt" | "trap-alt" => Some(Self::TrapezoidAlt),
            "double-circle" | "doublecircle" => Some(Self::DoubleCircle),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_shapes_wrap_correctly() {
        assert_eq!(NodeShape::Rectangle.wrap("Test"), "[\"Test\"]");
        assert_eq!(NodeShape::Rounded.wrap("Test"), "(\"Test\")");
        assert_eq!(NodeShape::Stadium.wrap("Test"), "([\"Test\"])");
        assert_eq!(NodeShape::Rhombus.wrap("Test"), "{\"Test\"}");
        assert_eq!(NodeShape::DoubleCircle.wrap("Test"), "(((\"Test\")))");
    }

    #[test]
    fn node_to_mermaid() {
        let node = Node::new("A", "Start", NodeShape::Stadium);
        // normalize_id lowercases and converts spaces to underscores
        assert_eq!(node.to_mermaid(), "a([\"Start\"])");
    }

    #[test]
    fn node_to_mermaid_with_spaces() {
        let node = Node::new("First Node", "Start Here", NodeShape::Rectangle);
        // Spaces should be converted to underscores
        assert_eq!(node.to_mermaid(), "first_node[\"Start Here\"]");
    }

    #[test]
    fn node_shape_parse() {
        assert_eq!(NodeShape::parse("rectangle"), Some(NodeShape::Rectangle));
        assert_eq!(NodeShape::parse("diamond"), Some(NodeShape::Rhombus));
        assert_eq!(NodeShape::parse("invalid"), None);
    }

    #[test]
    fn node_with_href() {
        let node =
            Node::new("github", "GitHub", NodeShape::Rectangle).with_href("https://github.com");
        let mermaid = node.to_mermaid();
        assert!(mermaid.contains("github[\"GitHub\"]"));
        assert!(mermaid.contains("click github \"https://github.com\" _blank"));
    }

    #[test]
    fn node_with_href_and_type() {
        let node = Node::new("docs", "Documentation", NodeShape::Rectangle)
            .with_href("https://docs.example.com")
            .with_href_type(HrefType::Self_);
        let mermaid = node.to_mermaid();
        assert!(mermaid.contains("click docs \"https://docs.example.com\" _self"));
    }

    #[test]
    fn href_type_parse() {
        assert_eq!(HrefType::parse("blank"), Some(HrefType::Blank));
        assert_eq!(HrefType::parse("_blank"), Some(HrefType::Blank));
        assert_eq!(HrefType::parse("self"), Some(HrefType::Self_));
        assert_eq!(HrefType::parse("_self"), Some(HrefType::Self_));
        assert_eq!(HrefType::parse("parent"), Some(HrefType::Parent));
        assert_eq!(HrefType::parse("top"), Some(HrefType::Top));
        assert_eq!(HrefType::parse("invalid"), None);
    }
}
