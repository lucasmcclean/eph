use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[command(name = "eph")]
#[command(author, version, about)]
pub struct Add {
    title: String,
    #[arg(short, long, default_value = "default")]
    context: String,
    #[arg(short, long, default_value_t = 3)]
    priority: u8,
    #[arg(short, long, value_delimiter = ',')]
    tags: Vec<String>,
    #[arg(short, long)]
    description: Option<String>,
}

impl Add {
    pub fn run(&self) {
        println!("Adding {}", self.title);
        println!(
            "- description: {}",
            self.description.as_deref().unwrap_or("…")
        );
        println!("- tags: {:?}", self.tags);
        println!("- context: {}", self.context);
        println!("- priority: {}", self.priority);
    }
}
