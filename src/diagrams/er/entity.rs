use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    #[serde(default)]
    pub attributes: Vec<Attribute>,
}

impl Entity {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            attributes: Vec::new(),
        }
    }

    pub fn with_attribute(mut self, attr: Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    pub fn with_attributes(mut self, attrs: Vec<Attribute>) -> Self {
        self.attributes = attrs;
        self
    }

    pub fn to_mermaid(&self) -> String {
        // mermaid-py always outputs braces, even for empty entities
        let mut output = format!("{}{{\n", self.name);
        for attr in &self.attributes {
            output.push_str(&format!("\t{}\n", attr.to_mermaid()));
        }
        output.push('}');
        output
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    #[serde(rename = "type")]
    pub attr_type: AttributeType,
    pub name: String,
    #[serde(default)]
    pub key: AttributeKey,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl Attribute {
    pub fn new(attr_type: AttributeType, name: impl Into<String>) -> Self {
        Self {
            attr_type,
            name: name.into(),
            key: AttributeKey::default(),
            comment: None,
        }
    }

    pub fn with_key(mut self, key: AttributeKey) -> Self {
        self.key = key;
        self
    }

    pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }

    pub fn to_mermaid(&self) -> String {
        let mut parts = vec![self.attr_type.to_string(), self.name.clone()];

        if self.key != AttributeKey::None {
            parts.push(self.key.to_string());
        }

        if let Some(comment) = &self.comment {
            parts.push(format!("\"{}\"", comment));
        }

        parts.join(" ")
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AttributeType {
    #[default]
    String,
    Int,
    Float,
    Boolean,
    Date,
    DateTime,
    Text,
    Uuid,
    Enum,
}

impl std::fmt::Display for AttributeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String => write!(f, "string"),
            Self::Int => write!(f, "int"),
            Self::Float => write!(f, "float"),
            Self::Boolean => write!(f, "boolean"),
            Self::Date => write!(f, "date"),
            Self::DateTime => write!(f, "datetime"),
            Self::Text => write!(f, "text"),
            Self::Uuid => write!(f, "uuid"),
            Self::Enum => write!(f, "enum"),
        }
    }
}

impl AttributeType {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "string" | "str" | "varchar" => Some(Self::String),
            "int" | "integer" | "bigint" => Some(Self::Int),
            "float" | "double" | "decimal" => Some(Self::Float),
            "boolean" | "bool" => Some(Self::Boolean),
            "date" => Some(Self::Date),
            "datetime" | "timestamp" => Some(Self::DateTime),
            "text" => Some(Self::Text),
            "uuid" => Some(Self::Uuid),
            "enum" => Some(Self::Enum),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AttributeKey {
    #[default]
    None,
    #[serde(rename = "PK")]
    PrimaryKey,
    #[serde(rename = "FK")]
    ForeignKey,
    #[serde(rename = "UK")]
    UniqueKey,
}

impl std::fmt::Display for AttributeKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, ""),
            Self::PrimaryKey => write!(f, "PK"),
            Self::ForeignKey => write!(f, "FK"),
            Self::UniqueKey => write!(f, "UK"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_basic() {
        let entity = Entity::new("User");
        let mermaid = entity.to_mermaid();
        assert!(mermaid.contains("User{"));
        assert!(mermaid.contains("}"));
    }

    #[test]
    fn entity_with_attributes() {
        let entity = Entity::new("User")
            .with_attribute(Attribute::new(AttributeType::Int, "id").with_key(AttributeKey::PrimaryKey))
            .with_attribute(Attribute::new(AttributeType::String, "name"));

        let mermaid = entity.to_mermaid();
        assert!(mermaid.contains("User{"));
        assert!(mermaid.contains("int id PK"));
        assert!(mermaid.contains("string name"));
    }

    #[test]
    fn attribute_type_parse() {
        assert_eq!(AttributeType::parse("string"), Some(AttributeType::String));
        assert_eq!(AttributeType::parse("int"), Some(AttributeType::Int));
        assert_eq!(AttributeType::parse("invalid"), None);
    }
}
