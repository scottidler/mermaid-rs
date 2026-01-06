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

    pub fn to_mermaid(&self) -> String {
        let arrow = self.message_type.arrow();
        let mut output = String::new();

        // Main message line
        let msg_line = match &self.text {
            Some(text) => format!("{}{}{}: {}", self.from, arrow, self.to, text),
            None => format!("{}{}{}", self.from, arrow, self.to),
        };

        // Handle activation/deactivation
        if self.activate {
            output.push_str(&format!("activate {}\n    ", self.to));
        }

        output.push_str(&msg_line);

        if self.deactivate {
            output.push_str(&format!("\n    deactivate {}", self.to));
        }

        output
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MessageType {
    #[default]
    Solid, // ->
    Dotted,      // -->
    SolidArrow,  // ->>
    DottedArrow, // -->>
    SolidCross,  // -x
    DottedCross, // --x
    SolidOpen,   // -)
    DottedOpen,  // --)
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
        assert_eq!(msg.to_mermaid(), "Alice->Bob: Hello");
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
        assert_eq!(MessageType::parse("solid-arrow"), Some(MessageType::SolidArrow));
        assert_eq!(MessageType::parse("invalid"), None);
    }
}
