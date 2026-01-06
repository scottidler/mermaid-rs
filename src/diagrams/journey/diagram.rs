use serde::{Deserialize, Serialize};

use crate::core::{Config, Diagram, MermaidError};

use super::{Section, Task};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Journey {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    pub sections: Vec<Section>,
    #[serde(skip)]
    raw_mermaid: Option<String>,
}

impl Journey {
    pub fn builder() -> JourneyBuilder {
        JourneyBuilder::new()
    }

    pub fn from_raw_mermaid(script: String) -> Self {
        Self {
            title: None,
            sections: Vec::new(),
            raw_mermaid: Some(script),
        }
    }

    pub fn from_json(json: &str) -> Result<Self, MermaidError> {
        serde_json::from_str(json).map_err(|e| MermaidError::ParseError(e.to_string()))
    }

    pub fn from_yaml(yaml: &str) -> Result<Self, MermaidError> {
        serde_yaml::from_str(yaml).map_err(|e| MermaidError::ParseError(e.to_string()))
    }

    pub fn from_toml(toml: &str) -> Result<Self, MermaidError> {
        toml::from_str(toml).map_err(|e| MermaidError::ParseError(e.to_string()))
    }
}

impl Diagram for Journey {
    fn diagram_type(&self) -> &'static str {
        "journey"
    }

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn config(&self) -> Option<&Config> {
        None
    }

    fn to_mermaid(&self) -> String {
        if let Some(raw) = &self.raw_mermaid {
            return raw.clone();
        }

        let mut output = String::from("journey\n");

        if let Some(title) = &self.title {
            output.push_str(&format!("\ttitle {}\n", title));
        }

        for section in &self.sections {
            output.push_str(&section.to_mermaid());
        }

        output
    }
}

#[derive(Debug, Default)]
pub struct JourneyBuilder {
    title: Option<String>,
    sections: Vec<Section>,
    current_section: Option<Section>,
}

impl JourneyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Start a new section
    pub fn section(mut self, name: impl Into<String>) -> Self {
        // Save current section if any
        if let Some(section) = self.current_section.take() {
            self.sections.push(section);
        }
        self.current_section = Some(Section::new(name));
        self
    }

    /// Add a task to the current section
    pub fn task(mut self, name: impl Into<String>, score: u8) -> Self {
        if let Some(section) = &mut self.current_section {
            section.tasks.push(Task::new(name, score));
        } else {
            // Create default section if none exists
            let mut section = Section::new("Default");
            section.tasks.push(Task::new(name, score));
            self.current_section = Some(section);
        }
        self
    }

    /// Add a task with actors to the current section
    pub fn task_with_actors(mut self, name: impl Into<String>, score: u8, actors: Vec<String>) -> Self {
        if let Some(section) = &mut self.current_section {
            section.tasks.push(Task::new(name, score).with_actors(actors));
        }
        self
    }

    /// Add a pre-built section
    pub fn add_section(mut self, section: Section) -> Self {
        // Save current section if any
        if let Some(current) = self.current_section.take() {
            self.sections.push(current);
        }
        self.sections.push(section);
        self
    }

    pub fn build(mut self) -> Journey {
        // Don't forget the last section
        if let Some(section) = self.current_section.take() {
            self.sections.push(section);
        }

        Journey {
            title: self.title,
            sections: self.sections,
            raw_mermaid: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn journey_basic() {
        let journey = Journey::builder()
            .title("User Onboarding")
            .section("Registration")
            .task("Visit site", 5)
            .task("Sign up", 4)
            .section("Activation")
            .task("Verify email", 3)
            .task("Complete profile", 4)
            .build();

        let mermaid = journey.to_mermaid();
        assert!(mermaid.contains("journey"));
        assert!(mermaid.contains("title User Onboarding"));
        assert!(mermaid.contains("section Registration"));
        assert!(mermaid.contains("Visit site: 5"));
    }

    #[test]
    fn journey_from_json() {
        let json = r#"{
            "title": "Test Journey",
            "sections": [
                {
                    "name": "Section 1",
                    "tasks": [
                        {"name": "Task 1", "score": 5}
                    ]
                }
            ]
        }"#;

        let journey = Journey::from_json(json).unwrap();
        assert_eq!(journey.title, Some("Test Journey".to_string()));
        assert_eq!(journey.sections.len(), 1);
    }

    #[test]
    fn journey_from_yaml() {
        let yaml = r#"
title: Test Journey
sections:
  - name: Section 1
    tasks:
      - name: Task 1
        score: 5
"#;

        let journey = Journey::from_yaml(yaml).unwrap();
        assert_eq!(journey.title, Some("Test Journey".to_string()));
    }

    #[test]
    fn journey_raw_mermaid() {
        let raw = "journey\n    title Test\n    section S1\n        Task: 5";
        let journey = Journey::from_raw_mermaid(raw.to_string());
        assert_eq!(journey.to_mermaid(), raw);
    }
}
