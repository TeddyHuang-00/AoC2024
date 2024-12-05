mod day1;
mod day2;
mod day3;
mod day4;
mod solution;

use clap::{Parser, ValueEnum};
use solution::Solution;
use std::{fs, process::exit};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Day {
    Day1,
    Day2,
    Day3,
    Day4,
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
        Day::Day2 => Box::new(day2::Day2),
        Day::Day3 => Box::new(day3::Day3),
        Day::Day4 => Box::new(day4::Day4),
    };
    let input =
        fs::read_to_string(format!("{}/day{}.txt", args.input, day_num)).unwrap_or_else(|_| {
            println!("Day {day_num}: No input file found");
            exit(0);
        });
    println!("Day {day_num} Part 1: {}", solution.part1(&input));
    println!("Day {day_num} Part 2: {}", solution.part2(&input));
}
