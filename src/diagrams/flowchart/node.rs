use serde::{Deserialize, Serialize};

use crate::core::Style;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub shape: NodeShape,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,
}

impl Node {
    pub fn new(id: impl Into<String>, label: impl Into<String>, shape: NodeShape) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            shape,
            style: None,
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    /// Renders the node in mermaid syntax
    pub fn to_mermaid(&self) -> String {
        format!("{}{}", self.id, self.shape.wrap(&self.label))
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
        assert_eq!(node.to_mermaid(), "A([\"Start\"])");
    }

    #[test]
    fn node_shape_parse() {
        assert_eq!(NodeShape::parse("rectangle"), Some(NodeShape::Rectangle));
        assert_eq!(NodeShape::parse("diamond"), Some(NodeShape::Rhombus));
        assert_eq!(NodeShape::parse("invalid"), None);
    }
}
