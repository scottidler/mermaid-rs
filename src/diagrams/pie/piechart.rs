use serde::{Deserialize, Serialize};

use crate::core::{Config, Diagram, FromConfig, MermaidError, Theme};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PieData {
    pub label: String,
    pub value: f64,
}

impl PieData {
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PieChart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    pub show_data: bool,
    #[serde(default)]
    pub data: Vec<PieData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Config>,
    /// Raw mermaid passthrough (if set, ignores other fields)
    #[serde(skip)]
    raw_mermaid: Option<String>,
}

impl PieChart {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> PieChartBuilder {
        PieChartBuilder::new()
    }

    pub fn from_raw_mermaid(mermaid: String) -> Self {
        Self {
            raw_mermaid: Some(mermaid),
            ..Default::default()
        }
    }

    pub fn from_json(json: &str) -> Result<Self, MermaidError> {
        let chart: Self = serde_json::from_str(json)?;
        Ok(chart)
    }

    pub fn from_yaml(yaml: &str) -> Result<Self, MermaidError> {
        let chart: Self = serde_yaml::from_str(yaml)?;
        Ok(chart)
    }

    pub fn from_toml(toml: &str) -> Result<Self, MermaidError> {
        let chart: Self = toml::from_str(toml)?;
        Ok(chart)
    }
}

impl Diagram for PieChart {
    fn to_mermaid(&self) -> String {
        // If raw mermaid was provided, return it directly
        if let Some(raw) = &self.raw_mermaid {
            return raw.clone();
        }

        let mut output = String::new();

        // Start with pie keyword
        output.push_str("pie");

        // Add showData if enabled
        if self.show_data {
            output.push_str(" showData");
        }

        // Title goes in frontmatter, not on pie line (matches mermaid-py)
        output.push('\n');

        // Add data entries
        for entry in &self.data {
            output.push_str(&format!("\t\"{}\" : {}\n", entry.label, entry.value));
        }

        output
    }

    fn diagram_type(&self) -> &'static str {
        "pie"
    }

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn config(&self) -> Option<&Config> {
        self.config.as_ref()
    }
}

impl FromConfig for PieChart {
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
pub struct PieChartBuilder {
    title: Option<String>,
    show_data: bool,
    data: Vec<PieData>,
    config: Option<Config>,
}

impl PieChartBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn show_data(mut self, show: bool) -> Self {
        self.show_data = show;
        self
    }

    pub fn data(mut self, label: impl Into<String>, value: f64) -> Self {
        self.data.push(PieData::new(label, value));
        self
    }

    pub fn add_data(mut self, data: PieData) -> Self {
        self.data.push(data);
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

    pub fn build(self) -> PieChart {
        PieChart {
            title: self.title,
            show_data: self.show_data,
            data: self.data,
            config: self.config,
            raw_mermaid: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pie_chart_basic() {
        let chart = PieChart::builder()
            .title("Browser Market Share")
            .data("Chrome", 65.0)
            .data("Firefox", 20.0)
            .data("Safari", 10.0)
            .data("Other", 5.0)
            .build();

        let mermaid = chart.to_mermaid();
        assert!(mermaid.starts_with("pie\n"));
        // Title goes in frontmatter, not on pie line
        assert!(!mermaid.contains("title Browser Market Share"));
        assert!(mermaid.contains("\"Chrome\" : 65"));
        assert!(mermaid.contains("\"Firefox\" : 20"));
    }

    #[test]
    fn pie_chart_show_data() {
        let chart = PieChart::builder()
            .title("Test")
            .show_data(true)
            .data("A", 50.0)
            .data("B", 50.0)
            .build();

        let mermaid = chart.to_mermaid();
        assert!(mermaid.contains("pie showData"));
    }

    #[test]
    fn pie_chart_from_json() {
        let json = r#"{
            "title": "Test Chart",
            "show_data": true,
            "data": [
                {"label": "A", "value": 30},
                {"label": "B", "value": 70}
            ]
        }"#;

        let chart = PieChart::from_json(json).unwrap();
        assert_eq!(chart.title, Some("Test Chart".to_string()));
        assert!(chart.show_data);
        assert_eq!(chart.data.len(), 2);
    }

    #[test]
    fn pie_chart_from_yaml() {
        let yaml = r#"
title: Test Chart
show_data: true
data:
  - label: A
    value: 30
  - label: B
    value: 70
"#;

        let chart = PieChart::from_yaml(yaml).unwrap();
        assert_eq!(chart.title, Some("Test Chart".to_string()));
        assert!(chart.show_data);
        assert_eq!(chart.data.len(), 2);
    }

    #[test]
    fn pie_chart_raw_mermaid() {
        let raw = "pie\n\t\"A\" : 50\n\t\"B\" : 50";
        let chart = PieChart::from_raw_mermaid(raw.to_string());
        assert_eq!(chart.to_mermaid(), raw);
    }

    #[test]
    fn pie_chart_build_script_with_config() {
        let chart = PieChart::builder()
            .title("Test")
            .theme(Theme::Dark)
            .data("A", 100.0)
            .build();

        let script = chart.build_script();
        assert!(script.contains("---"));
        assert!(script.contains("theme: dark"));
    }
}
