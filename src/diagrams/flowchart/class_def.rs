use serde::{Deserialize, Serialize};

use crate::core::Style;

/// Defines a CSS class that can be applied to nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassDef {
    pub name: String,
    #[serde(flatten)]
    pub style: Style,
}

impl ClassDef {
    pub fn new(name: impl Into<String>, style: Style) -> Self {
        Self {
            name: name.into(),
            style,
        }
    }

    /// Renders the classDef directive in mermaid syntax
    pub fn to_mermaid(&self) -> String {
        let css = self.style.to_css();
        format!("classDef {} {}", self.name, css)
    }
}

/// Assigns a class to one or more nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassAssignment {
    pub class_name: String,
    pub nodes: Vec<String>,
}

impl ClassAssignment {
    pub fn new(class_name: impl Into<String>, nodes: Vec<String>) -> Self {
        Self {
            class_name: class_name.into(),
            nodes,
        }
    }

    /// Renders the class assignment in mermaid syntax
    pub fn to_mermaid(&self) -> String {
        format!("class {} {}", self.nodes.join(","), self.class_name)
    }
}

/// Link styling for specific links by index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkStyleDef {
    /// Zero-based index of the link to style
    pub index: usize,
    #[serde(flatten)]
    pub style: Style,
}

impl LinkStyleDef {
    pub fn new(index: usize, style: Style) -> Self {
        Self { index, style }
    }

    /// Renders the linkStyle directive in mermaid syntax
    pub fn to_mermaid(&self) -> String {
        let css = self.style.to_css();
        format!("linkStyle {} {}", self.index, css)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_def_basic() {
        let class_def = ClassDef::new(
            "highlight",
            Style::builder().fill("#f9f").stroke("#333").build(),
        );
        assert_eq!(
            class_def.to_mermaid(),
            "classDef highlight fill:#f9f,stroke:#333"
        );
    }

    #[test]
    fn class_def_with_stroke_width() {
        let class_def = ClassDef::new(
            "thick",
            Style::builder()
                .fill("#bbf")
                .stroke("#333")
                .stroke_width("4px")
                .build(),
        );
        assert_eq!(
            class_def.to_mermaid(),
            "classDef thick fill:#bbf,stroke:#333,stroke-width:4px"
        );
    }

    #[test]
    fn class_assignment_single_node() {
        let assignment = ClassAssignment::new("highlight", vec!["A".to_string()]);
        assert_eq!(assignment.to_mermaid(), "class A highlight");
    }

    #[test]
    fn class_assignment_multiple_nodes() {
        let assignment = ClassAssignment::new(
            "highlight",
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
        );
        assert_eq!(assignment.to_mermaid(), "class A,B,C highlight");
    }

    #[test]
    fn link_style_def_basic() {
        let link_style = LinkStyleDef::new(
            0,
            Style::builder()
                .stroke("#ff0000")
                .stroke_width("2px")
                .build(),
        );
        assert_eq!(
            link_style.to_mermaid(),
            "linkStyle 0 stroke:#ff0000,stroke-width:2px"
        );
    }

    #[test]
    fn link_style_def_with_dasharray() {
        let link_style = LinkStyleDef::new(
            2,
            Style::builder()
                .stroke("#00f")
                .stroke_dasharray("5,5")
                .build(),
        );
        assert_eq!(
            link_style.to_mermaid(),
            "linkStyle 2 stroke:#00f,stroke-dasharray:5,5"
        );
    }
}
