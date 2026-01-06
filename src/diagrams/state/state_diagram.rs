use serde::{Deserialize, Serialize};

use crate::core::{Config, Diagram, Direction, FromConfig, MermaidError, Theme};

use super::{Choice, CompositeState, ConcurrentState, Fork, Join, State, Transition};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StateDiagram {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    pub direction: Direction,
    #[serde(default)]
    pub states: Vec<State>,
    #[serde(default)]
    pub transitions: Vec<Transition>,
    #[serde(default)]
    pub choices: Vec<Choice>,
    #[serde(default)]
    pub forks: Vec<Fork>,
    #[serde(default)]
    pub joins: Vec<Join>,
    #[serde(default)]
    pub composites: Vec<CompositeState>,
    #[serde(default)]
    pub concurrents: Vec<ConcurrentState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Config>,
    /// Raw mermaid passthrough (if set, ignores other fields)
    #[serde(skip)]
    raw_mermaid: Option<String>,
}

impl StateDiagram {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> StateDiagramBuilder {
        StateDiagramBuilder::new()
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
}

impl Diagram for StateDiagram {
    fn to_mermaid(&self) -> String {
        // If raw mermaid was provided, return it directly
        if let Some(raw) = &self.raw_mermaid {
            return raw.clone();
        }

        let mut output = String::new();

        // Start with stateDiagram-v2 and direction
        output.push_str("stateDiagram-v2\n");
        output.push_str(&format!("    direction {}\n", self.direction));

        // Render states
        for state in &self.states {
            let state_mermaid = state.to_mermaid();
            if !state_mermaid.is_empty() {
                output.push_str(&format!("    {}\n", state_mermaid));
            }
        }

        // Render composite states
        for composite in &self.composites {
            // Indent composite output
            for line in composite.to_mermaid().lines() {
                output.push_str(&format!("    {}\n", line));
            }
        }

        // Render concurrent states
        for concurrent in &self.concurrents {
            // Indent concurrent output
            for line in concurrent.to_mermaid().lines() {
                output.push_str(&format!("    {}\n", line));
            }
        }

        // Render choices
        for choice in &self.choices {
            for line in choice.to_mermaid().lines() {
                output.push_str(&format!("    {}\n", line));
            }
        }

        // Render forks
        for fork in &self.forks {
            for line in fork.to_mermaid().lines() {
                output.push_str(&format!("    {}\n", line));
            }
        }

        // Render joins
        for join in &self.joins {
            for line in join.to_mermaid().lines() {
                output.push_str(&format!("    {}\n", line));
            }
        }

        // Render transitions
        for transition in &self.transitions {
            output.push_str(&format!("    {}\n", transition.to_mermaid()));
        }

        output
    }

    fn diagram_type(&self) -> &'static str {
        "stateDiagram-v2"
    }

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn config(&self) -> Option<&Config> {
        self.config.as_ref()
    }
}

impl FromConfig for StateDiagram {
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
pub struct StateDiagramBuilder {
    title: Option<String>,
    direction: Direction,
    states: Vec<State>,
    transitions: Vec<Transition>,
    choices: Vec<Choice>,
    forks: Vec<Fork>,
    joins: Vec<Join>,
    composites: Vec<CompositeState>,
    concurrents: Vec<ConcurrentState>,
    config: Option<Config>,
}

impl StateDiagramBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn state(mut self, state: State) -> Self {
        self.states.push(state);
        self
    }

    pub fn state_simple(mut self, id: impl Into<String>) -> Self {
        self.states.push(State::new(id));
        self
    }

    pub fn state_with_description(mut self, id: impl Into<String>, description: impl Into<String>) -> Self {
        self.states.push(State::new(id).with_description(description));
        self
    }

    pub fn transition(mut self, transition: Transition) -> Self {
        self.transitions.push(transition);
        self
    }

    pub fn transition_simple(mut self, from: impl Into<String>, to: impl Into<String>) -> Self {
        self.transitions.push(Transition::new(from, to));
        self
    }

    pub fn transition_with_label(
        mut self,
        from: impl Into<String>,
        to: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        self.transitions.push(Transition::new(from, to).with_label(label));
        self
    }

    pub fn from_start(mut self, to: impl Into<String>) -> Self {
        self.transitions.push(Transition::from_start(to));
        self
    }

