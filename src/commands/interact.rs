use clap::Parser;

const DEFAULT_CONTEXT: &str = "default";

#[derive(Clone, Debug, Parser)]
#[command(name = "eph")]
#[command(author, version, about)]
pub struct Interact {
    #[arg(short, long, default_value = DEFAULT_CONTEXT)]
    context: String,
}

impl Default for Interact {
    fn default() -> Self {
        Interact {
            context: DEFAULT_CONTEXT.to_string(),
        }
    }
}

impl Interact {
    pub fn run(self) {}
}
