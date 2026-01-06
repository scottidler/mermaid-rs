use serde::{Deserialize, Serialize};

use super::{State, Transition};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeState {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    pub states: Vec<State>,
    #[serde(default)]
    pub transitions: Vec<Transition>,
}

impl CompositeState {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: None,
            states: Vec::new(),
            transitions: Vec::new(),
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_state(mut self, state: State) -> Self {
        self.states.push(state);
        self
    }

    pub fn with_state_simple(mut self, id: impl Into<String>) -> Self {
        self.states.push(State::new(id));
        self
    }

    pub fn with_transition(mut self, transition: Transition) -> Self {
        self.transitions.push(transition);
        self
    }

    pub fn with_transition_simple(
        mut self,
        from: impl Into<String>,
        to: impl Into<String>,
    ) -> Self {
        self.transitions.push(Transition::new(from, to));
        self
    }

    pub fn to_mermaid(&self) -> String {
        let mut output = String::new();

        // Start composite state
        let title = self.title.as_deref().unwrap_or(&self.id);
        output.push_str(&format!("state \"{}\" as {} {{\n", title, self.id));

        // Render inner states
        for state in &self.states {
            let state_mermaid = state.to_mermaid();
            if !state_mermaid.is_empty() {
                output.push_str(&format!("    {}\n", state_mermaid));
            }
        }

        // Render inner transitions
        for transition in &self.transitions {
            output.push_str(&format!("    {}\n", transition.to_mermaid()));
        }

        output.push_str("}\n");
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composite_basic() {
        let composite = CompositeState::new("Parent")
            .with_title("Parent State")
            .with_state_simple("Child1")
            .with_state_simple("Child2")
            .with_transition_simple("Child1", "Child2");

        let mermaid = composite.to_mermaid();
        assert!(mermaid.contains("state \"Parent State\" as Parent"));
        // mermaid-py lowercases state IDs
        assert!(mermaid.contains("child1"));
        assert!(mermaid.contains("child2"));
        assert!(mermaid.contains("child1 --> child2"));
        assert!(mermaid.contains("}"));
    }

    #[test]
    fn composite_with_start_end() {
        let composite = CompositeState::new("Process")
            .with_transition(Transition::from_start("Init"))
            .with_state_simple("Init")
            .with_state_simple("Done")
            .with_transition_simple("Init", "Done")
            .with_transition(Transition::to_end("Done"));

        let mermaid = composite.to_mermaid();
        // mermaid-py lowercases state IDs
        assert!(mermaid.contains("[*] --> init"));
        assert!(mermaid.contains("done --> [*]"));
    }
}
