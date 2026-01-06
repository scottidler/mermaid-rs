mod common;

use mermaid_rs::core::Diagram;
use mermaid_rs::diagrams::state::{
    Choice, CompositeState, ConcurrentRegion, ConcurrentState, Fork, Join, StateDiagram, Transition,
};
use mermaid_rs::Direction;

#[test]
fn state_diagram_empty() {
    let diagram = StateDiagram::builder().build();
    let mermaid = diagram.to_mermaid();
    assert!(mermaid.starts_with("stateDiagram-v2"));
}

#[test]
fn state_diagram_with_direction() {
    let diagram = StateDiagram::builder()
        .direction(Direction::LeftRight)
        .build();
    let mermaid = diagram.to_mermaid();
    assert!(mermaid.contains("direction LR"));
}

#[test]
fn state_diagram_with_states() {
    let diagram = StateDiagram::builder()
        .state_simple("Active")
        .state_simple("Inactive")
        .state_with_description("Pending", "Waiting for approval")
        .build();
    let mermaid = diagram.to_mermaid();

    // mermaid-py lowercases state IDs
    assert!(mermaid.contains("active"));
    assert!(mermaid.contains("inactive"));
    assert!(mermaid.contains("pending : Waiting for approval"));
}

#[test]
fn state_diagram_with_transitions() {
    let diagram = StateDiagram::builder()
        .state_simple("Off")
        .state_simple("On")
        .transition_simple("Off", "On")
        .transition_simple("On", "Off")
        .build();
    let mermaid = diagram.to_mermaid();

    // mermaid-py lowercases state IDs
    assert!(mermaid.contains("off --> on"));
    assert!(mermaid.contains("on --> off"));
}

#[test]
fn state_diagram_with_transition_labels() {
    let diagram = StateDiagram::builder()
        .state_simple("Idle")
        .state_simple("Running")
        .transition_with_label("Idle", "Running", "start")
        .transition_with_label("Running", "Idle", "stop")
        .build();
    let mermaid = diagram.to_mermaid();

    // mermaid-py lowercases state IDs
    assert!(mermaid.contains("idle --> running : start"));
    assert!(mermaid.contains("running --> idle : stop"));
}

#[test]
fn state_diagram_with_start_end() {
    let diagram = StateDiagram::builder()
        .state_simple("Active")
        .from_start("Active")
        .to_end("Active")
        .build();
    let mermaid = diagram.to_mermaid();

    // mermaid-py lowercases state IDs (except [*])
    assert!(mermaid.contains("[*] --> active"));
    assert!(mermaid.contains("active --> [*]"));
}

#[test]
fn state_diagram_with_choice() {
    let diagram = StateDiagram::builder()
        .state_simple("Check")
        .state_simple("Valid")
        .state_simple("Invalid")
        .transition_simple("Check", "decision")
        .choice(
            Choice::new("decision")
                .with_condition("is valid", "Valid")
                .with_condition("is invalid", "Invalid"),
        )
        .build();
    let mermaid = diagram.to_mermaid();

    assert!(mermaid.contains("<<choice>>"));
    assert!(mermaid.contains("decision --> Valid: is valid"));
}

#[test]
fn state_diagram_with_fork_join() {
    let diagram = StateDiagram::builder()
        .state_simple("Start")
        .state_simple("Task1")
        .state_simple("Task2")
        .state_simple("End")
        .transition_simple("Start", "fork1")
        .fork(Fork::new("fork1").with_target("Task1").with_target("Task2"))
        .join(
            Join::new("join1", "End")
                .with_source("Task1")
                .with_source("Task2"),
        )
        .build();
    let mermaid = diagram.to_mermaid();

    assert!(mermaid.contains("<<fork>>"));
    assert!(mermaid.contains("<<join>>"));
}

#[test]
fn state_diagram_with_composite() {
    let diagram = StateDiagram::builder()
        .from_start("Parent")
        .composite(
            CompositeState::new("Parent")
                .with_title("Parent State")
                .with_state_simple("Child1")
                .with_state_simple("Child2")
                .with_transition_simple("Child1", "Child2"),
        )
        .to_end("Parent")
        .build();
    let mermaid = diagram.to_mermaid();

    assert!(mermaid.contains("state \"Parent State\" as Parent"));
    // mermaid-py lowercases state IDs
    assert!(mermaid.contains("child1"));
    assert!(mermaid.contains("child1 --> child2"));
    assert!(mermaid.contains("}"));
}

