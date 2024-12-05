use crate::solution::Solution;
use regex::Regex;

pub struct Puzzle;

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        pattern
            .captures_iter(input)
            .map(|m| {
                // The first capture group is the whole match, so we skip it
                let a = m[1].parse::<u32>().unwrap();
                let b = m[2].parse::<u32>().unwrap();
                a * b
            })
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut enabled = true;
        let pattern = Regex::new(r"(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))").unwrap();
        pattern
            .captures_iter(input)
            .map(|m| {
                // First three letters of the match
                let start = &m.get(0).unwrap().as_str()[..3];
                enabled = match start {
                    "mul" => enabled,
                    "do(" => true,
                    "don" => false,
                    _ => unreachable!(),
                };
                if start == "mul" && enabled {
                    // The first capture group is the whole match, so we skip it
                    let a = m[2].parse::<u32>().unwrap();
                    let b = m[3].parse::<u32>().unwrap();
                    a * b
                } else {
                    0
                }
            })
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use util::*;

    fn read(file_path: String) -> String {
        fs::read_to_string(file_path).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            Puzzle.part1(&read(format!("{}/{}.txt", crate::RIN, stem!()))),
            read(format!("{}/{}-p1.txt", crate::ROUT, stem!()))
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Puzzle.part2(&read(format!("{}/{}.txt", crate::RIN, stem!()))),
            read(format!("{}/{}-p2.txt", crate::ROUT, stem!()))
        );
    }
}
