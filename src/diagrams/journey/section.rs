use serde::{Deserialize, Serialize};

use super::Task;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub name: String,
    #[serde(default)]
    pub tasks: Vec<Task>,
}

impl Section {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            tasks: Vec::new(),
        }
    }

    pub fn with_task(mut self, task: Task) -> Self {
        self.tasks.push(task);
        self
    }

    pub fn with_tasks(mut self, tasks: Vec<Task>) -> Self {
        self.tasks = tasks;
        self
    }

    /// Add a simple task with just name and score
    pub fn task(mut self, name: impl Into<String>, score: u8) -> Self {
        self.tasks.push(Task::new(name, score));
        self
    }

    pub fn to_mermaid(&self) -> String {
        let mut output = format!("    section {}\n", self.name);
        for task in &self.tasks {
            output.push_str(&task.to_mermaid());
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_basic() {
        let section = Section::new("Onboarding").task("Sign up", 5).task("Verify email", 3);

        let mermaid = section.to_mermaid();
        assert!(mermaid.contains("section Onboarding"));
        assert!(mermaid.contains("Sign up: 5"));
        assert!(mermaid.contains("Verify email: 3"));
    }

    #[test]
    fn section_with_tasks() {
        let section = Section::new("Test").with_tasks(vec![Task::new("T1", 4), Task::new("T2", 5)]);

        assert_eq!(section.tasks.len(), 2);
    }
}
