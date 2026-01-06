use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, rename = "type")]
    pub state_type: StateType,
}

impl State {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            description: None,
            state_type: StateType::Normal,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn start() -> Self {
        Self {
            id: "[*]".to_string(),
            description: None,
            state_type: StateType::Start,
        }
    }

    pub fn end() -> Self {
        Self {
            id: "[*]".to_string(),
            description: None,
            state_type: StateType::End,
        }
    }

    pub fn to_mermaid(&self) -> String {
        match self.state_type {
            StateType::Start | StateType::End => String::new(), // Start/End are rendered as transitions
            StateType::Normal => match &self.description {
                Some(desc) => format!("{}: {}", self.id, desc),
                None => self.id.clone(),
            },
        }
    }

    /// Get the state identifier for use in transitions
    pub fn state_id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StateType {
    #[default]
    Normal,
    Start,
    End,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_basic() {
        let state = State::new("Active");
        assert_eq!(state.to_mermaid(), "Active");
    }

    #[test]
    fn state_with_description() {
        let state = State::new("Active").with_description("The active state");
        assert_eq!(state.to_mermaid(), "Active: The active state");
    }

    #[test]
    fn state_start_end() {
        let start = State::start();
        let end = State::end();
        assert_eq!(start.state_id(), "[*]");
        assert_eq!(end.state_id(), "[*]");
    }
}
