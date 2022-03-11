mod cli;
mod helix;
mod tmi;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();
    println!("Hello, world!");
}
