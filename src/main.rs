mod day1;
mod day2;
mod solution;

use clap::{Parser, ValueEnum};
use solution::Solution;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Day {
    Day1,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of the day to run
    #[arg(value_enum)]
    day: Day,

    /// Input file root path
    #[arg(short, long, default_value = "inputs")]
    input: String,
}

fn main() {
    let args: Args = Args::parse();
    let day_num = args.day as u8 + 1;
    let solution: Box<dyn Solution> = match args.day {
        Day::Day1 => Box::new(day1::Day1),
    };
    fs::read_to_string(format!("{}/day{}-p1.txt", args.input, day_num))
        .and_then(|input| {
            println!("Day {day_num} Part 1: {}", solution.part1(&input));
            Ok(())
        })
        .unwrap_or_else(|_| {
            println!("Day {day_num} Part 1: No input file found");
        });

    fs::read_to_string(format!("{}/day{}-p2.txt", args.input, day_num))
        .and_then(|input| {
            println!("Day {day_num} Part 2: {}", solution.part2(&input));
            Ok(())
        })
        .unwrap_or_else(|_| {
            println!("Day {day_num} Part 2: No input file found");
        });
}
