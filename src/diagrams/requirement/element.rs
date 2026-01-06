use serde::{Deserialize, Serialize};

use super::{Risk, VerifyMethod};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requirement {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default)]
    pub risk: Risk,
    #[serde(default)]
    pub verify_method: VerifyMethod,
    #[serde(default, rename = "type")]
    pub req_type: RequirementType,
}

impl Requirement {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            text: None,
            risk: Risk::default(),
            verify_method: VerifyMethod::default(),
            req_type: RequirementType::default(),
        }
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn with_risk(mut self, risk: Risk) -> Self {
        self.risk = risk;
        self
    }

    pub fn with_verify_method(mut self, method: VerifyMethod) -> Self {
        self.verify_method = method;
        self
    }

    pub fn with_type(mut self, req_type: RequirementType) -> Self {
        self.req_type = req_type;
        self
    }

    pub fn to_mermaid(&self) -> String {
        let type_str = self.req_type.to_string();
        let mut output = format!("    {} {} {{\n", type_str, self.name);
        output.push_str(&format!("        id: {}\n", self.id));
        if let Some(text) = &self.text {
            output.push_str(&format!("        text: {}\n", text));
        }
        output.push_str(&format!("        risk: {}\n", self.risk));
        output.push_str(&format!("        verifymethod: {}\n", self.verify_method));
        output.push_str("    }\n");
        output
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RequirementType {
    #[default]
    Requirement,
    FunctionalRequirement,
    InterfaceRequirement,
    PerformanceRequirement,
    PhysicalRequirement,
    DesignConstraint,
}

impl std::fmt::Display for RequirementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Requirement => write!(f, "requirement"),
            Self::FunctionalRequirement => write!(f, "functionalRequirement"),
            Self::InterfaceRequirement => write!(f, "interfaceRequirement"),
            Self::PerformanceRequirement => write!(f, "performanceRequirement"),
            Self::PhysicalRequirement => write!(f, "physicalRequirement"),
            Self::DesignConstraint => write!(f, "designConstraint"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element {
    pub id: String,
    pub name: String,
    #[serde(default, rename = "type")]
    pub element_type: ElementType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_ref: Option<String>,
}

impl Element {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            element_type: ElementType::default(),
            doc_ref: None,
        }
    }

    pub fn with_type(mut self, element_type: ElementType) -> Self {
        self.element_type = element_type;
        self
    }

    pub fn with_doc_ref(mut self, doc_ref: impl Into<String>) -> Self {
        self.doc_ref = Some(doc_ref.into());
        self
    }

    pub fn to_mermaid(&self) -> String {
        let type_str = self.element_type.to_string();
        let mut output = format!("    {} {} {{\n", type_str, self.name);
        output.push_str(&format!("        type: {}\n", self.id));
        if let Some(doc_ref) = &self.doc_ref {
            output.push_str(&format!("        docRef: {}\n", doc_ref));
        }
        output.push_str("    }\n");
        output
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ElementType {
    #[default]
    Element,
    Simulation,
    TestCase,
}

impl std::fmt::Display for ElementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Element => write!(f, "element"),
            Self::Simulation => write!(f, "simulation"),
            Self::TestCase => write!(f, "testCase"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requirement_basic() {
        let req = Requirement::new("REQ-001", "Login")
            .with_text("User must be able to login")
            .with_risk(Risk::Medium)
            .with_verify_method(VerifyMethod::Test);

        let mermaid = req.to_mermaid();
        assert!(mermaid.contains("requirement Login"));
        assert!(mermaid.contains("id: REQ-001"));
        assert!(mermaid.contains("risk: Medium"));
    }

    #[test]
    fn element_basic() {
        let elem = Element::new("LoginModule", "Login Module").with_doc_ref("docs/login.md");

        let mermaid = elem.to_mermaid();
        assert!(mermaid.contains("element Login Module"));
        assert!(mermaid.contains("docRef: docs/login.md"));
    }
}
