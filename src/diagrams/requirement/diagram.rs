use serde::{Deserialize, Serialize};

use crate::core::{Config, Diagram, MermaidError};

use super::{Element, ReqRelationship, Requirement, Risk, VerifyMethod};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequirementDiagram {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    pub requirements: Vec<Requirement>,
    #[serde(default)]
    pub elements: Vec<Element>,
    #[serde(default)]
    pub relationships: Vec<ReqRelationship>,
    #[serde(skip)]
    raw_mermaid: Option<String>,
}

impl RequirementDiagram {
    pub fn builder() -> RequirementDiagramBuilder {
        RequirementDiagramBuilder::new()
    }

    pub fn from_raw_mermaid(script: String) -> Self {
        Self {
            title: None,
            requirements: Vec::new(),
            elements: Vec::new(),
            relationships: Vec::new(),
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

impl Diagram for RequirementDiagram {
    fn diagram_type(&self) -> &'static str {
        "requirementDiagram"
    }

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn config(&self) -> Option<&Config> {
        None
    }

    fn to_mermaid(&self) -> String {
        if let Some(raw) = &self.raw_mermaid {
            return raw.clone();
        }

        let mut output = String::from("requirementDiagram\n");

        // Requirements
        for req in &self.requirements {
            output.push_str(&req.to_mermaid());
        }

        // Elements
        for elem in &self.elements {
            output.push_str(&elem.to_mermaid());
        }

        // Relationships
        for rel in &self.relationships {
            output.push_str(&rel.to_mermaid());
        }

        output
    }
}

#[derive(Debug, Default)]
pub struct RequirementDiagramBuilder {
    title: Option<String>,
    requirements: Vec<Requirement>,
    elements: Vec<Element>,
    relationships: Vec<ReqRelationship>,
}

impl RequirementDiagramBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn requirement(mut self, req: Requirement) -> Self {
        self.requirements.push(req);
        self
    }

    pub fn requirement_simple(
        mut self,
        id: impl Into<String>,
        name: impl Into<String>,
        text: Option<&str>,
    ) -> Self {
        let mut req = Requirement::new(id, name);
        if let Some(t) = text {
            req = req.with_text(t);
        }
        self.requirements.push(req);
        self
    }

    pub fn requirement_full(
        mut self,
        id: impl Into<String>,
        name: impl Into<String>,
        text: Option<&str>,
        risk: Risk,
        verify: VerifyMethod,
    ) -> Self {
        let mut req = Requirement::new(id, name)
            .with_risk(risk)
            .with_verify_method(verify);
        if let Some(t) = text {
            req = req.with_text(t);
        }
        self.requirements.push(req);
        self
    }

    pub fn element(mut self, elem: Element) -> Self {
        self.elements.push(elem);
        self
    }

    pub fn element_simple(mut self, id: impl Into<String>, name: impl Into<String>) -> Self {
        self.elements.push(Element::new(id, name));
        self
    }

    pub fn relationship(mut self, rel: ReqRelationship) -> Self {
        self.relationships.push(rel);
        self
    }

    pub fn satisfies(mut self, element: impl Into<String>, requirement: impl Into<String>) -> Self {
        self.relationships
            .push(ReqRelationship::satisfies(element, requirement));
        self
    }

    pub fn verifies(mut self, element: impl Into<String>, requirement: impl Into<String>) -> Self {
        self.relationships
            .push(ReqRelationship::verifies(element, requirement));
        self
    }

    pub fn derives(mut self, source: impl Into<String>, target: impl Into<String>) -> Self {
        self.relationships
            .push(ReqRelationship::derives(source, target));
        self
    }

    pub fn build(self) -> RequirementDiagram {
        RequirementDiagram {
            title: self.title,
            requirements: self.requirements,
            elements: self.elements,
            relationships: self.relationships,
            raw_mermaid: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requirement_diagram_basic() {
        let diagram = RequirementDiagram::builder()
            .requirement_simple("REQ-001", "Login", Some("User must login"))
            .element_simple("LoginUI", "Login Interface")
            .satisfies("LoginUI", "Login")
            .build();

        let mermaid = diagram.to_mermaid();
        assert!(mermaid.contains("requirementDiagram"));
        assert!(mermaid.contains("requirement Login"));
        assert!(mermaid.contains("element Login Interface"));
    }

    #[test]
    fn requirement_diagram_from_json() {
        let json = r#"{
            "title": "Test Requirements",
            "requirements": [
                {"id": "REQ-001", "name": "Test Req"}
            ],
            "elements": [
                {"id": "E1", "name": "Element 1"}
            ]
        }"#;

        let diagram = RequirementDiagram::from_json(json).unwrap();
        assert_eq!(diagram.requirements.len(), 1);
        assert_eq!(diagram.elements.len(), 1);
    }

    #[test]
    fn requirement_diagram_from_yaml() {
        let yaml = r#"
title: Test Requirements
requirements:
  - id: REQ-001
    name: Test Req
elements:
  - id: E1
    name: Element 1
"#;

        let diagram = RequirementDiagram::from_yaml(yaml).unwrap();
        assert_eq!(diagram.requirements.len(), 1);
    }

    #[test]
    fn requirement_diagram_raw_mermaid() {
        let raw = "requirementDiagram\n    requirement Test {\n        id: REQ-001\n    }";
        let diagram = RequirementDiagram::from_raw_mermaid(raw.to_string());
        assert_eq!(diagram.to_mermaid(), raw);
    }
}
