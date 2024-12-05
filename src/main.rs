mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod solution;

use clap::{Parser, ValueEnum};
use solution::Solution;
use std::{fs, process::exit};

/// Example input file root path
pub const RIN: &str = "examples/inputs";
/// Example output file root path
pub const ROUT: &str = "examples/outputs";

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Day {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
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
        Day::Day1 => Box::new(day1::Puzzle),
        Day::Day2 => Box::new(day2::Puzzle),
        Day::Day3 => Box::new(day3::Puzzle),
        Day::Day4 => Box::new(day4::Puzzle),
        Day::Day5 => Box::new(day5::Puzzle),
    };
    let input =
        fs::read_to_string(format!("{}/day{}.txt", args.input, day_num)).unwrap_or_else(|_| {
            println!("Day {day_num}: No input file found");
            exit(0);
        });
    println!("Day {day_num} Part 1: {}", solution.part1(&input));
    println!("Day {day_num} Part 2: {}", solution.part2(&input));
}

#[cfg(test)]
mod tests {
    use util::stem;

    #[test]
    fn test_file_stem_macro() {
        assert_eq!(stem!(), "main");
    }
}
