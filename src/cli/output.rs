use std::path::PathBuf;

use arboard::Clipboard;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use tokio::fs;

use crate::core::MermaidError;

#[derive(Debug, Clone)]
pub enum OutputTarget {
    File(PathBuf),
    Stdout,
    Clipboard,
    Browser,
}

pub struct OutputHandler {
    targets: Vec<OutputTarget>,
}

impl OutputHandler {
    pub fn new(file: Option<PathBuf>, stdout: bool, clipboard: bool, open_browser: bool) -> Self {
        let mut targets = Vec::new();

        if let Some(path) = file {
            targets.push(OutputTarget::File(path));
        }
        if stdout {
            targets.push(OutputTarget::Stdout);
        }
        if clipboard {
            targets.push(OutputTarget::Clipboard);
        }
        if open_browser {
            targets.push(OutputTarget::Browser);
        }

        // Default to stdout if nothing specified
        if targets.is_empty() {
            targets.push(OutputTarget::Stdout);
        }

        Self { targets }
    }

    pub fn targets(&self) -> &[OutputTarget] {
        &self.targets
    }

    pub async fn write_svg(&self, content: &str) -> Result<(), MermaidError> {
        for target in &self.targets {
            match target {
                OutputTarget::File(path) => {
                    fs::write(path, content).await?;
                }
                OutputTarget::Stdout => {
                    println!("{}", content);
                }
                OutputTarget::Clipboard => {
                    let mut clipboard = Clipboard::new()
                        .map_err(|e| MermaidError::ClipboardError(e.to_string()))?;
                    clipboard
                        .set_text(content)
                        .map_err(|e| MermaidError::ClipboardError(e.to_string()))?;
                }
                OutputTarget::Browser => {
                    // Write to temp file and open
                    let temp_path = std::env::temp_dir().join("mermaid-output.svg");
                    fs::write(&temp_path, content).await?;
                    open::that(&temp_path)?;
                }
            }
        }
        Ok(())
    }

    pub async fn write_png(&self, content: &[u8]) -> Result<(), MermaidError> {
        for target in &self.targets {
            match target {
                OutputTarget::File(path) => {
                    fs::write(path, content).await?;
                }
                OutputTarget::Stdout => {
                    use std::io::Write;
                    std::io::stdout().write_all(content)?;
                }
                OutputTarget::Clipboard => {
                    // PNG to clipboard requires image crate integration
                    // For now, warn the user
                    eprintln!("Warning: PNG clipboard not yet implemented");
                }
                OutputTarget::Browser => {
                    let temp_path = std::env::temp_dir().join("mermaid-output.png");
                    fs::write(&temp_path, content).await?;
                    open::that(&temp_path)?;
                }
            }
        }
        Ok(())
    }

    pub async fn write_mermaid(&self, content: &str) -> Result<(), MermaidError> {
        for target in &self.targets {
            match target {
                OutputTarget::File(path) => {
                    fs::write(path, content).await?;
                }
                OutputTarget::Stdout => {
                    println!("{}", content);
                }
                OutputTarget::Clipboard => {
                    let mut clipboard = Clipboard::new()
                        .map_err(|e| MermaidError::ClipboardError(e.to_string()))?;
                    clipboard
                        .set_text(content)
                        .map_err(|e| MermaidError::ClipboardError(e.to_string()))?;
                }
                OutputTarget::Browser => {
                    // Open mermaid.live with the diagram
                    let encoded = URL_SAFE_NO_PAD.encode(content.as_bytes());
                    let url = format!("https://mermaid.live/edit#base64:{}", encoded);
                    open::that(&url)?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_handler_default_to_stdout() {
        let handler = OutputHandler::new(None, false, false, false);
        assert_eq!(handler.targets().len(), 1);
        assert!(matches!(handler.targets()[0], OutputTarget::Stdout));
    }

    #[test]
    fn output_handler_multiple_targets() {
        let handler = OutputHandler::new(Some(PathBuf::from("test.svg")), true, true, false);
        assert_eq!(handler.targets().len(), 3);
    }
}
