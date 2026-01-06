mod common;

use mermaid_rs::core::Diagram;
use mermaid_rs::diagrams::flowchart::{FlowChart, LinkStyle, Node, NodeShape, Subgraph};
use mermaid_rs::Direction;

#[test]
fn flowchart_empty() {
    let chart = FlowChart::builder().build();
    let mermaid = chart.to_mermaid();
    assert!(mermaid.starts_with("flowchart"));
}

#[test]
fn flowchart_with_direction() {
    let chart = FlowChart::builder().direction(Direction::LeftRight).build();
    let mermaid = chart.to_mermaid();
    assert!(mermaid.contains("flowchart LR"));
}

#[test]
fn flowchart_with_nodes() {
    let chart = FlowChart::builder()
        .node_with_shape("A", "Start", NodeShape::Stadium)
        .node_with_shape("B", "Process", NodeShape::Rectangle)
        .node_with_shape("C", "End", NodeShape::Stadium)
        .build();
    let mermaid = chart.to_mermaid();

    assert!(mermaid.contains("A([\"Start\"])"));
    assert!(mermaid.contains("B[\"Process\"]"));
    assert!(mermaid.contains("C([\"End\"])"));
}

#[test]
fn flowchart_with_links() {
    let chart = FlowChart::builder()
        .node_simple("A", "Start")
        .node_simple("B", "End")
        .link_simple("A", "B")
        .build();
    let mermaid = chart.to_mermaid();
    assert!(mermaid.contains("A --> B"));
}

#[test]
fn flowchart_with_link_labels() {
    let chart = FlowChart::builder()
        .node_simple("A", "Start")
        .node_simple("B", "End")
        .link_with_label("A", "B", "goes to")
        .build();
    let mermaid = chart.to_mermaid();
    assert!(mermaid.contains("-->|goes to|"));
}

#[test]
fn flowchart_with_link_styles() {
    let chart = FlowChart::builder()
        .node_simple("A", "A")
        .node_simple("B", "B")
        .node_simple("C", "C")
        .link_with_style("A", "B", LinkStyle::Dotted)
        .link_with_style("B", "C", LinkStyle::Thick)
        .build();
    let mermaid = chart.to_mermaid();
    assert!(mermaid.contains("-.->"));
    assert!(mermaid.contains("==>"));
}

#[test]
fn flowchart_with_subgraph() {
    let chart = FlowChart::builder()
        .node_simple("A", "Node A")
        .node_simple("B", "Node B")
        .node_simple("C", "Node C")
        .subgraph(
            Subgraph::new("sg1")
                .with_title("Group")
                .with_nodes(vec!["A".to_string(), "B".to_string()]),
        )
        .link_simple("A", "B")
        .link_simple("B", "C")
        .build();

    let mermaid = chart.to_mermaid();
    assert!(mermaid.contains("subgraph sg1"));
    assert!(mermaid.contains("Group"));
    assert!(mermaid.contains("end"));
}

#[test]
fn flowchart_from_json() {
    let json = r#"{
        "direction": "LR",
        "title": "Test Flow",
        "nodes": [
            {"id": "A", "label": "Start", "shape": "stadium"},
            {"id": "B", "label": "End", "shape": "stadium"}
        ],
        "links": [
            {"from": "A", "to": "B", "label": "next"}
        ]
    }"#;

    let chart = FlowChart::from_json(json).unwrap();
    let mermaid = chart.to_mermaid();

    assert!(mermaid.contains("flowchart LR"));
    assert!(mermaid.contains("A([\"Start\"])"));
    assert!(mermaid.contains("-->|next|"));
}

#[test]
fn flowchart_from_yaml() {
    let yaml = r#"
direction: TB
title: Simple Flow
nodes:
  - id: start
    label: Begin
    shape: stadium
  - id: process
    label: Do Work
    shape: rectangle
  - id: end
    label: Done
    shape: stadium
links:
  - from: start
    to: process
  - from: process
    to: end
"#;

    let chart = FlowChart::from_yaml(yaml).unwrap();
    let mermaid = chart.to_mermaid();

    assert!(mermaid.contains("flowchart TB"));
    assert!(mermaid.contains("start([\"Begin\"])"));
    assert!(mermaid.contains("start --> process"));
}

#[test]
fn flowchart_from_toml() {
    let toml = r#"
direction = "TB"

[[nodes]]
id = "A"
label = "Start"
shape = "stadium"

[[nodes]]
id = "B"
label = "End"
shape = "stadium"

[[links]]
from = "A"
to = "B"
"#;

    let chart = FlowChart::from_toml(toml).unwrap();
    let mermaid = chart.to_mermaid();

    assert!(mermaid.contains("A([\"Start\"])"));
    assert!(mermaid.contains("A --> B"));
}

#[test]
fn flowchart_node_shapes() {
    // Test all node shapes generate correct syntax
    let shapes = vec![
        (NodeShape::Rectangle, "[\"Test\"]"),
        (NodeShape::Rounded, "(\"Test\")"),
        (NodeShape::Stadium, "([\"Test\"])"),
        (NodeShape::Rhombus, "{\"Test\"}"),
        (NodeShape::DoubleCircle, "(((\"Test\")))"),
    ];

    for (shape, expected_suffix) in shapes {
        let node = Node::new("X", "Test", shape);
        assert!(node.to_mermaid().contains(expected_suffix));
    }
}

#[test]
fn flowchart_diagram_type() {
    let chart = FlowChart::builder().build();
    assert_eq!(chart.diagram_type(), "flowchart");
}

#[test]
fn flowchart_build_script_includes_frontmatter() {
    let chart = FlowChart::builder().title("My Flow").node_simple("A", "Test").build();

    let script = chart.build_script();
    assert!(script.contains("---"));
    assert!(script.contains("title: My Flow"));
}