#[test]
fn state_diagram_with_concurrent() {
    let diagram = StateDiagram::builder()
        .concurrent(
            ConcurrentState::new("Parallel")
                .with_title("Parallel Work")
                .with_region(
                    ConcurrentRegion::new()
                        .with_state_simple("A1")
                        .with_state_simple("A2")
                        .with_transition_simple("A1", "A2"),
                )
                .with_region(
                    ConcurrentRegion::new()
                        .with_state_simple("B1")
                        .with_state_simple("B2")
                        .with_transition_simple("B1", "B2"),
                ),
        )
        .build();
    let mermaid = diagram.to_mermaid();

    assert!(mermaid.contains("state \"Parallel Work\" as Parallel"));
    // mermaid-py lowercases state IDs
    assert!(mermaid.contains("a1"));
    assert!(mermaid.contains("b1"));
    assert!(mermaid.contains("--")); // Region separator
}

#[test]
fn state_diagram_from_json() {
    let json = r#"{
        "direction": "LR",
        "title": "Order State",
        "states": [
            {"id": "Pending"},
            {"id": "Shipped"},
            {"id": "Delivered"}
        ],
        "transitions": [
            {"from": "[*]", "to": "Pending"},
            {"from": "Pending", "to": "Shipped", "label": "ship"},
            {"from": "Shipped", "to": "Delivered", "label": "deliver"},
            {"from": "Delivered", "to": "[*]"}
        ]
    }"#;

    let diagram = StateDiagram::from_json(json).unwrap();
    let mermaid = diagram.to_mermaid();

    assert!(mermaid.contains("direction LR"));
    // mermaid-py lowercases state IDs
    assert!(mermaid.contains("[*] --> pending"));
    assert!(mermaid.contains("pending --> shipped : ship"));
}

#[test]
fn state_diagram_from_yaml() {
    let yaml = r#"
title: Traffic Light
direction: TB
states:
  - id: Red
    description: Stop
  - id: Yellow
    description: Caution
  - id: Green
    description: Go
transitions:
  - from: "[*]"
    to: Red
  - from: Red
    to: Green
    label: timer
  - from: Green
    to: Yellow
    label: timer
  - from: Yellow
    to: Red
    label: timer
"#;

    let diagram = StateDiagram::from_yaml(yaml).unwrap();
    let mermaid = diagram.to_mermaid();

    // mermaid-py lowercases state IDs and uses "id : content" format
    assert!(mermaid.contains("red : Stop"));
    assert!(mermaid.contains("green : Go"));
    assert!(mermaid.contains("red --> green : timer"));
}

#[test]
fn state_diagram_from_toml() {
    let toml = r#"
direction = "TB"
title = "Simple State"

[[states]]
id = "On"

[[states]]
id = "Off"

[[transitions]]
from = "[*]"
to = "Off"

[[transitions]]
from = "Off"
to = "On"
label = "toggle"

[[transitions]]
from = "On"
to = "Off"
label = "toggle"
"#;

    let diagram = StateDiagram::from_toml(toml).unwrap();
    let mermaid = diagram.to_mermaid();

    // mermaid-py lowercases state IDs
    assert!(mermaid.contains("[*] --> off"));
    assert!(mermaid.contains("off --> on : toggle"));
}

#[test]
fn state_diagram_type() {
    let diagram = StateDiagram::builder().build();
    assert_eq!(diagram.diagram_type(), "stateDiagram-v2");
}

#[test]
fn state_diagram_build_script_includes_frontmatter() {
    let diagram = StateDiagram::builder()
        .title("Test State")
        .state_simple("Active")
        .build();

    let script = diagram.build_script();
    assert!(script.contains("---"));
    assert!(script.contains("title: Test State"));
}

#[test]
fn transition_helpers() {
    // mermaid-py lowercases state IDs
    let from_start = Transition::from_start("Init");
    assert_eq!(from_start.to_mermaid(), "[*] --> init");

    let to_end = Transition::to_end("Final");
    assert_eq!(to_end.to_mermaid(), "final --> [*]");

    let with_label = Transition::new("A", "B").with_label("event");
    assert_eq!(with_label.to_mermaid(), "a --> b : event");
}
