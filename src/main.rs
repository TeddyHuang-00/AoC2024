mod solution;
mod util;

use clap::{Parser, ValueEnum};
use seq_macro::seq;
use solution::Solution;
use std::{fs, process::exit, time::Instant};
use util::*;

seq!(D in 1..=5 {
    #(mod day~D;)*

    #[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
    pub enum Day {
        #(Day~D,)*
    }
});

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of the day to run
    #[arg(value_enum)]
    day: Day,

    /// Input file root path
    #[arg(short, long, default_value = "inputs")]
    input: String,

    /// Enable benchmarking
    #[arg(short, long)]
    benchmark: bool,
}

fn main() {
    let args: Args = Args::parse();
    let day_num = args.day as u8 + 1;

    seq!(D in 1..=5 {
        let solution: Box<dyn Solution> = match args.day {
            #(Day::Day~D => Box::new(day~D::Puzzle),)*
        };
    });
    let file_path = format!("{}/day{}.txt", args.input, day_num);
    let input = fs::read_to_string(&file_path).unwrap_or_else(|_| {
        println!("Day {day_num}: No input file found at {file_path}");
        exit(0);
    });
    seq!(N in 1..=2 {
        if args.benchmark {
            #(
                let mut part_time_~N = Vec::new();
                part_time_~N.reserve(1000);
            )*
            for _ in 0..1000 {
                #(
                    let start = Instant::now();
                    let _ = solution.part~N(&input);
                    part_time_~N.push(start.elapsed().as_micros() as f64);
                )*
            }
            #(
                println!(
                    "Day {day_num} Part {} ({:.2} ± {:.2} us): {}",
                    N,
                    mean(&part_time_~N),
                    std_dev(&part_time_~N),
                    solution.part~N(&input)
                );
            )*
        } else {
            #(
                println!("Day {day_num} Part {}: {}", N, solution.part~N(&input));
            )*
        }
    });
}

#[cfg(test)]
mod tests {
    use seq_macro::seq;

    seq!(D in 1..=5 {
        mod day_~D {
            use crate::day~D;
            use crate::solution::Solution;
            use seq_macro::seq;

            seq!(N in 1..=2 {
                #[test]
                fn test_part~N() {
                    let input = std::fs::read_to_string(format!("examples/inputs/day{}.txt", D)).unwrap();
                    let output = std::fs::read_to_string(format!("examples/outputs/day{}-p{}.txt", D, N)).unwrap();
                    let solution = day~D::Puzzle;
                    assert_eq!(solution.part~N(&input), output.trim());
                }
            });
        }
    });
}
