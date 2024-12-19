mod solution;
mod util;

use std::{fs, process::exit, time::Instant};

use clap::{Parser, ValueEnum};
use seq_macro::seq;
use solution::Solution;
use util::*;

seq!(D in 1..=19 {
    #(mod day~D;)*

    #[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
    pub enum Day {
        /// Run all days' puzzles
        All,
        #(
            /// Run this day's puzzles
            Day~D,
        )*
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

    seq!(D in 1..=19 {
        let solvers: Vec<(usize, Box<dyn Solution>)> = match args.day {
            Day::All => vec![
                #((D, Box::new(day~D::Puzzle)),)*
            ],
            #(Day::Day~D => vec![(D, Box::new(day~D::Puzzle))],)*
        };
    });
    seq!(N in 1..=2 {
        for (day, solver) in solvers.into_iter() {
            let file_path = format!("{}/day{}.txt", args.input, day);
            let input = fs::read_to_string(&file_path).unwrap_or_else(|_| {
                println!("Day {day}: No input file found at {file_path}");
                exit(0);
            });
            if args.benchmark {
                #(
                    let mut part_time_~N = Vec::new();
                    part_time_~N.reserve(1000);
                )*
                let total_time = Instant::now();
                for num_eval in 1..=1000 {
                    #(
                        let start = Instant::now();
                        let _ = solver.part~N(&input);
                        part_time_~N.push(start.elapsed().as_micros() as f64);
                    )*
                    if num_eval >= 100 && total_time.elapsed().as_secs_f64() > 30.0 {
                        // Early exit if the benchmark is taking too long
                        // and we have enough data points
                        break;
                    }
                }
                #(
                    println!(
                        "Day {day} Part {} ({:.2} ± {:.2} us): {}",
                        N,
                        mean(&part_time_~N),
                        std_dev(&part_time_~N),
                        solver.part~N(&input)
                    );
                )*
            } else {
                #(
                    println!("Day {day} Part {}: {}", N, solver.part~N(&input));
                )*
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use seq_macro::seq;

    seq!(D in 1..=19 {
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
