use thiserror::Error;

#[derive(Error, Debug)]
pub enum MermaidError {
    #[error("Failed to parse input file: {0}")]
    ParseError(String),

    #[error("Invalid diagram configuration: {0}")]
    ConfigError(String),

    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Render failed: {0}")]
    RenderFailed(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Clipboard error: {0}")]
    ClipboardError(String),

    #[error("JSON parse error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("YAML parse error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("TOML parse error: {0}")]
    TomlError(#[from] toml::de::Error),
}
