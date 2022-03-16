mod cli;
mod config;
mod helix;
mod tmi;

use clap::Parser;
use cli::Args;
use config::Config;

fn main() {
    let config = Config::read();
    let args = Args::parse();
    println!("Hello, world!");
}
