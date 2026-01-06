mod common;

use mermaid_rs::core::Diagram;
use mermaid_rs::diagrams::requirement::{
    Element, ElementType, ReqRelationship, Requirement, RequirementDiagram, Risk, VerifyMethod,
};

#[test]
fn requirement_diagram_empty() {
    let diagram = RequirementDiagram::builder().build();
    let mermaid = diagram.to_mermaid();
    assert!(mermaid.starts_with("requirementDiagram"));
}

#[test]
fn requirement_diagram_with_requirement() {
    let diagram = RequirementDiagram::builder()
        .requirement_simple("REQ-001", "Login", Some("User must login"))
        .build();
    let mermaid = diagram.to_mermaid();
    assert!(mermaid.contains("requirement Login"));
    assert!(mermaid.contains("id: REQ-001"));
    assert!(mermaid.contains("text: User must login"));
}

#[test]
fn requirement_diagram_with_full_requirement() {
    let diagram = RequirementDiagram::builder()
        .requirement_full(
            "REQ-002",
            "Security",
            Some("System must be secure"),
            Risk::High,
            VerifyMethod::Test,
        )
        .build();
    let mermaid = diagram.to_mermaid();
    assert!(mermaid.contains("requirement Security"));
    assert!(mermaid.contains("id: REQ-002"));
    assert!(mermaid.contains("risk: High"));
    assert!(mermaid.contains("verifymethod: Test"));
}

#[test]
fn requirement_diagram_with_element() {
    let diagram = RequirementDiagram::builder()
        .element_simple("LoginUI", "Login Interface")
        .build();
    let mermaid = diagram.to_mermaid();
    assert!(mermaid.contains("element Login Interface"));
}

#[test]
fn requirement_diagram_with_element_type() {
    let diagram = RequirementDiagram::builder()
        .element(Element::new("TestSuite", "Test Suite").with_type(ElementType::TestCase))
        .build();
    let mermaid = diagram.to_mermaid();
    assert!(mermaid.contains("testCase Test Suite"));
}

#[test]
fn requirement_diagram_with_relationship() {
    let diagram = RequirementDiagram::builder()
        .requirement_simple("REQ-001", "Login", None)
        .element_simple("LoginUI", "Login Interface")
        .satisfies("LoginUI", "Login")
        .build();
    let mermaid = diagram.to_mermaid();
    assert!(mermaid.contains("LoginUI - satisfies -> Login"));
}

#[test]
fn requirement_diagram_relationship_types() {
    let relationships = vec![
        (ReqRelationship::contains("A", "B"), "- contains ->"),
        (ReqRelationship::copies("A", "B"), "- copies ->"),
        (ReqRelationship::derives("A", "B"), "- derives ->"),
        (ReqRelationship::satisfies("A", "B"), "- satisfies ->"),
        (ReqRelationship::verifies("A", "B"), "- verifies ->"),
        (ReqRelationship::refines("A", "B"), "- refines ->"),
        (ReqRelationship::traces("A", "B"), "- traces ->"),
    ];

    for (rel, expected) in relationships {
        let mermaid = rel.to_mermaid();
        assert!(
            mermaid.contains(expected),
            "Relationship should contain {}",
            expected
        );
    }
}

#[test]
fn requirement_diagram_risk_levels() {
    let risks = vec![
        (Risk::Low, "Low"),
        (Risk::Medium, "Medium"),
        (Risk::High, "High"),
    ];

    for (risk, expected) in risks {
        let req = Requirement::new("ID", "Name").with_risk(risk);
        let mermaid = req.to_mermaid();
        assert!(mermaid.contains(&format!("risk: {}", expected)));
    }
}

#[test]
fn requirement_diagram_verify_methods() {
    let methods = vec![
        (VerifyMethod::Test, "Test"),
        (VerifyMethod::Inspection, "Inspection"),
        (VerifyMethod::Analysis, "Analysis"),
        (VerifyMethod::Demonstration, "Demonstration"),
    ];

    for (method, expected) in methods {
        let req = Requirement::new("ID", "Name").with_verify_method(method);
        let mermaid = req.to_mermaid();
        assert!(mermaid.contains(&format!("verifymethod: {}", expected)));
    }
}

