mod common;

use mermaid_rs::core::Diagram;
use mermaid_rs::diagrams::sequence::{Logic, Message, MessageType, SequenceDiagram};

#[test]
fn sequence_empty() {
    let diagram = SequenceDiagram::builder().build();
    let mermaid = diagram.to_mermaid();
    assert!(mermaid.starts_with("sequenceDiagram"));
}

#[test]
fn sequence_with_participants() {
    let diagram = SequenceDiagram::builder()
        .actor("User")
        .participant_simple("Server")
        .build();
    let mermaid = diagram.to_mermaid();

    assert!(mermaid.contains("actor User"));
    assert!(mermaid.contains("participant Server"));
}

#[test]
fn sequence_with_labeled_participants() {
    let diagram = SequenceDiagram::builder()
        .actor_with_label("U", "User")
        .participant_with_label("S", "API Server")
        .build();
    let mermaid = diagram.to_mermaid();

    assert!(mermaid.contains("actor U as User"));
    assert!(mermaid.contains("participant S as API Server"));
}

#[test]
fn sequence_with_messages() {
    let diagram = SequenceDiagram::builder()
        .participant_simple("Alice")
        .participant_simple("Bob")
        .message_simple("Alice", "Bob", "Hello!")
        .message_simple("Bob", "Alice", "Hi!")
        .build();
    let mermaid = diagram.to_mermaid();

    assert!(mermaid.contains("Alice->Bob: Hello!"));
    assert!(mermaid.contains("Bob->Alice: Hi!"));
}

#[test]
fn sequence_with_message_types() {
    let diagram = SequenceDiagram::builder()
        .participant_simple("A")
        .participant_simple("B")
        .message_with_type("A", "B", MessageType::SolidArrow, "async")
        .message_with_type("B", "A", MessageType::DottedArrow, "response")
        .build();
    let mermaid = diagram.to_mermaid();

    assert!(mermaid.contains("A->>B: async"));
    assert!(mermaid.contains("B-->>A: response"));
}

#[test]
fn sequence_with_autonumber() {
    let diagram = SequenceDiagram::builder()
        .autonumber(true)
        .participant_simple("A")
        .participant_simple("B")
        .message_simple("A", "B", "Message")
        .build();
    let mermaid = diagram.to_mermaid();

    assert!(mermaid.contains("autonumber"));
}

#[test]
fn sequence_with_notes() {
    let diagram = SequenceDiagram::builder()
        .participant_simple("Alice")
        .note_right("Alice", "This is Alice")
        .note_left("Alice", "Left note")
        .note_over("Alice", "Over note")
        .build();
    let mermaid = diagram.to_mermaid();

    assert!(mermaid.contains("Note right of Alice"));
    assert!(mermaid.contains("Note left of Alice"));
    assert!(mermaid.contains("Note over Alice"));
}

#[test]
fn sequence_with_logic_alt() {
    let diagram = SequenceDiagram::builder()
        .participant_simple("Client")
        .participant_simple("Server")
        .logic(
            Logic::alt("Success")
                .with_message(Message::new("Server", "Client").with_text("200 OK"))
                .with_else_condition("Failure", vec![Message::new("Server", "Client").with_text("500 Error")]),
        )
        .build();
    let mermaid = diagram.to_mermaid();

    assert!(mermaid.contains("alt Success"));
    assert!(mermaid.contains("else Failure"));
    assert!(mermaid.contains("end"));
}

#[test]
fn sequence_with_logic_loop() {
    let diagram = SequenceDiagram::builder()
        .participant_simple("Server")
        .participant_simple("Client")
        .logic(Logic::loop_block("Every 5 seconds").with_message(Message::new("Server", "Client").with_text("ping")))
        .build();
    let mermaid = diagram.to_mermaid();

    assert!(mermaid.contains("loop Every 5 seconds"));
    assert!(mermaid.contains("end"));
}

#[test]
fn sequence_from_json() {
    let json = r#"{
        "title": "API Flow",
        "autonumber": true,
        "participants": [
            {"id": "User", "type": "actor"},
            {"id": "API", "type": "participant", "label": "API Server"}
        ],
        "messages": [
            {"from": "User", "to": "API", "text": "GET /data", "type": "solid-arrow"},
            {"from": "API", "to": "User", "text": "200 OK", "type": "dotted-arrow"}
        ]
    }"#;

    let diagram = SequenceDiagram::from_json(json).unwrap();
    let mermaid = diagram.to_mermaid();

    assert!(mermaid.contains("autonumber"));
    assert!(mermaid.contains("actor User"));
    assert!(mermaid.contains("User->>API"));
    assert!(mermaid.contains("API-->>User"));
}

#[test]
fn sequence_from_yaml() {
    let yaml = r#"
title: Login Flow
participants:
  - id: User
    type: actor
  - id: App
    type: participant
  - id: Auth
    type: participant
    label: Auth Service
messages:
  - from: User
    to: App
    text: Enter credentials
  - from: App
    to: Auth
    type: solid-arrow
    text: Validate
  - from: Auth
    to: App
    type: dotted-arrow
    text: Token
notes:
  - position: right
    over: [Auth]
    text: JWT validation
"#;

    let diagram = SequenceDiagram::from_yaml(yaml).unwrap();
    let mermaid = diagram.to_mermaid();

    assert!(mermaid.contains("actor User"));
    assert!(mermaid.contains("Auth as Auth Service"));
    assert!(mermaid.contains("User->App"));
}

#[test]
fn sequence_from_toml() {
    let toml = r#"
title = "Simple Sequence"
autonumber = true

[[participants]]
id = "A"
type = "participant"

[[participants]]
id = "B"
type = "participant"

[[messages]]
from = "A"
to = "B"
text = "Hello"
"#;

    let diagram = SequenceDiagram::from_toml(toml).unwrap();
    let mermaid = diagram.to_mermaid();

    assert!(mermaid.contains("autonumber"));
    assert!(mermaid.contains("A->B: Hello"));
}

#[test]
fn sequence_message_types() {
    assert_eq!(MessageType::Solid.arrow(), "->");
    assert_eq!(MessageType::Dotted.arrow(), "-->");
    assert_eq!(MessageType::SolidArrow.arrow(), "->>");
    assert_eq!(MessageType::DottedArrow.arrow(), "-->>");
    assert_eq!(MessageType::SolidCross.arrow(), "-x");
    assert_eq!(MessageType::DottedCross.arrow(), "--x");
    assert_eq!(MessageType::SolidOpen.arrow(), "-)");
    assert_eq!(MessageType::DottedOpen.arrow(), "--)");
}

#[test]
fn sequence_diagram_type() {
    let diagram = SequenceDiagram::builder().build();
    assert_eq!(diagram.diagram_type(), "sequenceDiagram");
}

#[test]
fn sequence_build_script_includes_frontmatter() {
    let diagram = SequenceDiagram::builder()
        .title("Test Sequence")
        .participant_simple("A")
        .build();

    let script = diagram.build_script();
    assert!(script.contains("---"));
    assert!(script.contains("title: Test Sequence"));
}
