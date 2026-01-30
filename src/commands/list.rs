use std::sync::OnceLock;

use clap::Parser;

use crate::priority::Priority;

#[derive(Clone, Debug, Parser)]
#[command(name = "eph")]
#[command(author, version, about)]
pub struct List {
    #[arg(short, long)]
    contexts: Vec<String>,

    #[arg(
        short = 'p',
        long = "priorities",
        value_delimiter = ',',
        value_name = "PRIORITY[-PRIORITY][,...]",
        value_parser = parse_priority_ranges,
        help = "Include priorities 1–3 or ranges like low-high."
    )]
    _priority_ranges: Vec<Vec<Priority>>,

    #[arg(skip)]
    priorities: OnceLock<Vec<Priority>>,

    #[arg(short = 't', long, value_delimiter = ',')]
    include_tags: Vec<String>,
}

impl List {
    pub fn run(&self) {
        println!("list: {:?}", self.priorities())
    }

    pub fn priorities(&self) -> &[Priority] {
        self.priorities
            .get_or_init(|| normalize_priority_ranges(&self._priority_ranges))
    }
}

fn parse_priority_ranges(s: &str) -> Result<Vec<Priority>, String> {
    let s = s.trim();

    if let Some((a, b)) = s.split_once('-') {
        let start: Priority = a
            .parse()
            .map_err(|_| format!("'{}' is not one of [high, medium, low, backlog]", a))?;
        let end: Priority = b
            .parse()
            .map_err(|_| format!("'{}' is not one of [high, medium, low, backlog]", b))?;

        let (start, end) = (start.min(end), start.max(end));

        return Ok(Priority::all()
            .iter()
            .copied()
            .filter(|p| *p >= start && *p <= end)
            .collect());
    }

    s.parse()
        .map_err(|_| format!("'{}' is not one of [high, medium, low, backlog]", s))
        .map(|p| vec![p])
}

fn normalize_priority_ranges(priority_ranges: &[Vec<Priority>]) -> Vec<Priority> {
    let mut priorities: Vec<Priority> = priority_ranges.iter().flatten().copied().collect();
    priorities.sort_unstable();
    priorities.dedup();
    priorities
}
