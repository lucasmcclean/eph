use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[command(name = "eph")]
#[command(author, version, about)]
pub struct Done {
    identifier: String,
}

impl Done {
    pub fn run(&self) {
        println!("Marked {} as done", self.identifier);
    }
}
