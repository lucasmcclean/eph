use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
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
