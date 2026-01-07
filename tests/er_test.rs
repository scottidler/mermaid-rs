mod common;

use mermaid_rs::core::Diagram;
use mermaid_rs::diagrams::er::{
    Attribute, AttributeKey, AttributeType, Cardinality, ERDiagram, Entity,
};

#[test]
fn er_diagram_empty() {
    let diagram = ERDiagram::builder().build();
    let mermaid = diagram.to_mermaid();
    assert!(mermaid.starts_with("erDiagram"));
}

#[test]
fn er_diagram_with_entity() {
    let diagram = ERDiagram::builder().entity_simple("User").build();
    let mermaid = diagram.to_mermaid();
    assert!(mermaid.contains("User"));
}

#[test]
fn er_diagram_with_entity_attributes() {
    let diagram = ERDiagram::builder()
        .entity(
            Entity::new("User")
                .with_attribute(
                    Attribute::new(AttributeType::Int, "id").with_key(AttributeKey::PrimaryKey),
                )
                .with_attribute(Attribute::new(AttributeType::String, "name"))
                .with_attribute(Attribute::new(AttributeType::String, "email")),
        )
        .build();
    let mermaid = diagram.to_mermaid();
    assert!(mermaid.contains("User"));
    assert!(mermaid.contains("int id PK"));
    assert!(mermaid.contains("string name"));
    assert!(mermaid.contains("string email"));
}

#[test]
fn er_diagram_with_relationship() {
    let diagram = ERDiagram::builder()
        .entity_simple("User")
        .entity_simple("Order")
        .one_to_many("User", "Order", Some("places"))
        .build();
    let mermaid = diagram.to_mermaid();
    assert!(mermaid.contains("User"));
    assert!(mermaid.contains("Order"));
    assert!(mermaid.contains("places"));
}

#[test]
fn er_diagram_cardinality_symbols() {
    // Test cardinality symbols
    assert_eq!(Cardinality::ZeroOrOne.symbol_left(), "|o");
    assert_eq!(Cardinality::ExactlyOne.symbol_left(), "||");
    assert_eq!(Cardinality::ZeroOrMore.symbol_left(), "}o");
    assert_eq!(Cardinality::OneOrMore.symbol_left(), "}|");
}

#[test]
fn er_diagram_from_json() {
    let json = r#"{
        "title": "E-Commerce",
        "entities": [
            {"name": "Customer"},
            {"name": "Order"}
        ],
        "relationships": [
            {
                "from": "Customer",
                "from_cardinality": "exactly-one",
                "to": "Order",
                "to_cardinality": "zero-or-more"
            }
        ]
    }"#;

    let diagram = ERDiagram::from_json(json).unwrap();
    let mermaid = diagram.to_mermaid();
    assert!(mermaid.contains("Customer"));
    assert!(mermaid.contains("Order"));
}

#[test]
fn er_diagram_from_yaml() {
    let yaml = r#"
title: Database Schema
entities:
  - name: User
    attributes:
      - type: int
        name: id
        key: PK
      - type: string
        name: email
  - name: Post
    attributes:
      - type: int
        name: id
        key: PK
      - type: string
        name: title
relationships:
  - from: User
    from_cardinality: exactly-one
    to: Post
    to_cardinality: zero-or-more
    label: writes
"#;

    let diagram = ERDiagram::from_yaml(yaml).unwrap();
    let mermaid = diagram.to_mermaid();
    assert!(mermaid.contains("User"));
    assert!(mermaid.contains("Post"));
    assert!(mermaid.contains("writes"));
}

#[test]
fn er_diagram_from_toml() {
    let toml = r#"
title = "Simple Schema"

[[entities]]
name = "Customer"

[[entities]]
name = "Order"

[[relationships]]
from = "Customer"
from_cardinality = "exactly-one"
to = "Order"
to_cardinality = "zero-or-more"
label = "places"
"#;

    let diagram = ERDiagram::from_toml(toml).unwrap();
    let mermaid = diagram.to_mermaid();
    assert!(mermaid.contains("Customer"));
    assert!(mermaid.contains("Order"));
}

#[test]
fn er_diagram_type() {
    let diagram = ERDiagram::builder().build();
    assert_eq!(diagram.diagram_type(), "erDiagram");
}

#[test]
fn er_diagram_build_script_with_theme() {
    use mermaid_rs::core::Theme;

    let diagram = ERDiagram::builder()
        .theme(Theme::Dark)
        .entity_simple("Test")
        .build();

    let script = diagram.build_script();
    assert!(script.contains("%%{init:"));
    assert!(script.contains("'theme': 'dark'"));
}

#[test]
fn er_attribute_types() {
    let types = vec![
        (AttributeType::String, "string"),
        (AttributeType::Int, "int"),
        (AttributeType::Float, "float"),
        (AttributeType::Boolean, "boolean"),
        (AttributeType::Date, "date"),
        (AttributeType::DateTime, "datetime"),
    ];

    for (attr_type, expected) in types {
        let attr = Attribute::new(attr_type, "test");
        assert!(attr.to_mermaid().contains(expected));
    }
}

#[test]
fn er_attribute_keys() {
    let keys = vec![
        (AttributeKey::PrimaryKey, "PK"),
        (AttributeKey::ForeignKey, "FK"),
        (AttributeKey::UniqueKey, "UK"),
    ];

    for (key, expected) in keys {
        let attr = Attribute::new(AttributeType::Int, "id").with_key(key);
        assert!(attr.to_mermaid().contains(expected));
    }
}

#[test]
fn er_diagram_one_to_many() {
    let diagram = ERDiagram::builder()
        .entity_simple("User")
        .entity_simple("Order")
        .one_to_many("User", "Order", None)
        .build();

    let mermaid = diagram.to_mermaid();
    assert!(mermaid.contains("User"));
    assert!(mermaid.contains("Order"));
}

#[test]
fn er_diagram_many_to_many() {
    let diagram = ERDiagram::builder()
        .entity_simple("Student")
        .entity_simple("Course")
        .many_to_many("Student", "Course", Some("enrolls"))
        .build();

    let mermaid = diagram.to_mermaid();
    assert!(mermaid.contains("Student"));
    assert!(mermaid.contains("Course"));
}
