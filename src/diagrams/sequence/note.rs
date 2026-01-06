use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub position: NotePosition,
    pub over: Vec<String>,
    pub text: String,
}

impl Note {
    pub fn new(position: NotePosition, text: impl Into<String>) -> Self {
        Self {
            position,
            over: Vec::new(),
            text: text.into(),
        }
    }

    pub fn over_participant(position: NotePosition, participant: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            position,
            over: vec![participant.into()],
            text: text.into(),
        }
    }

    pub fn over_participants(position: NotePosition, participants: Vec<String>, text: impl Into<String>) -> Self {
        Self {
            position,
            over: participants,
            text: text.into(),
        }
    }

    pub fn with_participant(mut self, participant: impl Into<String>) -> Self {
        self.over.push(participant.into());
        self
    }

    pub fn to_mermaid(&self) -> String {
        let position_str = match self.position {
            NotePosition::Left => "left of",
            NotePosition::Right => "right of",
            NotePosition::Over => "over",
        };

        let participants = self.over.join(",");
        format!("Note {} {}: {}", position_str, participants, self.text)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotePosition {
    Left,
    Right,
    #[default]
    Over,
}

impl NotePosition {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "left" => Some(Self::Left),
            "right" => Some(Self::Right),
            "over" => Some(Self::Over),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note_over_single() {
        let note = Note::over_participant(NotePosition::Over, "Alice", "This is a note");
        assert_eq!(note.to_mermaid(), "Note over Alice: This is a note");
    }

    #[test]
    fn note_over_multiple() {
        let note = Note::over_participants(
            NotePosition::Over,
            vec!["Alice".to_string(), "Bob".to_string()],
            "Shared note",
        );
        assert_eq!(note.to_mermaid(), "Note over Alice,Bob: Shared note");
    }

    #[test]
    fn note_left() {
        let note = Note::over_participant(NotePosition::Left, "Alice", "Left note");
        assert_eq!(note.to_mermaid(), "Note left of Alice: Left note");
    }

    #[test]
    fn note_right() {
        let note = Note::over_participant(NotePosition::Right, "Alice", "Right note");
        assert_eq!(note.to_mermaid(), "Note right of Alice: Right note");
    }
}
