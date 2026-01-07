use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub theme: Theme,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_variables: Option<ThemeVariables>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    #[default]
    Default,
    Forest,
    Dark,
    Neutral,
    Base,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Forest => "forest",
            Self::Dark => "dark",
            Self::Neutral => "neutral",
            Self::Base => "base",
        }
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Theme {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "default" => Ok(Self::Default),
            "forest" => Ok(Self::Forest),
            "dark" => Ok(Self::Dark),
            "neutral" => Ok(Self::Neutral),
            "base" => Ok(Self::Base),
            _ => Err(format!("Invalid theme: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Light,
    #[default]
    Dark,
}

impl Mode {
    pub fn theme(&self) -> Theme {
        match self {
            Self::Light => Theme::Default,
            Self::Dark => Theme::Dark,
        }
    }

    pub fn background_color(&self) -> Option<&'static str> {
        match self {
            Self::Light => None,
            Self::Dark => Some("#1e1e1e"),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "light" => Ok(Self::Light),
            "dark" => Ok(Self::Dark),
            _ => Err(format!("Invalid mode: {}. Use 'light' or 'dark'", s)),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeVariables {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tertiary_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_color: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub fn with_theme_variables(mut self, vars: ThemeVariables) -> Self {
        self.theme_variables = Some(vars);
        self
    }

    pub fn to_yaml(&self) -> String {
        let mut yaml = String::new();
        yaml.push_str(&format!("theme: {}\n", self.theme.as_str()));
        if let Some(vars) = &self.theme_variables {
            yaml.push_str("themeVariables:\n");
            if let Some(c) = &vars.primary_color {
                yaml.push_str(&format!("  primaryColor: \"{}\"\n", c));
            }
            if let Some(c) = &vars.secondary_color {
                yaml.push_str(&format!("  secondaryColor: \"{}\"\n", c));
            }
            if let Some(c) = &vars.tertiary_color {
                yaml.push_str(&format!("  tertiaryColor: \"{}\"\n", c));
            }
            if let Some(c) = &vars.primary_text_color {
                yaml.push_str(&format!("  primaryTextColor: \"{}\"\n", c));
            }
            if let Some(c) = &vars.line_color {
                yaml.push_str(&format!("  lineColor: \"{}\"\n", c));
            }
        }
        yaml
    }

    /// Generate %%{init}%% directive for mermaid.ink compatibility
    pub fn to_init_directive(&self) -> String {
        let mut parts = vec![format!("'theme': '{}'", self.theme.as_str())];

        if let Some(vars) = &self.theme_variables {
            let mut var_parts = Vec::new();
            if let Some(c) = &vars.primary_color {
                var_parts.push(format!("'primaryColor': '{}'", c));
            }
            if let Some(c) = &vars.secondary_color {
                var_parts.push(format!("'secondaryColor': '{}'", c));
            }
            if let Some(c) = &vars.tertiary_color {
                var_parts.push(format!("'tertiaryColor': '{}'", c));
            }
            if let Some(c) = &vars.primary_text_color {
                var_parts.push(format!("'primaryTextColor': '{}'", c));
            }
            if let Some(c) = &vars.line_color {
                var_parts.push(format!("'lineColor': '{}'", c));
            }
            if !var_parts.is_empty() {
                parts.push(format!("'themeVariables': {{{}}}", var_parts.join(", ")));
            }
        }

        format!("%%{{init: {{{}}}}}%%", parts.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_display() {
        assert_eq!(Theme::Default.to_string(), "default");
        assert_eq!(Theme::Forest.to_string(), "forest");
        assert_eq!(Theme::Dark.to_string(), "dark");
    }

    #[test]
    fn theme_from_str() {
        assert_eq!(Theme::from_str("forest").unwrap(), Theme::Forest);
        assert_eq!(Theme::from_str("DARK").unwrap(), Theme::Dark);
    }

    #[test]
    fn config_to_yaml() {
        let config = Config::new().with_theme(Theme::Forest);
        assert!(config.to_yaml().contains("theme: forest"));
    }
}
