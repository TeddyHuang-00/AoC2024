use crate::solution::Solution;
use regex::Regex;
use std::collections::BTreeSet;

pub struct Day3;

impl Solution for Day3 {
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
