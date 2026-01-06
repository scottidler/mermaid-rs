use serde::{Deserialize, Serialize};

use super::{State, Transition};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrentState {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    pub regions: Vec<ConcurrentRegion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrentRegion {
    #[serde(default)]
    pub states: Vec<State>,
    #[serde(default)]
    pub transitions: Vec<Transition>,
}

impl ConcurrentRegion {
    pub fn new() -> Self {
        Self {
            states: Vec::new(),
            transitions: Vec::new(),
        }
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
}

impl Default for ConcurrentRegion {
    fn default() -> Self {
        Self::new()
    }
}

impl ConcurrentState {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: None,
            regions: Vec::new(),
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_region(mut self, region: ConcurrentRegion) -> Self {
        self.regions.push(region);
        self
    }

    pub fn to_mermaid(&self) -> String {
        let mut output = String::new();

        // Start concurrent state
        let title = self.title.as_deref().unwrap_or(&self.id);
        output.push_str(&format!("state \"{}\" as {} {{\n", title, self.id));

        // Render regions separated by --
        for (i, region) in self.regions.iter().enumerate() {
            if i > 0 {
                output.push_str("    --\n");
            }

            // Render states in region
            for state in &region.states {
                let state_mermaid = state.to_mermaid();
                if !state_mermaid.is_empty() {
                    output.push_str(&format!("    {}\n", state_mermaid));
                }
            }

            // Render transitions in region
            for transition in &region.transitions {
                output.push_str(&format!("    {}\n", transition.to_mermaid()));
            }
        }

        output.push_str("}\n");
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concurrent_basic() {
        let concurrent = ConcurrentState::new("Parallel")
            .with_title("Parallel Processing")
            .with_region(
                ConcurrentRegion::new()
                    .with_state_simple("StateA1")
                    .with_state_simple("StateA2")
                    .with_transition_simple("StateA1", "StateA2"),
            )
            .with_region(
                ConcurrentRegion::new()
                    .with_state_simple("StateB1")
                    .with_state_simple("StateB2")
                    .with_transition_simple("StateB1", "StateB2"),
            );

        let mermaid = concurrent.to_mermaid();
        assert!(mermaid.contains("state \"Parallel Processing\" as Parallel"));
        assert!(mermaid.contains("StateA1"));
        assert!(mermaid.contains("StateB1"));
        assert!(mermaid.contains("--")); // Region separator
        assert!(mermaid.contains("}"));
    }

    #[test]
    fn concurrent_with_start_end() {
        let concurrent = ConcurrentState::new("Work")
            .with_region(
                ConcurrentRegion::new()
                    .with_transition(Transition::from_start("A"))
                    .with_state_simple("A")
                    .with_transition(Transition::to_end("A")),
            )
            .with_region(
                ConcurrentRegion::new()
                    .with_transition(Transition::from_start("B"))
                    .with_state_simple("B")
                    .with_transition(Transition::to_end("B")),
            );

        let mermaid = concurrent.to_mermaid();
        // mermaid-py lowercases state IDs
        assert!(mermaid.contains("[*] --> a"));
        assert!(mermaid.contains("[*] --> b"));
    }
}
