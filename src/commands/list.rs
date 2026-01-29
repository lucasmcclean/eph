use clap::Parser;

use crate::priority::Priority;

#[derive(Clone, Debug, Parser)]
#[command(name = "eph")]
#[command(author, version, about)]
pub struct List {
    // If no contexts are provided, default to all
    #[arg(short, long)]
    contexts: Vec<String>,

    #[arg(
        short = 'p',
        long,
        value_delimiter = ',',
        value_name = "PRIORITY[-PRIORITY][,PRIORITY[-PRIORITY]...]",
        value_parser = parse_priority_or_range,
        conflicts_with = "exclude_priorities",
        help = "Include priorities 1–3 or ranges like low-high. Multiple
                priorities or ranges can be comma-separated."
    )]
    include_priorities: Vec<Vec<Priority>>,

    #[arg(
        short = 'x',
        long,
        value_delimiter = ',',
        value_name = "PRIORITY[-PRIORITY][,PRIORITY[-PRIORITY]...]",
        value_parser = parse_priority_or_range,
        conflicts_with = "include_priorities",
        help = "Exclude priorities 1–3 or ranges like low-high. Multiple
                priorities or ranges can be comma-separated."
    )]
    exclude_priorities: Vec<Vec<Priority>>,

    #[arg(
        short = 't',
        long,
        value_delimiter = ',',
        conflicts_with = "exclude_tags"
    )]
    include_tags: Vec<String>,

    #[arg(
        short = 'e',
        long,
        value_delimiter = ',',
        conflicts_with = "include_tags"
    )]
    exclude_tags: Vec<String>,
}

impl List {
    pub fn run(&self) {
        let include_priorities = normalize(&self.include_priorities);
        let exclude_priorities = normalize(&self.exclude_priorities);
        println!("Listing all tasks in:");
        if !self.contexts.is_empty() {
            println!("- Contexts: {:?}", self.contexts);
        } else {
            println!("- All contexts");
        }
        if !self.include_priorities.is_empty() {
            println!("- Including priorities: {:?}", include_priorities);
        }
        if !self.exclude_priorities.is_empty() {
            println!("- Excluding priorities: {:?}", exclude_priorities);
        }
        if !self.include_tags.is_empty() {
            println!("- Including tags: {:?}", self.include_tags);
        }
        if !self.exclude_tags.is_empty() {
            println!("- Excluding tags: {:?}", self.exclude_tags);
        }
    }
}

fn parse_priority_or_range(s: &str) -> Result<Vec<Priority>, String> {
    let s = s.trim();
    if let Some((start, end)) = s.split_once('-') {
        let (start, end) = {
            let mut start: Priority = start.parse()?;
            let mut end: Priority = end.parse()?;
            if start > end {
                (start, end) = (end, start)
            }
            (start, end)
        };

        Ok(Priority::all()
            .iter()
            .copied()
            .filter(|p| *p >= start && *p <= end)
            .collect())
    } else {
        Ok(vec![s.parse()?])
    }
}

fn normalize(priorities: &[Vec<Priority>]) -> Vec<Priority> {
    let mut p: Vec<Priority> = priorities.iter().flatten().copied().collect();
    p.sort();
    p.dedup();
    p
}
