use clap::Parser;

#[derive(Parser, Clone, Debug)]
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
        conflicts_with = "exclude_priorities",
        value_parser = parse_priority_range
    )]
    include_priorities: Vec<Vec<u8>>,
    #[arg(
        short = 'x',
        long,
        value_delimiter = ',',
        conflicts_with = "include_priorities",
        value_parser = parse_priority_range
    )]
    exclude_priorities: Vec<Vec<u8>>,
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
        println!("Listing all tasks in:");
        if !self.contexts.is_empty() {
            println!("- Contexts: {:?}", self.contexts);
        } else {
            println!("- All contexts");
        }
        if !self.include_priorities.is_empty() {
            println!(
                "- Including priorities: {:?}",
                normalize_priorities(&self.include_priorities)
            );
        }
        if !self.exclude_priorities.is_empty() {
            println!(
                "- Excluding priorities: {:?}",
                normalize_priorities(&self.exclude_priorities)
            );
        }
        if !self.include_tags.is_empty() {
            println!("- Including tags: {:?}", self.include_tags);
        }
        if !self.exclude_tags.is_empty() {
            println!("- Excluding tags: {:?}", self.exclude_tags);
        }
    }
}

fn parse_priority_range(s: &str) -> Result<Vec<u8>, String> {
    let err_msg_invalid_u8 = "priority must be between 0 and 255";

    if let Some((start, end)) = s.split_once('-') {
        let start: u8 = start.parse().map_err(|_| err_msg_invalid_u8)?;
        let end: u8 = end.parse().map_err(|_| err_msg_invalid_u8)?;

        if start > end {
            return Err("range must be ascending".into());
        }

        Ok((start..=end).collect())
    } else {
        let p: u8 = s.parse().map_err(|_| err_msg_invalid_u8)?;
        Ok(vec![p])
    }
}

fn normalize_priorities(v: &[Vec<u8>]) -> Vec<u8> {
    let mut p: Vec<u8> = v.iter().flatten().copied().collect();
    p.sort_unstable();
    p.dedup();
    p
}
