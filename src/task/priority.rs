use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    High = 1,
    Medium = 2,
    Low = 3,
    Backlog = 4,
}

impl Priority {
    pub fn all() -> &'static [Priority] {
        &[
            Priority::High,
            Priority::Medium,
            Priority::Low,
            Priority::Backlog,
        ]
    }
}

impl FromStr for Priority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "1" | "h" | "hi" | "high" => Ok(Priority::High),
            "2" | "m" | "med" | "medium" => Ok(Priority::Medium),
            "3" | "l" | "lo" | "low" => Ok(Priority::Low),
            "4" | "b" | "back" | "backlog" => Ok(Priority::Backlog),
            _ => Err("priority must be one of [high, medium, low, backlog]".into()),
        }
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::High => write!(f, "high"),
            Priority::Medium => write!(f, "medium"),
            Priority::Low => write!(f, "low"),
            Priority::Backlog => write!(f, "backlog"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_all_valid_strings() {
        assert_eq!("1".parse::<Priority>().unwrap(), Priority::High);
        assert_eq!("h".parse::<Priority>().unwrap(), Priority::High);
        assert_eq!("hi".parse::<Priority>().unwrap(), Priority::High);
        assert_eq!("high".parse::<Priority>().unwrap(), Priority::High);
        assert_eq!("HIGH".parse::<Priority>().unwrap(), Priority::High);

        assert_eq!("2".parse::<Priority>().unwrap(), Priority::Medium);
        assert_eq!("m".parse::<Priority>().unwrap(), Priority::Medium);
        assert_eq!("med".parse::<Priority>().unwrap(), Priority::Medium);
        assert_eq!("medium".parse::<Priority>().unwrap(), Priority::Medium);

        assert_eq!("3".parse::<Priority>().unwrap(), Priority::Low);
        assert_eq!("l".parse::<Priority>().unwrap(), Priority::Low);
        assert_eq!("lo".parse::<Priority>().unwrap(), Priority::Low);
        assert_eq!("low".parse::<Priority>().unwrap(), Priority::Low);

        assert_eq!("4".parse::<Priority>().unwrap(), Priority::Backlog);
        assert_eq!("b".parse::<Priority>().unwrap(), Priority::Backlog);
        assert_eq!("back".parse::<Priority>().unwrap(), Priority::Backlog);
        assert_eq!("backlog".parse::<Priority>().unwrap(), Priority::Backlog);
    }

    #[test]
    fn parse_invalid_string_fails() {
        assert!("invalid".parse::<Priority>().is_err());
        assert!("5".parse::<Priority>().is_err());
        assert!("".parse::<Priority>().is_err());
    }

    #[test]
    fn ordering() {
        assert!(Priority::High < Priority::Medium);
        assert!(Priority::Medium < Priority::Low);
        assert!(Priority::Low < Priority::Backlog);
    }

    #[test]
    fn all_contains_four() {
        assert_eq!(Priority::all().len(), 4);
    }

    #[test]
    fn display() {
        assert_eq!(Priority::High.to_string(), "high");
        assert_eq!(Priority::Medium.to_string(), "medium");
        assert_eq!(Priority::Low.to_string(), "low");
        assert_eq!(Priority::Backlog.to_string(), "backlog");
    }

    #[test]
    fn display_and_fromstr_roundtrip() {
        for p in Priority::all() {
            let s = p.to_string();
            let back: Priority = s.parse().unwrap();
            assert_eq!(*p, back);
        }
    }

    #[test]
    fn numcode_roundtrip() {
        for (code, expected) in [
            ("1", Priority::High),
            ("2", Priority::Medium),
            ("3", Priority::Low),
            ("4", Priority::Backlog),
        ] {
            assert_eq!(code.parse::<Priority>().unwrap(), expected);
        }
    }
}
