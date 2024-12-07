use clap::{Parser, Subcommand};

mod commands;
use commands::{command::Command, day1::Day1};

#[derive(Parser)]
#[command(about = "Advent of Code 2024")]
#[allow(clippy::upper_case_acronyms)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Day1,
}

fn main() {
    let cli = CLI::parse();
    let result = match cli.command {
        Commands::Day1 => Day1.execute(),
    };

    println!("{}", result);
}
