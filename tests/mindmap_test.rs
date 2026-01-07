mod common;

use mermaid_rs::core::Diagram;
use mermaid_rs::diagrams::mindmap::{Mindmap, MindmapNode, MindmapNodeShape};

#[test]
fn mindmap_basic() {
    let mindmap = Mindmap::builder("Root").build();
    let mermaid = mindmap.to_mermaid();
    assert!(mermaid.starts_with("mindmap"));
    assert!(mermaid.contains("Root"));
}

#[test]
fn mindmap_with_title() {
    let mindmap = Mindmap::builder("Root").title("My Mindmap").build();
    assert_eq!(mindmap.title(), Some("My Mindmap"));
}

#[test]
fn mindmap_with_children() {
    let mindmap = Mindmap::builder("Root")
        .child("Child 1")
        .child("Child 2")
        .build();
    let mermaid = mindmap.to_mermaid();
    assert!(mermaid.contains("Root"));
    assert!(mermaid.contains("Child 1"));
    assert!(mermaid.contains("Child 2"));
}

#[test]
fn mindmap_with_nested_children() {
    let mindmap = Mindmap::builder("Root")
        .child_node(
            MindmapNode::new("Branch 1")
                .with_child(MindmapNode::new("Leaf 1"))
                .with_child(MindmapNode::new("Leaf 2")),
        )
        .build();
    let mermaid = mindmap.to_mermaid();
    assert!(mermaid.contains("Root"));
    assert!(mermaid.contains("Branch 1"));
    assert!(mermaid.contains("Leaf 1"));
    assert!(mermaid.contains("Leaf 2"));
}

#[test]
fn mindmap_node_shapes() {
    let shapes = vec![
        (MindmapNodeShape::Square, "[Test]"),
        (MindmapNodeShape::Rounded, "(Test)"),
        (MindmapNodeShape::Circle, "((Test))"),
        (MindmapNodeShape::Cloud, ")Test("),
        (MindmapNodeShape::Hexagon, "{{Test}}"),
        (MindmapNodeShape::Bang, "))Test(("),
    ];

    for (shape, expected) in shapes {
        let node = MindmapNode::new("Test").with_shape(shape);
        let mermaid = node.to_mermaid(0);
        assert!(
            mermaid.contains(expected),
            "Shape {:?} should contain {}",
            shape,
            expected
        );
    }
}

#[test]
fn mindmap_with_shaped_children() {
    let mindmap = Mindmap::builder("Root")
        .child_with_shape("Square Child", MindmapNodeShape::Square)
        .child_with_shape("Circle Child", MindmapNodeShape::Circle)
        .build();
    let mermaid = mindmap.to_mermaid();
    assert!(mermaid.contains("[Square Child]"));
    assert!(mermaid.contains("((Circle Child))"));
}

#[test]
fn mindmap_from_json() {
    let json = r#"{
        "title": "My Mindmap",
        "root": {
            "text": "Central Topic",
            "children": [
                {"text": "Branch A"},
                {"text": "Branch B", "shape": "square"}
            ]
        }
    }"#;

    let mindmap = Mindmap::from_json(json).unwrap();
    let mermaid = mindmap.to_mermaid();
    assert!(mermaid.contains("Central Topic"));
    assert!(mermaid.contains("Branch A"));
    assert!(mermaid.contains("Branch B"));
}

#[test]
fn mindmap_from_yaml() {
    let yaml = r#"
title: Project Planning
root:
  text: Project
  children:
    - text: Phase 1
      shape: square
      children:
        - text: Research
        - text: Design
    - text: Phase 2
      shape: square
      children:
        - text: Development
        - text: Testing
"#;

    let mindmap = Mindmap::from_yaml(yaml).unwrap();
    let mermaid = mindmap.to_mermaid();
    assert!(mermaid.contains("Project"));
    assert!(mermaid.contains("Phase 1"));
    assert!(mermaid.contains("Research"));
}

#[test]
fn mindmap_from_toml() {
    let toml = r#"
title = "Simple Mindmap"

[root]
text = "Main Topic"

[[root.children]]
text = "Subtopic 1"

[[root.children]]
text = "Subtopic 2"
"#;

    let mindmap = Mindmap::from_toml(toml).unwrap();
    let mermaid = mindmap.to_mermaid();
    assert!(mermaid.contains("Main Topic"));
    assert!(mermaid.contains("Subtopic 1"));
}

#[test]
fn mindmap_diagram_type() {
    let mindmap = Mindmap::builder("Root").build();
    assert_eq!(mindmap.diagram_type(), "mindmap");
}

#[test]
fn mindmap_build_script_with_theme() {
    use mermaid_rs::core::Theme;

    let mindmap = Mindmap::builder("Root").theme(Theme::Dark).build();

    let script = mindmap.build_script();
    assert!(script.contains("%%{init:"));
    assert!(script.contains("'theme': 'dark'"));
}

#[test]
fn mindmap_node_with_icon() {
    let node = MindmapNode::new("Test").with_icon("fa fa-check");
    let mermaid = node.to_mermaid(0);
    assert!(mermaid.contains("Test"));
    assert!(mermaid.contains("::icon(fa fa-check)"));
}

#[test]
fn mindmap_node_with_class() {
    let node = MindmapNode::new("Test").with_class("highlight");
    let mermaid = node.to_mermaid(0);
    assert!(mermaid.contains("Test"));
    assert!(mermaid.contains("::::highlight"));
}

#[test]
fn mindmap_root_with_shape() {
    let mindmap = Mindmap::builder("Root")
        .root_shape(MindmapNodeShape::Hexagon)
        .build();
    let mermaid = mindmap.to_mermaid();
    assert!(mermaid.contains("{{Root}}"));
}

#[test]
fn mindmap_root_with_icon() {
    let mindmap = Mindmap::builder("Root").root_icon("fa fa-home").build();
    let mermaid = mindmap.to_mermaid();
    assert!(mermaid.contains("::icon(fa fa-home)"));
}
