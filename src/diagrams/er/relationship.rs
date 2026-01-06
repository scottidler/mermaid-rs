use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub from: String,
    pub to: String,
    pub from_cardinality: Cardinality,
    pub to_cardinality: Cardinality,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// mermaid-py: dotted=False by default, meaning identifying (solid --) is the default
    #[serde(default = "default_identifying")]
    pub identifying: bool,
}

fn default_identifying() -> bool {
    true
}

impl Relationship {
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            from_cardinality: Cardinality::ExactlyOne,
            to_cardinality: Cardinality::ExactlyOne,
            label: None,
            identifying: true, // mermaid-py default
        }
    }

    pub fn one_to_many(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            from_cardinality: Cardinality::ExactlyOne,
            to_cardinality: Cardinality::ZeroOrMore,
            label: None,
            identifying: true, // mermaid-py default
        }
    }

    pub fn many_to_one(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            from_cardinality: Cardinality::ZeroOrMore,
            to_cardinality: Cardinality::ExactlyOne,
            label: None,
            identifying: true, // mermaid-py default
        }
    }

    pub fn many_to_many(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            from_cardinality: Cardinality::ZeroOrMore,
            to_cardinality: Cardinality::ZeroOrMore,
            label: None,
            identifying: true, // mermaid-py default
        }
    }

    pub fn with_cardinality(mut self, from: Cardinality, to: Cardinality) -> Self {
        self.from_cardinality = from;
        self.to_cardinality = to;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn identifying(mut self) -> Self {
        self.identifying = true;
        self
    }

    pub fn to_mermaid(&self) -> String {
        let from_sym = self.from_cardinality.symbol_left();
        let to_sym = self.to_cardinality.symbol_right();
        // mermaid-py: identifying = solid (--), non-identifying = dotted (..)
        let line = if self.identifying { "--" } else { ".." };

        // Format: {from}{left_sym}{line}{right_sym}{to} : "{label}"
        match &self.label {
            Some(label) => format!("{}{}{}{}{} : \"{}\"", self.from, from_sym, line, to_sym, self.to, label),
            None => format!("{}{}{}{}{}", self.from, from_sym, line, to_sym, self.to),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Cardinality {
    #[default]
    ExactlyOne, // ||
    ZeroOrOne,  // |o
    ZeroOrMore, // }o
    OneOrMore,  // }|
}

impl Cardinality {
    pub fn symbol_left(&self) -> &'static str {
        match self {
            Self::ExactlyOne => "||",
            Self::ZeroOrOne => "|o",
            Self::ZeroOrMore => "}o",
            Self::OneOrMore => "}|",
        }
    }

    pub fn symbol_right(&self) -> &'static str {
        match self {
            Self::ExactlyOne => "||",
            Self::ZeroOrOne => "o|",
            Self::ZeroOrMore => "o{",
            Self::OneOrMore => "|{",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "exactly-one" | "one" | "||" => Some(Self::ExactlyOne),
            "zero-or-one" | "optional" | "|o" | "o|" => Some(Self::ZeroOrOne),
            "zero-or-more" | "many" | "}o" | "o{" => Some(Self::ZeroOrMore),
            "one-or-more" | "}|" | "|{" => Some(Self::OneOrMore),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relationship_basic() {
        let rel = Relationship::new("User", "Order");
        let mermaid = rel.to_mermaid();
        assert!(mermaid.contains("User"));
        assert!(mermaid.contains("Order"));
    }

    #[test]
    fn relationship_one_to_many() {
        let rel = Relationship::one_to_many("User", "Order");
        let mermaid = rel.to_mermaid();
        assert!(mermaid.contains("||"));
        assert!(mermaid.contains("o{"));
    }

    #[test]
    fn cardinality_parse() {
        assert_eq!(Cardinality::parse("one"), Some(Cardinality::ExactlyOne));
        assert_eq!(Cardinality::parse("many"), Some(Cardinality::ZeroOrMore));
        assert_eq!(Cardinality::parse("invalid"), None);
    }
}
