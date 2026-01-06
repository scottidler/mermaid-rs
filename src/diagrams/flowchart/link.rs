use serde::{Deserialize, Serialize};

use crate::core::normalize_id;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub style: LinkStyle,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default = "LinkHead::arrow")]
    pub head: LinkHead,
    #[serde(default)]
    pub tail: LinkHead,
}

impl Link {
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            style: LinkStyle::default(),
            label: None,
            head: LinkHead::Arrow,
            tail: LinkHead::None,
        }
    }

    pub fn with_style(mut self, style: LinkStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_head(mut self, head: LinkHead) -> Self {
        self.head = head;
        self
    }

    pub fn with_tail(mut self, tail: LinkHead) -> Self {
        self.tail = tail;
        self
    }

    /// Renders the link in mermaid syntax
    pub fn to_mermaid(&self) -> String {
        let arrow = self.style.arrow_syntax(&self.tail, &self.head);
        // Normalize IDs to match mermaid-py's text_to_snake_case()
        let from = normalize_id(&self.from);
        let to = normalize_id(&self.to);
        match &self.label {
            Some(label) => format!("{} {}|{}| {}", from, arrow, label, to),
            None => format!("{} {} {}", from, arrow, to),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LinkStyle {
    #[default]
    Arrow, // -->
    Dotted,    // -.->
    Thick,     // ==>
    Invisible, // ~~~
    Open,      // ---
}

impl LinkStyle {
    pub fn arrow_syntax(&self, tail: &LinkHead, head: &LinkHead) -> String {
        let tail_sym = tail.symbol_left();
        let head_sym = head.symbol_right();

        match self {
            Self::Arrow => format!("{}--{}", tail_sym, head_sym),
            Self::Dotted => format!("{}-.-{}", tail_sym, head_sym),
            Self::Thick => format!("{}=={}", tail_sym, head_sym),
            Self::Invisible => "~~~".to_string(),
            Self::Open => format!(
                "{}---{}",
                tail_sym.replace(['<', 'o', 'x'], ""),
                head_sym.replace(['>', 'o', 'x'], "")
            ),
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "arrow" | "solid" => Some(Self::Arrow),
            "dotted" | "dashed" => Some(Self::Dotted),
            "thick" | "bold" => Some(Self::Thick),
            "invisible" | "hidden" => Some(Self::Invisible),
            "open" | "line" => Some(Self::Open),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LinkHead {
    Arrow,  // >
    Circle, // o
    Cross,  // x
    #[default]
    None, // (no head)
}

impl LinkHead {
    /// Default for serde deserialization of head field
    pub fn arrow() -> Self {
        Self::Arrow
    }

    pub fn symbol_right(&self) -> &'static str {
        match self {
            Self::Arrow => ">",
            Self::Circle => "o",
            Self::Cross => "x",
            Self::None => "",
        }
    }

    pub fn symbol_left(&self) -> &'static str {
        match self {
            Self::Arrow => "<",
            Self::Circle => "o",
            Self::Cross => "x",
            Self::None => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_default_arrow() {
        let link = Link::new("A", "B");
        // normalize_id lowercases IDs
        assert_eq!(link.to_mermaid(), "a --> b");
    }

    #[test]
    fn link_with_spaces_in_ids() {
        let link = Link::new("First Node", "Second Node");
        // Spaces should be converted to underscores
        assert_eq!(link.to_mermaid(), "first_node --> second_node");
    }

    #[test]
    fn link_with_label() {
        let link = Link::new("A", "B").with_label("connects");
        assert_eq!(link.to_mermaid(), "a -->|connects| b");
    }

    #[test]
    fn link_dotted() {
        let link = Link::new("A", "B").with_style(LinkStyle::Dotted);
        assert_eq!(link.to_mermaid(), "a -.-> b");
    }

    #[test]
    fn link_thick() {
        let link = Link::new("A", "B").with_style(LinkStyle::Thick);
        assert_eq!(link.to_mermaid(), "a ==> b");
    }

    #[test]
    fn link_invisible() {
        let link = Link::new("A", "B").with_style(LinkStyle::Invisible);
        assert_eq!(link.to_mermaid(), "a ~~~ b");
    }

    #[test]
    fn link_style_parse() {
        assert_eq!(LinkStyle::parse("arrow"), Some(LinkStyle::Arrow));
        assert_eq!(LinkStyle::parse("dotted"), Some(LinkStyle::Dotted));
        assert_eq!(LinkStyle::parse("invalid"), None);
    }

    #[test]
    fn link_circle_head() {
        let link = Link::new("A", "B").with_head(LinkHead::Circle);
        assert_eq!(link.to_mermaid(), "a --o b");
    }

    #[test]
    fn link_cross_head() {
        let link = Link::new("A", "B").with_head(LinkHead::Cross);
        assert_eq!(link.to_mermaid(), "a --x b");
    }

    #[test]
    fn link_bidirectional() {
        let link = Link::new("A", "B")
            .with_head(LinkHead::Arrow)
            .with_tail(LinkHead::Arrow);
        assert_eq!(link.to_mermaid(), "a <--> b");
    }

    #[test]
    fn link_circle_to_circle() {
        let link = Link::new("A", "B")
            .with_head(LinkHead::Circle)
            .with_tail(LinkHead::Circle);
        assert_eq!(link.to_mermaid(), "a o--o b");
    }

    #[test]
    fn link_cross_to_cross() {
        let link = Link::new("A", "B")
            .with_head(LinkHead::Cross)
            .with_tail(LinkHead::Cross);
        assert_eq!(link.to_mermaid(), "a x--x b");
    }

    #[test]
    fn link_dotted_with_circle() {
        let link = Link::new("A", "B")
            .with_style(LinkStyle::Dotted)
            .with_head(LinkHead::Circle);
        assert_eq!(link.to_mermaid(), "a -.-o b");
    }

    #[test]
    fn link_thick_bidirectional() {
        let link = Link::new("A", "B")
            .with_style(LinkStyle::Thick)
            .with_head(LinkHead::Arrow)
            .with_tail(LinkHead::Arrow);
        assert_eq!(link.to_mermaid(), "a <==> b");
    }

    #[test]
    fn link_open_style() {
        let link = Link::new("A", "B").with_style(LinkStyle::Open);
        assert_eq!(link.to_mermaid(), "a --- b");
    }
}
