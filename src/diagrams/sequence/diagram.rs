use serde::{Deserialize, Serialize};

use crate::core::{Config, Diagram, FromConfig, MermaidError, Theme};

use super::{Logic, Message, MessageType, Note, NotePosition, Participant, ParticipantBox};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SequenceDiagram {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    pub autonumber: bool,
    #[serde(default)]
    pub participants: Vec<Participant>,
    #[serde(default)]
    pub boxes: Vec<ParticipantBox>,
    #[serde(default)]
    pub messages: Vec<Message>,
    #[serde(default)]
    pub notes: Vec<Note>,
    #[serde(default)]
    pub logic: Vec<Logic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Config>,
    /// Raw mermaid passthrough (if set, ignores other fields)
    #[serde(skip)]
    raw_mermaid: Option<String>,
}

impl SequenceDiagram {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> SequenceDiagramBuilder {
        SequenceDiagramBuilder::new()
    }

    pub fn from_raw_mermaid(mermaid: String) -> Self {
        Self {
            raw_mermaid: Some(mermaid),
            ..Default::default()
        }
    }

    pub fn from_json(json: &str) -> Result<Self, MermaidError> {
        let diagram: Self = serde_json::from_str(json)?;
        Ok(diagram)
    }

    pub fn from_yaml(yaml: &str) -> Result<Self, MermaidError> {
        let diagram: Self = serde_yaml::from_str(yaml)?;
        Ok(diagram)
    }

    pub fn from_toml(toml: &str) -> Result<Self, MermaidError> {
        let diagram: Self = toml::from_str(toml)?;
        Ok(diagram)
    }

    /// Find participants that belong to a specific box
    fn participants_in_box(&self, box_def: &ParticipantBox) -> Vec<&Participant> {
        self.participants
            .iter()
            .filter(|p| box_def.members.contains(&p.id))
            .collect()
    }
}

impl Diagram for SequenceDiagram {
    fn to_mermaid(&self) -> String {
        // If raw mermaid was provided, return it directly
        if let Some(raw) = &self.raw_mermaid {
            return raw.clone();
        }

        let mut output = String::new();

        // Start with sequenceDiagram
        output.push_str("sequenceDiagram\n");

        // Autonumber
        if self.autonumber {
            output.push_str("    autonumber\n");
        }

        // Collect participants that are in boxes
        let mut participants_in_boxes: Vec<String> = Vec::new();
        for box_def in &self.boxes {
            participants_in_boxes.extend(box_def.members.clone());
        }

        // Render participants not in any box
        for participant in &self.participants {
            if !participants_in_boxes.contains(&participant.id) {
                output.push_str(&format!("    {}\n", participant.to_mermaid()));
            }
        }

        // Render boxes with their participants
        for box_def in &self.boxes {
            output.push_str(&format!("    {}", box_def.to_mermaid_start()));
            for participant in self.participants_in_box(box_def) {
                output.push_str(&format!("        {}\n", participant.to_mermaid()));
            }
            output.push_str(&format!("    {}\n", box_def.to_mermaid_end()));
        }

        // Render messages
        for message in &self.messages {
            output.push_str(&format!("    {}\n", message.to_mermaid()));
        }

        // Render notes
        for note in &self.notes {
            output.push_str(&format!("    {}\n", note.to_mermaid()));
        }

        // Render logic blocks
        for logic_block in &self.logic {
            // Indent logic block output
            for line in logic_block.to_mermaid().lines() {
                output.push_str(&format!("    {}\n", line));
            }
        }

        output
    }

    fn diagram_type(&self) -> &'static str {
        "sequenceDiagram"
    }

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn config(&self) -> Option<&Config> {
        self.config.as_ref()
    }
}

impl FromConfig for SequenceDiagram {
    fn from_json(json: &str) -> Result<Self, MermaidError> {
        Self::from_json(json)
    }

    fn from_yaml(yaml: &str) -> Result<Self, MermaidError> {
        Self::from_yaml(yaml)
    }

    fn from_toml(toml: &str) -> Result<Self, MermaidError> {
        Self::from_toml(toml)
    }
}

#[derive(Debug, Default)]
pub struct SequenceDiagramBuilder {
    title: Option<String>,
    autonumber: bool,
    participants: Vec<Participant>,
    boxes: Vec<ParticipantBox>,
    messages: Vec<Message>,
    notes: Vec<Note>,
    logic: Vec<Logic>,
    config: Option<Config>,
}