#[test]
fn requirement_diagram_from_json() {
    let json = r#"{
        "title": "System Requirements",
        "requirements": [
            {"id": "REQ-001", "name": "Login", "text": "Must have login"}
        ],
        "elements": [
            {"id": "UI001", "name": "Login Form"}
        ],
        "relationships": [
            {"from": "UI001", "to": "Login", "type": "satisfies"}
        ]
    }"#;

    let diagram = RequirementDiagram::from_json(json).unwrap();
    let mermaid = diagram.to_mermaid();
    assert!(mermaid.contains("requirement Login"));
    assert!(mermaid.contains("element Login Form"));
}

#[test]
fn requirement_diagram_from_yaml() {
    let yaml = r#"
title: Software Requirements
requirements:
  - id: REQ-001
    name: Authentication
    text: System must authenticate users
    risk: high
    verify_method: test
  - id: REQ-002
    name: Performance
    text: Response time under 1 second
    risk: medium
    verify_method: analysis
elements:
  - id: AUTH
    name: Auth Module
relationships:
  - from: AUTH
    to: Authentication
    type: satisfies
"#;

    let diagram = RequirementDiagram::from_yaml(yaml).unwrap();
    let mermaid = diagram.to_mermaid();
    assert!(mermaid.contains("requirement Authentication"));
    assert!(mermaid.contains("requirement Performance"));
    assert!(mermaid.contains("element Auth Module"));
}

#[test]
fn requirement_diagram_from_toml() {
    let toml = r#"
title = "Basic Requirements"

[[requirements]]
id = "REQ-001"
name = "Feature"
text = "Must have feature"

[[elements]]
id = "E001"
name = "Implementation"

[[relationships]]
from = "E001"
to = "Feature"
type = "satisfies"
"#;

    let diagram = RequirementDiagram::from_toml(toml).unwrap();
    let mermaid = diagram.to_mermaid();
    assert!(mermaid.contains("requirement Feature"));
    assert!(mermaid.contains("element Implementation"));
}

#[test]
fn requirement_diagram_type() {
    let diagram = RequirementDiagram::builder().build();
    assert_eq!(diagram.diagram_type(), "requirementDiagram");
}

#[test]
fn requirement_diagram_build_script() {
    let diagram = RequirementDiagram::builder()
        .title("My Requirements")
        .requirement_simple("REQ-001", "Test", None)
        .build();

    let script = diagram.build_script();
    assert!(script.contains("requirementDiagram"));
}

#[test]
fn requirement_with_text() {
    let req = Requirement::new("REQ-001", "Test").with_text("Test requirement description");
    let mermaid = req.to_mermaid();
    assert!(mermaid.contains("text: Test requirement description"));
}

#[test]
fn element_with_doc_ref() {
    let elem = Element::new("E001", "Test Element").with_doc_ref("REF-001");
    let mermaid = elem.to_mermaid();
    assert!(mermaid.contains("docRef: REF-001"));
}

#[test]
fn requirement_diagram_verifies() {
    let diagram = RequirementDiagram::builder()
        .requirement_simple("REQ-001", "Security", None)
        .element_simple("TestSuite", "Security Tests")
        .verifies("TestSuite", "Security")
        .build();
    let mermaid = diagram.to_mermaid();
    assert!(mermaid.contains("TestSuite - verifies -> Security"));
}

#[test]
fn requirement_diagram_derives() {
    let diagram = RequirementDiagram::builder()
        .requirement_simple("REQ-001", "Parent", None)
        .requirement_simple("REQ-002", "Child", None)
        .derives("Child", "Parent")
        .build();
    let mermaid = diagram.to_mermaid();
    assert!(mermaid.contains("Child - derives -> Parent"));
}
