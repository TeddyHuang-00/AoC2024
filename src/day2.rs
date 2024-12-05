use crate::solution::Solution;

pub struct Puzzle;

impl Puzzle {
    fn parse_input(input: &str) -> Vec<Vec<i32>> {
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect()
            })
            .collect()
    }

    fn is_safe_sequence(sequence: &[i32]) -> bool {
        let diff = sequence
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<i32>>();
        // Check if all differences are either all positive or all negative
        (diff.iter().all(|&d| d > 0) || diff.iter().all(|&d| d < 0))
        // Check if all differences are between 1 and 3 inclusive
            && diff.iter().all(|d| d.abs() >= 1 && d.abs() <= 3)
    }
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let input = Self::parse_input(input);
        input
            .into_iter()
            .map(|row| Self::is_safe_sequence(&row))
            .filter(|&b| b)
            .count()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let input = Puzzle::parse_input(input);
        input
            .into_iter()
            .map(|row| {
                // Check if the row is already a safe sequence
                Self::is_safe_sequence(&row)
                    // Check if removing any element from the row results in a safe sequence
                    // NOTE: This is a brute force solution and is not efficient for large inputs, however we are facing a few elements in this puzzle so it's fine
                    || (0..row.len()).any(|i| {
                        let mut row = row.clone();
                        row.remove(i);
                        Self::is_safe_sequence(&row)
                    })
            })
            .filter(|&b| b)
            .count()
            .to_string()
    }
}
