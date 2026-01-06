use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Style {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_dasharray: Option<String>,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> Self {
        Self::default()
    }

    pub fn build(self) -> Self {
        self
    }

    pub fn fill(mut self, fill: impl Into<String>) -> Self {
        self.fill = Some(fill.into());
        self
    }

    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn stroke(mut self, stroke: impl Into<String>) -> Self {
        self.stroke = Some(stroke.into());
        self
    }

    pub fn stroke_width(mut self, width: impl Into<String>) -> Self {
        self.stroke_width = Some(width.into());
        self
    }

    pub fn stroke_dasharray(mut self, dasharray: impl Into<String>) -> Self {
        self.stroke_dasharray = Some(dasharray.into());
        self
    }

    pub fn to_css(&self) -> String {
        let mut parts = Vec::new();
        if let Some(fill) = &self.fill {
            parts.push(format!("fill:{}", fill));
        }
        if let Some(color) = &self.color {
            parts.push(format!("color:{}", color));
        }
        if let Some(stroke) = &self.stroke {
            parts.push(format!("stroke:{}", stroke));
        }
        if let Some(sw) = &self.stroke_width {
            parts.push(format!("stroke-width:{}", sw));
        }
        if let Some(sd) = &self.stroke_dasharray {
            parts.push(format!("stroke-dasharray:{}", sd));
        }
        parts.join(",")
    }

    pub fn is_empty(&self) -> bool {
        self.fill.is_none()
            && self.color.is_none()
            && self.stroke.is_none()
            && self.stroke_width.is_none()
            && self.stroke_dasharray.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn style_builder() {
        let style = Style::new().fill("#f9f").stroke("#333").stroke_width("2px");

        assert_eq!(style.fill, Some("#f9f".to_string()));
        assert_eq!(style.stroke, Some("#333".to_string()));
        assert_eq!(style.stroke_width, Some("2px".to_string()));
    }

    #[test]
    fn style_to_css() {
        let style = Style::new().fill("#f9f").stroke("#333");
        assert_eq!(style.to_css(), "fill:#f9f,stroke:#333");
    }

    #[test]
    fn style_is_empty() {
        assert!(Style::new().is_empty());
        assert!(!Style::new().fill("#f9f").is_empty());
    }
}
