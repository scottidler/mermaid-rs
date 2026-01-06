use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    pub from: String,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl Transition {
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            label: None,
        }
    }

    pub fn from_start(to: impl Into<String>) -> Self {
        Self {
            from: "[*]".to_string(),
            to: to.into(),
            label: None,
        }
    }

    pub fn to_end(from: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: "[*]".to_string(),
            label: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn to_mermaid(&self) -> String {
        // mermaid-py lowercases state IDs (but not [*])
        let from = if self.from == "[*]" { self.from.clone() } else { self.from.to_lowercase() };
        let to = if self.to == "[*]" { self.to.clone() } else { self.to.to_lowercase() };
        match &self.label {
            Some(label) => format!("{} --> {} : {}", from, to, label),
            None => format!("{} --> {}", from, to),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub id: String,
    pub conditions: Vec<ChoiceCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceCondition {
    pub condition: String,
    pub target: String,
}

impl Choice {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            conditions: Vec::new(),
        }
    }

    pub fn with_condition(mut self, condition: impl Into<String>, target: impl Into<String>) -> Self {
        self.conditions.push(ChoiceCondition {
            condition: condition.into(),
            target: target.into(),
        });
        self
    }

    pub fn to_mermaid(&self) -> String {
        let mut output = format!("state {} <<choice>>\n", self.id);
        for cond in &self.conditions {
            output.push_str(&format!("    {} --> {}: {}\n", self.id, cond.target, cond.condition));
        }
        output
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fork {
    pub id: String,
    pub targets: Vec<String>,
}

impl Fork {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            targets: Vec::new(),
        }
    }

    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.targets.push(target.into());
        self
    }

    pub fn with_targets(mut self, targets: Vec<String>) -> Self {
        self.targets = targets;
        self
    }

    pub fn to_mermaid(&self) -> String {
        let mut output = format!("state {} <<fork>>\n", self.id);
        for target in &self.targets {
            output.push_str(&format!("    {} --> {}\n", self.id, target));
        }
        output
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Join {
    pub id: String,
    pub sources: Vec<String>,
    pub target: String,
}

impl Join {
    pub fn new(id: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            sources: Vec::new(),
            target: target.into(),
        }
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.sources.push(source.into());
        self
    }

    pub fn with_sources(mut self, sources: Vec<String>) -> Self {
        self.sources = sources;
        self
    }

    pub fn to_mermaid(&self) -> String {
        let mut output = format!("state {} <<join>>\n", self.id);
        for source in &self.sources {
            output.push_str(&format!("    {} --> {}\n", source, self.id));
        }
        output.push_str(&format!("    {} --> {}\n", self.id, self.target));
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transition_basic() {
        let t = Transition::new("A", "B");
        // mermaid-py lowercases state IDs
        assert_eq!(t.to_mermaid(), "a --> b");
    }

    #[test]
    fn transition_with_label() {
        let t = Transition::new("A", "B").with_label("event");
        assert_eq!(t.to_mermaid(), "a --> b : event");
    }

    #[test]
    fn transition_from_start() {
        let t = Transition::from_start("Init");
        // [*] stays as-is, but Init gets lowercased
        assert_eq!(t.to_mermaid(), "[*] --> init");
    }

    #[test]
    fn transition_to_end() {
        let t = Transition::to_end("Final");
        assert_eq!(t.to_mermaid(), "final --> [*]");
    }

    #[test]
    fn choice_basic() {
        let c = Choice::new("decide")
            .with_condition("yes", "StateA")
            .with_condition("no", "StateB");
        let mermaid = c.to_mermaid();
        assert!(mermaid.contains("<<choice>>"));
        assert!(mermaid.contains("decide --> StateA: yes"));
        assert!(mermaid.contains("decide --> StateB: no"));
    }

    #[test]
    fn fork_basic() {
        let f = Fork::new("fork_state")
            .with_target("Parallel1")
            .with_target("Parallel2");
        let mermaid = f.to_mermaid();
        assert!(mermaid.contains("<<fork>>"));
        assert!(mermaid.contains("fork_state --> Parallel1"));
        assert!(mermaid.contains("fork_state --> Parallel2"));
    }

    #[test]
    fn join_basic() {
        let j = Join::new("join_state", "Next")
            .with_source("Parallel1")
            .with_source("Parallel2");
        let mermaid = j.to_mermaid();
        assert!(mermaid.contains("<<join>>"));
        assert!(mermaid.contains("Parallel1 --> join_state"));
        assert!(mermaid.contains("join_state --> Next"));
    }
}
