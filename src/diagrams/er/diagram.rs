use serde::{Deserialize, Serialize};

use crate::core::{Config, Diagram, MermaidError, Theme};

use super::{Attribute, AttributeKey, AttributeType, Entity, Relationship};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ERDiagram {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    pub entities: Vec<Entity>,
    #[serde(default)]
    pub relationships: Vec<Relationship>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Config>,
    #[serde(skip)]
    raw_mermaid: Option<String>,
}

impl ERDiagram {
    pub fn builder() -> ERDiagramBuilder {
        ERDiagramBuilder::new()
    }

    pub fn from_raw_mermaid(script: String) -> Self {
        Self {
            title: None,
            entities: Vec::new(),
            relationships: Vec::new(),
            config: None,
            raw_mermaid: Some(script),
        }
    }

    pub fn from_json(json: &str) -> Result<Self, MermaidError> {
        serde_json::from_str(json).map_err(|e| MermaidError::ParseError(e.to_string()))
    }

    pub fn from_yaml(yaml: &str) -> Result<Self, MermaidError> {
        serde_yaml::from_str(yaml).map_err(|e| MermaidError::ParseError(e.to_string()))
    }

    pub fn from_toml(toml: &str) -> Result<Self, MermaidError> {
        toml::from_str(toml).map_err(|e| MermaidError::ParseError(e.to_string()))
    }
}

impl Diagram for ERDiagram {
    fn diagram_type(&self) -> &'static str {
        "erDiagram"
    }

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn config(&self) -> Option<&Config> {
        self.config.as_ref()
    }

    fn to_mermaid(&self) -> String {
        if let Some(raw) = &self.raw_mermaid {
            return raw.clone();
        }

        let mut output = String::from("erDiagram");

        // Entities
        for entity in &self.entities {
            output.push_str(&format!("\n\t{}", entity.to_mermaid()));
        }

        // Relationships
        for rel in &self.relationships {
            output.push_str(&format!("\n\t{}", rel.to_mermaid()));
        }

        output.push('\n');
        output
    }
}

#[derive(Debug, Default)]
pub struct ERDiagramBuilder {
    title: Option<String>,
    entities: Vec<Entity>,
    relationships: Vec<Relationship>,
    config: Option<Config>,
}

impl ERDiagramBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn entity(mut self, entity: Entity) -> Self {
        self.entities.push(entity);
        self
    }

    pub fn entity_simple(mut self, name: impl Into<String>) -> Self {
        self.entities.push(Entity::new(name));
        self
    }

    pub fn entity_with_attrs(
        mut self,
        name: impl Into<String>,
        attrs: Vec<(AttributeType, &str, Option<AttributeKey>)>,
    ) -> Self {
        let mut entity = Entity::new(name);
        for (attr_type, attr_name, key) in attrs {
            let mut attr = Attribute::new(attr_type, attr_name);
            if let Some(k) = key {
                attr = attr.with_key(k);
            }
            entity = entity.with_attribute(attr);
        }
        self.entities.push(entity);
        self
    }

    pub fn relationship(mut self, rel: Relationship) -> Self {
        self.relationships.push(rel);
        self
    }

    pub fn one_to_many(
        mut self,
        from: impl Into<String>,
        to: impl Into<String>,
        label: Option<&str>,
    ) -> Self {
        let mut rel = Relationship::one_to_many(from, to);
        if let Some(l) = label {
            rel = rel.with_label(l);
        }
        self.relationships.push(rel);
        self
    }

    pub fn many_to_one(
        mut self,
        from: impl Into<String>,
        to: impl Into<String>,
        label: Option<&str>,
    ) -> Self {
        let mut rel = Relationship::many_to_one(from, to);
        if let Some(l) = label {
            rel = rel.with_label(l);
        }
        self.relationships.push(rel);
        self
    }

    pub fn many_to_many(
        mut self,
        from: impl Into<String>,
        to: impl Into<String>,
        label: Option<&str>,
    ) -> Self {
        let mut rel = Relationship::many_to_many(from, to);
        if let Some(l) = label {
            rel = rel.with_label(l);
        }
        self.relationships.push(rel);
        self
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        let config = self.config.get_or_insert_with(Config::default);
        config.theme = theme;
        self
    }

    pub fn build(self) -> ERDiagram {
        ERDiagram {
            title: self.title,
            entities: self.entities,
            relationships: self.relationships,
            config: self.config,
            raw_mermaid: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn er_diagram_basic() {
        let diagram = ERDiagram::builder()
            .entity_simple("User")
            .entity_simple("Order")
            .one_to_many("User", "Order", Some("places"))
            .build();

        let mermaid = diagram.to_mermaid();
        assert!(mermaid.contains("erDiagram"));
        assert!(mermaid.contains("User"));
        assert!(mermaid.contains("Order"));
    }

    #[test]
    fn er_diagram_from_json() {
        let json = r#"{
            "title": "Test ER",
            "entities": [
                {"name": "User", "attributes": []},
                {"name": "Order", "attributes": []}
            ]
        }"#;

        let diagram = ERDiagram::from_json(json).unwrap();
        assert_eq!(diagram.entities.len(), 2);
    }

    #[test]
    fn er_diagram_from_yaml() {
        let yaml = r#"
title: Test ER
entities:
  - name: User
  - name: Order
"#;

        let diagram = ERDiagram::from_yaml(yaml).unwrap();
        assert_eq!(diagram.entities.len(), 2);
    }

    #[test]
    fn er_diagram_raw_mermaid() {
        let raw = "erDiagram\n    User ||--o{ Order : places";
        let diagram = ERDiagram::from_raw_mermaid(raw.to_string());
        assert_eq!(diagram.to_mermaid(), raw);
    }
}