    pub fn to_end(mut self, from: impl Into<String>) -> Self {
        self.transitions.push(Transition::to_end(from));
        self
    }

    pub fn choice(mut self, choice: Choice) -> Self {
        self.choices.push(choice);
        self
    }

    pub fn fork(mut self, fork: Fork) -> Self {
        self.forks.push(fork);
        self
    }

    pub fn join(mut self, join: Join) -> Self {
        self.joins.push(join);
        self
    }

    pub fn composite(mut self, composite: CompositeState) -> Self {
        self.composites.push(composite);
        self
    }

    pub fn concurrent(mut self, concurrent: ConcurrentState) -> Self {
        self.concurrents.push(concurrent);
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

    pub fn build(self) -> StateDiagram {
        StateDiagram {
            title: self.title,
            direction: self.direction,
            states: self.states,
            transitions: self.transitions,
            choices: self.choices,
            forks: self.forks,
            joins: self.joins,
            composites: self.composites,
            concurrents: self.concurrents,
            config: self.config,
            raw_mermaid: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_diagram_basic() {
        let diagram = StateDiagram::builder()
            .state_simple("Active")
            .state_simple("Inactive")
            .from_start("Inactive")
            .transition_with_label("Inactive", "Active", "activate")
            .transition_with_label("Active", "Inactive", "deactivate")
            .to_end("Inactive")
            .build();

        let mermaid = diagram.to_mermaid();
        assert!(mermaid.contains("stateDiagram-v2"));
        assert!(mermaid.contains("[*] --> Inactive"));
        assert!(mermaid.contains("Inactive --> Active: activate"));
        assert!(mermaid.contains("Active --> Inactive: deactivate"));
        assert!(mermaid.contains("Inactive --> [*]"));
    }

    #[test]
    fn state_diagram_with_direction() {
        let diagram = StateDiagram::builder()
            .direction(Direction::LeftRight)
            .state_simple("A")
            .state_simple("B")
            .transition_simple("A", "B")
            .build();

        let mermaid = diagram.to_mermaid();
        assert!(mermaid.contains("direction LR"));
    }

    #[test]
    fn state_diagram_with_choice() {
        let diagram = StateDiagram::builder()
            .state_simple("Check")
            .state_simple("Success")
            .state_simple("Failure")
            .transition_simple("Check", "decide")
            .choice(
                Choice::new("decide")
                    .with_condition("valid", "Success")
                    .with_condition("invalid", "Failure"),
            )
            .build();

        let mermaid = diagram.to_mermaid();
        assert!(mermaid.contains("<<choice>>"));
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
        assert!(mermaid.contains("Child1"));
        assert!(mermaid.contains("}"));
    }

    #[test]
    fn state_diagram_from_json() {
        let json = r#"{
            "direction": "TB",
            "states": [
                {"id": "Active"},
                {"id": "Inactive"}
            ],
            "transitions": [
                {"from": "[*]", "to": "Inactive"},
                {"from": "Inactive", "to": "Active", "label": "start"},
                {"from": "Active", "to": "[*]"}
            ]
        }"#;

        let diagram = StateDiagram::from_json(json).unwrap();
        assert_eq!(diagram.states.len(), 2);
        assert_eq!(diagram.transitions.len(), 3);
    }

    #[test]
    fn state_diagram_from_yaml() {
        let yaml = r#"
title: Order State
direction: LR
states:
  - id: Pending
    description: Waiting for payment
  - id: Paid
  - id: Shipped
  - id: Delivered
transitions:
  - from: "[*]"
    to: Pending
  - from: Pending
    to: Paid
    label: payment received
  - from: Paid
    to: Shipped
    label: shipped
  - from: Shipped
    to: Delivered
    label: delivered
  - from: Delivered
    to: "[*]"
"#;

        let diagram = StateDiagram::from_yaml(yaml).unwrap();
        assert_eq!(diagram.title, Some("Order State".to_string()));
        assert_eq!(diagram.states.len(), 4);
        assert_eq!(diagram.transitions.len(), 5);
    }

    #[test]
    fn state_diagram_raw_mermaid() {
        let raw = "stateDiagram-v2\n    [*] --> Active";
        let diagram = StateDiagram::from_raw_mermaid(raw.to_string());
        assert_eq!(diagram.to_mermaid(), raw);
    }
}
