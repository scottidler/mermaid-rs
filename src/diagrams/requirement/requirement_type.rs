use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Risk {
    #[default]
    Low,
    Medium,
    High,
}

impl std::fmt::Display for Risk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Low => write!(f, "Low"),
            Self::Medium => write!(f, "Medium"),
            Self::High => write!(f, "High"),
        }
    }
}

impl Risk {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "low" => Some(Self::Low),
            "medium" | "med" => Some(Self::Medium),
            "high" => Some(Self::High),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VerifyMethod {
    #[default]
    Test,
    Inspection,
    Analysis,
    Demonstration,
}

impl std::fmt::Display for VerifyMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Test => write!(f, "Test"),
            Self::Inspection => write!(f, "Inspection"),
            Self::Analysis => write!(f, "Analysis"),
            Self::Demonstration => write!(f, "Demonstration"),
        }
    }
}

impl VerifyMethod {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "test" => Some(Self::Test),
            "inspection" | "inspect" => Some(Self::Inspection),
            "analysis" | "analyze" => Some(Self::Analysis),
            "demonstration" | "demo" => Some(Self::Demonstration),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub from: String,
    pub to: String,
    #[serde(rename = "type")]
    pub rel_type: RelationshipType,
}

impl Relationship {
    pub fn new(from: impl Into<String>, to: impl Into<String>, rel_type: RelationshipType) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            rel_type,
        }
    }

    pub fn contains(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self::new(source, target, RelationshipType::Contains)
    }

    pub fn copies(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self::new(source, target, RelationshipType::Copies)
    }

    pub fn derives(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self::new(source, target, RelationshipType::Derives)
    }

    pub fn satisfies(element: impl Into<String>, requirement: impl Into<String>) -> Self {
        Self::new(element, requirement, RelationshipType::Satisfies)
    }

    pub fn verifies(element: impl Into<String>, requirement: impl Into<String>) -> Self {
        Self::new(element, requirement, RelationshipType::Verifies)
    }

    pub fn refines(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self::new(source, target, RelationshipType::Refines)
    }

    pub fn traces(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self::new(source, target, RelationshipType::Traces)
    }

    pub fn to_mermaid(&self) -> String {
        let arrow = self.rel_type.arrow();
        format!("    {} {} {}\n", self.from, arrow, self.to)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RelationshipType {
    #[default]
    Contains,
    Copies,
    Derives,
    Satisfies,
    Verifies,
    Refines,
    Traces,
}

impl RelationshipType {
    pub fn arrow(&self) -> &'static str {
        match self {
            Self::Contains => "- contains ->",
            Self::Copies => "- copies ->",
            Self::Derives => "- derives ->",
            Self::Satisfies => "- satisfies ->",
            Self::Verifies => "- verifies ->",
            Self::Refines => "- refines ->",
            Self::Traces => "- traces ->",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "contains" => Some(Self::Contains),
            "copies" => Some(Self::Copies),
            "derives" => Some(Self::Derives),
            "satisfies" => Some(Self::Satisfies),
            "verifies" => Some(Self::Verifies),
            "refines" => Some(Self::Refines),
            "traces" => Some(Self::Traces),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn risk_parse() {
        assert_eq!(Risk::parse("low"), Some(Risk::Low));
        assert_eq!(Risk::parse("high"), Some(Risk::High));
        assert_eq!(Risk::parse("invalid"), None);
    }

    #[test]
    fn verify_method_parse() {
        assert_eq!(VerifyMethod::parse("test"), Some(VerifyMethod::Test));
        assert_eq!(
            VerifyMethod::parse("inspection"),
            Some(VerifyMethod::Inspection)
        );
        assert_eq!(VerifyMethod::parse("invalid"), None);
    }

    #[test]
    fn relationship_type_parse() {
        assert_eq!(
            RelationshipType::parse("contains"),
            Some(RelationshipType::Contains)
        );
        assert_eq!(
            RelationshipType::parse("verifies"),
            Some(RelationshipType::Verifies)
        );
        assert_eq!(RelationshipType::parse("invalid"), None);
    }
}
