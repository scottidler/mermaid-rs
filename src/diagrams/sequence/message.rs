use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub from: String,
    pub to: String,
    #[serde(default, rename = "type")]
    pub message_type: MessageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default)]
    pub activate: bool,
    #[serde(default)]
    pub deactivate: bool,
    /// Use shorthand activation syntax (->>+ / -->>-)
    #[serde(default)]
    pub shorthand_activation: bool,
}

impl Message {
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            message_type: MessageType::default(),
            text: None,
            activate: false,
            deactivate: false,
            shorthand_activation: false,
        }
    }

    pub fn with_type(mut self, message_type: MessageType) -> Self {
        self.message_type = message_type;
        self
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn activate(mut self) -> Self {
        self.activate = true;
        self
    }

    pub fn deactivate(mut self) -> Self {
        self.deactivate = true;
        self
    }

    /// Use shorthand activation syntax (->>+ instead of separate activate statement)
    pub fn with_shorthand_activation(mut self) -> Self {
        self.shorthand_activation = true;
        self
    }

    pub fn to_mermaid(&self) -> String {
        let arrow = self.message_type.arrow();
        let mut output = String::new();

        // Build arrow with optional shorthand activation/deactivation
        let arrow_with_activation = if self.shorthand_activation {
            if self.activate && self.deactivate {
                // Both activate and deactivate: ->>+- (unusual but possible)
                format!("{}+-", arrow)
            } else if self.activate {
                format!("{}+", arrow)
            } else if self.deactivate {
                format!("{}-", arrow)
            } else {
                arrow.to_string()
            }
        } else {
            arrow.to_string()
        };

        // Handle non-shorthand activation (separate line before)
        if !self.shorthand_activation && self.activate {
            output.push_str(&format!("activate {}\n    ", self.to));
        }

        // Main message line
        let msg_line = match &self.text {
            Some(text) => format!(
                "{}{}{}: {}",
                self.from, arrow_with_activation, self.to, text
            ),
            None => format!("{}{}{}", self.from, arrow_with_activation, self.to),
        };

        output.push_str(&msg_line);

        // Handle non-shorthand deactivation (separate line after)
        if !self.shorthand_activation && self.deactivate {
            output.push_str(&format!("\n    deactivate {}", self.to));
        }

        output
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MessageType {
    Solid,  // ->
    Dotted, // -->
    #[default]
    SolidArrow, // ->>
    DottedArrow, // -->>
    SolidCross, // -x
    DottedCross, // --x
    SolidOpen, // -)
    DottedOpen, // --)
}

impl MessageType {
    pub fn arrow(&self) -> &'static str {
        match self {
            Self::Solid => "->",
            Self::Dotted => "-->",
            Self::SolidArrow => "->>",
            Self::DottedArrow => "-->>",
            Self::SolidCross => "-x",
            Self::DottedCross => "--x",
            Self::SolidOpen => "-)",
            Self::DottedOpen => "--)",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "solid" | "sync" => Some(Self::Solid),
            "dotted" | "reply" => Some(Self::Dotted),
            "solid-arrow" | "solidarrow" | "async" => Some(Self::SolidArrow),
            "dotted-arrow" | "dottedarrow" | "async-reply" => Some(Self::DottedArrow),
            "solid-cross" | "solidcross" => Some(Self::SolidCross),
            "dotted-cross" | "dottedcross" => Some(Self::DottedCross),
            "solid-open" | "solidopen" => Some(Self::SolidOpen),
            "dotted-open" | "dottedopen" => Some(Self::DottedOpen),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_basic() {
        let msg = Message::new("Alice", "Bob").with_text("Hello");
        assert_eq!(msg.to_mermaid(), "Alice->>Bob: Hello");
    }

    #[test]
    fn message_arrow_types() {
        assert_eq!(MessageType::Solid.arrow(), "->");
        assert_eq!(MessageType::Dotted.arrow(), "-->");
        assert_eq!(MessageType::SolidArrow.arrow(), "->>");
        assert_eq!(MessageType::DottedArrow.arrow(), "-->>");
        assert_eq!(MessageType::SolidCross.arrow(), "-x");
        assert_eq!(MessageType::DottedCross.arrow(), "--x");
    }

    #[test]
    fn message_with_type() {
        let msg = Message::new("A", "B")
            .with_type(MessageType::SolidArrow)
            .with_text("async call");
        assert_eq!(msg.to_mermaid(), "A->>B: async call");
    }

    #[test]
    fn message_type_parse() {
        assert_eq!(MessageType::parse("solid"), Some(MessageType::Solid));
        assert_eq!(
            MessageType::parse("solid-arrow"),
            Some(MessageType::SolidArrow)
        );
        assert_eq!(MessageType::parse("invalid"), None);
    }

    #[test]
    fn message_shorthand_activate() {
        let msg = Message::new("Alice", "Bob")
            .with_text("request")
            .activate()
            .with_shorthand_activation();
        assert_eq!(msg.to_mermaid(), "Alice->>+Bob: request");
    }

    #[test]
    fn message_shorthand_deactivate() {
        let msg = Message::new("Bob", "Alice")
            .with_text("response")
            .deactivate()
            .with_shorthand_activation();
        assert_eq!(msg.to_mermaid(), "Bob->>-Alice: response");
    }

    #[test]
    fn message_shorthand_dotted_activate() {
        let msg = Message::new("Alice", "Bob")
            .with_type(MessageType::DottedArrow)
            .with_text("async call")
            .activate()
            .with_shorthand_activation();
        assert_eq!(msg.to_mermaid(), "Alice-->>+Bob: async call");
    }

    #[test]
    fn message_cross_type() {
        let msg = Message::new("A", "B")
            .with_type(MessageType::SolidCross)
            .with_text("failed");
        assert_eq!(msg.to_mermaid(), "A-xB: failed");
    }

    #[test]
    fn message_open_type() {
        let msg = Message::new("A", "B")
            .with_type(MessageType::SolidOpen)
            .with_text("async");
        assert_eq!(msg.to_mermaid(), "A-)B: async");
    }
}