impl SequenceDiagramBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn autonumber(mut self, enabled: bool) -> Self {
        self.autonumber = enabled;
        self
    }

    pub fn participant(mut self, participant: Participant) -> Self {
        self.participants.push(participant);
        self
    }

    pub fn actor(mut self, id: impl Into<String>) -> Self {
        self.participants.push(Participant::actor(id));
        self
    }

    pub fn actor_with_label(mut self, id: impl Into<String>, label: impl Into<String>) -> Self {
        self.participants.push(Participant::actor(id).with_label(label));
        self
    }

    pub fn participant_simple(mut self, id: impl Into<String>) -> Self {
        self.participants.push(Participant::non_actor(id));
        self
    }

    pub fn participant_with_label(mut self, id: impl Into<String>, label: impl Into<String>) -> Self {
        self.participants.push(Participant::non_actor(id).with_label(label));
        self
    }

    pub fn participant_box(mut self, box_def: ParticipantBox) -> Self {
        self.boxes.push(box_def);
        self
    }

    pub fn message(mut self, message: Message) -> Self {
        self.messages.push(message);
        self
    }

    pub fn message_simple(mut self, from: impl Into<String>, to: impl Into<String>, text: impl Into<String>) -> Self {
        self.messages.push(Message::new(from, to).with_text(text));
        self
    }

    pub fn message_with_type(
        mut self,
        from: impl Into<String>,
        to: impl Into<String>,
        message_type: MessageType,
        text: impl Into<String>,
    ) -> Self {
        self.messages
            .push(Message::new(from, to).with_type(message_type).with_text(text));
        self
    }

    pub fn note(mut self, note: Note) -> Self {
        self.notes.push(note);
        self
    }

    pub fn note_over(mut self, participant: impl Into<String>, text: impl Into<String>) -> Self {
        self.notes
            .push(Note::over_participant(NotePosition::Over, participant, text));
        self
    }

    pub fn note_left(mut self, participant: impl Into<String>, text: impl Into<String>) -> Self {
        self.notes
            .push(Note::over_participant(NotePosition::Left, participant, text));
        self
    }

    pub fn note_right(mut self, participant: impl Into<String>, text: impl Into<String>) -> Self {
        self.notes
            .push(Note::over_participant(NotePosition::Right, participant, text));
        self
    }

    pub fn logic(mut self, logic_block: Logic) -> Self {
        self.logic.push(logic_block);
        self
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        let config = self.config.get_or_insert_with(Config::default);
        config.theme = theme;
        self
    }

    pub fn config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    pub fn build(self) -> SequenceDiagram {
        SequenceDiagram {
            title: self.title,
            autonumber: self.autonumber,
            participants: self.participants,
            boxes: self.boxes,
            messages: self.messages,
            notes: self.notes,
            logic: self.logic,
            config: self.config,
            raw_mermaid: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequence_basic() {
        let diagram = SequenceDiagram::builder()
            .actor("User")
            .participant_simple("Server")
            .message_simple("User", "Server", "Request")
            .message_with_type("Server", "User", MessageType::DottedArrow, "Response")
            .build();

        let mermaid = diagram.to_mermaid();
        assert!(mermaid.contains("sequenceDiagram"));
        assert!(mermaid.contains("actor User"));
        assert!(mermaid.contains("participant Server"));
        // mermaid-py defaults to SolidArrow (->>)
        assert!(mermaid.contains("User->>Server: Request"));
        assert!(mermaid.contains("Server-->>User: Response"));
    }

    #[test]
    fn sequence_with_autonumber() {
        let diagram = SequenceDiagram::builder()
            .autonumber(true)
            .participant_simple("A")
            .participant_simple("B")
            .message_simple("A", "B", "Hello")
            .build();

        let mermaid = diagram.to_mermaid();
        assert!(mermaid.contains("autonumber"));
    }

    #[test]
    fn sequence_with_boxes() {
        let diagram = SequenceDiagram::builder()
            .participant_with_label("C", "Client")
            .participant_with_label("S", "Server")
            .participant_box(
                ParticipantBox::new("Backend")
                    .with_color("rgb(200,255,200)")
                    .with_member("S"),
            )
            .message_simple("C", "S", "Request")
            .build();

        let mermaid = diagram.to_mermaid();
        assert!(mermaid.contains("box rgb(200,255,200) Backend"));
        assert!(mermaid.contains("end"));
    }

    #[test]
    fn sequence_with_notes() {
        let diagram = SequenceDiagram::builder()
            .participant_simple("Alice")
            .participant_simple("Bob")
            .message_simple("Alice", "Bob", "Hello")
            .note_right("Alice", "This is a note")
            .build();

        let mermaid = diagram.to_mermaid();
        assert!(mermaid.contains("Note right of Alice"));
    }

    #[test]
    fn sequence_with_logic() {
        let diagram = SequenceDiagram::builder()
            .participant_simple("A")
            .participant_simple("B")
            .logic(
                Logic::alt("Success")
                    .with_message(Message::new("A", "B").with_text("OK"))
                    .with_else_condition("Failure", vec![Message::new("A", "B").with_text("Error")]),
            )
            .build();

        let mermaid = diagram.to_mermaid();
        assert!(mermaid.contains("alt Success"));
        assert!(mermaid.contains("else Failure"));
        assert!(mermaid.contains("end"));
    }

    #[test]
    fn sequence_from_json() {
        let json = r#"{
            "autonumber": true,
            "participants": [
                {"id": "A", "type": "actor"},
                {"id": "B", "type": "participant"}
            ],
            "messages": [
                {"from": "A", "to": "B", "text": "Hello"}
            ]
        }"#;

        let diagram = SequenceDiagram::from_json(json).unwrap();
        assert!(diagram.autonumber);
        assert_eq!(diagram.participants.len(), 2);
        assert_eq!(diagram.messages.len(), 1);
    }

    #[test]
    fn sequence_from_yaml() {
        let yaml = r#"
title: API Flow
autonumber: true
participants:
  - id: User
    type: actor
  - id: API
    type: participant
messages:
  - from: User
    to: API
    text: GET /users
    type: solid-arrow
  - from: API
    to: User
    text: 200 OK
    type: dotted-arrow
"#;

        let diagram = SequenceDiagram::from_yaml(yaml).unwrap();
        assert_eq!(diagram.title, Some("API Flow".to_string()));
        assert!(diagram.autonumber);
        assert_eq!(diagram.participants.len(), 2);
        assert_eq!(diagram.messages.len(), 2);
    }

    #[test]
    fn sequence_raw_mermaid() {
        let raw = "sequenceDiagram\n    Alice->>Bob: Hello";
        let diagram = SequenceDiagram::from_raw_mermaid(raw.to_string());
        assert_eq!(diagram.to_mermaid(), raw);
    }
}
