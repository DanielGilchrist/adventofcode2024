use clap::{Parser, Subcommand};

mod commands;
mod utils;
use commands::{command::Command, day1::Day1, day2::Day2};

#[derive(Parser)]
#[command(about = "Advent of Code 2024")]
#[allow(clippy::upper_case_acronyms)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Day1 {
        #[arg(short, long, value_parser = part_value_parser)]
        part: u8,
    },
    Day2 {
        #[arg(short, long, value_parser = part_value_parser)]
        part: u8,
    },
}

fn main() {
    let cli = CLI::parse();
    let result = match cli.command {
        Commands::Day1 { part } => Day1::new(part).execute(),
        Commands::Day2 { part } => Day2::new(part).execute(),
    };

    println!("{}", result);
}

fn part_value_parser(val: &str) -> Result<u8, String> {
    match val.parse::<u8>() {
        Ok(1) | Ok(2) => Ok(val.parse().unwrap()),
        _ => Err("Part must be either 1 or 2".into()),
    }
}
