use serde::{Deserialize, Serialize};

use super::Message;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logic {
    pub logic_type: LogicType,
    pub condition: String,
    pub messages: Vec<Message>,
    #[serde(default)]
    pub else_blocks: Vec<ElseBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElseBlock {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub messages: Vec<Message>,
}

impl Logic {
    pub fn new(logic_type: LogicType, condition: impl Into<String>) -> Self {
        Self {
            logic_type,
            condition: condition.into(),
            messages: Vec::new(),
            else_blocks: Vec::new(),
        }
    }

    pub fn alt(condition: impl Into<String>) -> Self {
        Self::new(LogicType::Alt, condition)
    }

    pub fn opt(condition: impl Into<String>) -> Self {
        Self::new(LogicType::Opt, condition)
    }

    pub fn loop_block(condition: impl Into<String>) -> Self {
        Self::new(LogicType::Loop, condition)
    }

    pub fn par(condition: impl Into<String>) -> Self {
        Self::new(LogicType::Par, condition)
    }

    pub fn critical(condition: impl Into<String>) -> Self {
        Self::new(LogicType::Critical, condition)
    }

    pub fn break_block(condition: impl Into<String>) -> Self {
        Self::new(LogicType::Break, condition)
    }

    pub fn with_message(mut self, message: Message) -> Self {
        self.messages.push(message);
        self
    }

    pub fn with_messages(mut self, messages: Vec<Message>) -> Self {
        self.messages = messages;
        self
    }

    pub fn with_else(mut self, messages: Vec<Message>) -> Self {
        self.else_blocks.push(ElseBlock {
            condition: None,
            messages,
        });
        self
    }

    pub fn with_else_condition(mut self, condition: impl Into<String>, messages: Vec<Message>) -> Self {
        self.else_blocks.push(ElseBlock {
            condition: Some(condition.into()),
            messages,
        });
        self
    }

    pub fn to_mermaid(&self) -> String {
        let mut output = String::new();

        // Opening
        let keyword = self.logic_type.keyword();
        output.push_str(&format!("{} {}\n", keyword, self.condition));

        // Main messages
        for msg in &self.messages {
            output.push_str(&format!("    {}\n", msg.to_mermaid()));
        }

        // Else blocks
        for else_block in &self.else_blocks {
            match &else_block.condition {
                Some(cond) => output.push_str(&format!("else {}\n", cond)),
                None => output.push_str("else\n"),
            }
            for msg in &else_block.messages {
                output.push_str(&format!("    {}\n", msg.to_mermaid()));
            }
        }

        output.push_str("end");
        output
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogicType {
    #[default]
    Alt, // Alternative paths
    Opt,      // Optional
    Loop,     // Loop
    Par,      // Parallel
    Critical, // Critical region
    Break,    // Break out of loop
}

impl LogicType {
    pub fn keyword(&self) -> &'static str {
        match self {
            Self::Alt => "alt",
            Self::Opt => "opt",
            Self::Loop => "loop",
            Self::Par => "par",
            Self::Critical => "critical",
            Self::Break => "break",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "alt" | "alternative" => Some(Self::Alt),
            "opt" | "optional" => Some(Self::Opt),
            "loop" => Some(Self::Loop),
            "par" | "parallel" => Some(Self::Par),
            "critical" => Some(Self::Critical),
            "break" => Some(Self::Break),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logic_alt() {
        let logic = Logic::alt("Success")
            .with_message(Message::new("A", "B").with_text("OK"))
            .with_else_condition("Failure", vec![Message::new("A", "B").with_text("Error")]);

        let mermaid = logic.to_mermaid();
        assert!(mermaid.contains("alt Success"));
        assert!(mermaid.contains("else Failure"));
        assert!(mermaid.contains("end"));
    }

    #[test]
    fn logic_loop() {
        let logic = Logic::loop_block("Every minute").with_message(Message::new("Server", "Client").with_text("ping"));

        let mermaid = logic.to_mermaid();
        assert!(mermaid.contains("loop Every minute"));
        // mermaid-py defaults to SolidArrow (->>)
        assert!(mermaid.contains("Server->>Client: ping"));
        assert!(mermaid.contains("end"));
    }

    #[test]
    fn logic_opt() {
        let logic = Logic::opt("Has cache").with_message(Message::new("Client", "Cache").with_text("get"));

        let mermaid = logic.to_mermaid();
        assert!(mermaid.contains("opt Has cache"));
    }

    #[test]
    fn logic_type_parse() {
        assert_eq!(LogicType::parse("alt"), Some(LogicType::Alt));
        assert_eq!(LogicType::parse("loop"), Some(LogicType::Loop));
        assert_eq!(LogicType::parse("invalid"), None);
    }
}
