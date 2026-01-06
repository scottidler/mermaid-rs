use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    #[default]
    #[serde(rename = "TB")]
    TopBottom,
    #[serde(rename = "BT")]
    BottomTop,
    #[serde(rename = "LR")]
    LeftRight,
    #[serde(rename = "RL")]
    RightLeft,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TopBottom => write!(f, "TB"),
            Self::BottomTop => write!(f, "BT"),
            Self::LeftRight => write!(f, "LR"),
            Self::RightLeft => write!(f, "RL"),
        }
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "TB" | "TD" => Ok(Self::TopBottom),
            "BT" => Ok(Self::BottomTop),
            "LR" => Ok(Self::LeftRight),
            "RL" => Ok(Self::RightLeft),
            _ => Err(format!("Invalid direction: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_display() {
        assert_eq!(Direction::TopBottom.to_string(), "TB");
        assert_eq!(Direction::BottomTop.to_string(), "BT");
        assert_eq!(Direction::LeftRight.to_string(), "LR");
        assert_eq!(Direction::RightLeft.to_string(), "RL");
    }

    #[test]
    fn direction_from_str() {
        assert_eq!(Direction::from_str("TB").unwrap(), Direction::TopBottom);
        assert_eq!(Direction::from_str("td").unwrap(), Direction::TopBottom);
        assert_eq!(Direction::from_str("BT").unwrap(), Direction::BottomTop);
        assert_eq!(Direction::from_str("LR").unwrap(), Direction::LeftRight);
        assert_eq!(Direction::from_str("RL").unwrap(), Direction::RightLeft);
    }

    #[test]
    fn direction_from_str_invalid() {
        assert!(Direction::from_str("XX").is_err());
    }
}
