use clap::{Parser};
use marseille::modulate;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    plaintext: String
}


pub fn main() {
    let cli = Cli::parse();
    println!("{}", modulate(cli.plaintext.clone(), &" "))
}
