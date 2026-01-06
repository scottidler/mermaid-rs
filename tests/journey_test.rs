mod common;

use mermaid_rs::core::Diagram;
use mermaid_rs::diagrams::journey::{Journey, Section, Task};

#[test]
fn journey_empty() {
    let journey = Journey::builder().build();
    let mermaid = journey.to_mermaid();
    assert!(mermaid.starts_with("journey"));
}

#[test]
fn journey_with_title() {
    let journey = Journey::builder().title("My Journey").build();
    let mermaid = journey.to_mermaid();
    assert!(mermaid.contains("title My Journey"));
}

#[test]
fn journey_with_section() {
    let journey = Journey::builder().section("Getting Started").build();
    let mermaid = journey.to_mermaid();
    assert!(mermaid.contains("section Getting Started"));
}

#[test]
fn journey_with_tasks() {
    let journey = Journey::builder()
        .section("Setup")
        .task("Install software", 5)
        .task("Configure settings", 4)
        .build();
    let mermaid = journey.to_mermaid();
    assert!(mermaid.contains("section Setup"));
    assert!(mermaid.contains("Install software: 5"));
    assert!(mermaid.contains("Configure settings: 4"));
}

#[test]
fn journey_task_with_actors() {
    let journey = Journey::builder()
        .section("Onboarding")
        .task_with_actors("Sign up", 5, vec!["User".to_string(), "System".to_string()])
        .build();
    let mermaid = journey.to_mermaid();
    assert!(mermaid.contains("Sign up: 5: User, System"));
}

#[test]
fn journey_task_score_clamped() {
    // Scores should be clamped to 0-5 range
    let task_high = Task::new("Test", 10); // Should clamp to 5
    let task_low = Task::new("Test", 0);

    assert!(task_high.to_mermaid().contains(": 5"));
    assert!(task_low.to_mermaid().contains(": 0"));
}

#[test]
fn journey_multiple_sections() {
    let journey = Journey::builder()
        .section("Phase 1")
        .task("Task A", 3)
        .section("Phase 2")
        .task("Task B", 4)
        .build();
    let mermaid = journey.to_mermaid();
    assert!(mermaid.contains("section Phase 1"));
    assert!(mermaid.contains("section Phase 2"));
    assert!(mermaid.contains("Task A"));
    assert!(mermaid.contains("Task B"));
}

#[test]
fn journey_from_json() {
    let json = r#"{
        "title": "User Onboarding",
        "sections": [
            {
                "name": "Registration",
                "tasks": [
                    {"name": "Visit site", "score": 5},
                    {"name": "Fill form", "score": 3}
                ]
            }
        ]
    }"#;

    let journey = Journey::from_json(json).unwrap();
    let mermaid = journey.to_mermaid();
    assert!(mermaid.contains("title User Onboarding"));
    assert!(mermaid.contains("section Registration"));
    assert!(mermaid.contains("Visit site: 5"));
}

#[test]
fn journey_from_yaml() {
    let yaml = r#"
title: Customer Journey
sections:
  - name: Discovery
    tasks:
      - name: See ad
        score: 4
      - name: Visit website
        score: 5
        actors:
          - Customer
  - name: Purchase
    tasks:
      - name: Add to cart
        score: 4
      - name: Checkout
        score: 3
"#;

    let journey = Journey::from_yaml(yaml).unwrap();
    let mermaid = journey.to_mermaid();
    assert!(mermaid.contains("title Customer Journey"));
    assert!(mermaid.contains("section Discovery"));
    assert!(mermaid.contains("section Purchase"));
    assert!(mermaid.contains("See ad: 4"));
}

#[test]
fn journey_from_toml() {
    let toml = r#"
title = "Simple Journey"

[[sections]]
name = "Start"

[[sections.tasks]]
name = "Begin"
score = 5

[[sections.tasks]]
name = "Continue"
score = 4
"#;

    let journey = Journey::from_toml(toml).unwrap();
    let mermaid = journey.to_mermaid();
    assert!(mermaid.contains("title Simple Journey"));
    assert!(mermaid.contains("section Start"));
    assert!(mermaid.contains("Begin: 5"));
}

#[test]
fn journey_diagram_type() {
    let journey = Journey::builder().build();
    assert_eq!(journey.diagram_type(), "journey");
}

#[test]
fn journey_add_pre_built_section() {
    let section = Section::new("Custom").with_task(Task::new("Custom Task", 4));

    let journey = Journey::builder().add_section(section).build();

    let mermaid = journey.to_mermaid();
    assert!(mermaid.contains("section Custom"));
    assert!(mermaid.contains("Custom Task: 4"));
}

#[test]
fn journey_task_basic() {
    let task = Task::new("Test Task", 3);
    let mermaid = task.to_mermaid();
    assert!(mermaid.contains("Test Task: 3"));
}

#[test]
fn journey_section_with_tasks() {
    let section = Section::new("Section Name")
        .with_task(Task::new("Task 1", 5))
        .with_task(Task::new("Task 2", 4));

    let mermaid = section.to_mermaid();
    assert!(mermaid.contains("section Section Name"));
    assert!(mermaid.contains("Task 1: 5"));
    assert!(mermaid.contains("Task 2: 4"));
}
