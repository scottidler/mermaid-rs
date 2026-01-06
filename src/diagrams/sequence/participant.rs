use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, rename = "type")]
    pub participant_type: ParticipantType,
}

impl Participant {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: None,
            participant_type: ParticipantType::default(),
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn actor(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: None,
            participant_type: ParticipantType::Actor,
        }
    }

    /// Create a participant (not actor) explicitly
    pub fn non_actor(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: None,
            participant_type: ParticipantType::Participant,
        }
    }

    pub fn to_mermaid(&self) -> String {
        let keyword = match self.participant_type {
            ParticipantType::Actor => "actor",
            ParticipantType::Participant => "participant",
        };

        match &self.label {
            Some(label) => format!("{} {} as {}", keyword, self.id, label),
            None => format!("{} {}", keyword, self.id),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParticipantType {
    Participant,
    #[default]
    Actor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantBox {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    pub members: Vec<String>,
}

impl ParticipantBox {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            color: None,
            members: Vec::new(),
        }
    }

    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn with_member(mut self, member: impl Into<String>) -> Self {
        self.members.push(member.into());
        self
    }

    pub fn with_members(mut self, members: Vec<String>) -> Self {
        self.members = members;
        self
    }

    pub fn to_mermaid_start(&self) -> String {
        match &self.color {
            Some(color) => format!("box {} {}\n", color, self.title),
            None => format!("box {}\n", self.title),
        }
    }

    pub fn to_mermaid_end(&self) -> &'static str {
        "end"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn participant_basic() {
        let p = Participant::new("Alice");
        assert_eq!(p.to_mermaid(), "actor Alice");
    }

    #[test]
    fn participant_with_label() {
        let p = Participant::new("A").with_label("Alice");
        assert_eq!(p.to_mermaid(), "actor A as Alice");
    }

    #[test]
    fn actor_basic() {
        let a = Participant::actor("User");
        assert_eq!(a.to_mermaid(), "actor User");
    }

    #[test]
    fn box_basic() {
        let b = ParticipantBox::new("Frontend")
            .with_color("rgb(200,220,255)")
            .with_members(vec!["Client".to_string()]);
        assert!(b.to_mermaid_start().contains("box rgb(200,220,255) Frontend"));
    }
}
