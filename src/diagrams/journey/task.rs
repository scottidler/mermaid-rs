use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub score: u8, // 0-5 satisfaction score
    #[serde(default)]
    pub actors: Vec<String>,
}

impl Task {
    pub fn new(name: impl Into<String>, score: u8) -> Self {
        Self {
            name: name.into(),
            score: score.clamp(0, 5),
            actors: Vec::new(),
        }
    }

    pub fn with_actor(mut self, actor: impl Into<String>) -> Self {
        self.actors.push(actor.into());
        self
    }

    pub fn with_actors(mut self, actors: Vec<String>) -> Self {
        self.actors = actors;
        self
    }

    pub fn to_mermaid(&self) -> String {
        // Match mermaid-py format: always include actors field (even if empty)
        format!(
            "\t\t{}: {} : {}\n",
            self.name,
            self.score,
            self.actors.join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_basic() {
        let task = Task::new("Sign up", 5);
        let mermaid = task.to_mermaid();
        // Format: {name}: {score} : {actors} (actors empty)
        assert!(mermaid.contains("Sign up: 5 : "));
    }

    #[test]
    fn task_with_actors() {
        let task = Task::new("Login", 4).with_actor("User");
        let mermaid = task.to_mermaid();
        assert!(mermaid.contains("Login: 4 : User"));
    }

    #[test]
    fn task_score_clamped() {
        let task = Task::new("Test", 10); // Should clamp to 5
        assert_eq!(task.score, 5);
    }
}
