use std::collections::HashSet;

use rayon::prelude::*;

use crate::solution::Solution;

pub struct Puzzle;

impl Puzzle {
    fn parse_input(input: &str) -> Vec<(usize, Vec<usize>)> {
        input
            .lines()
            .map(|s| {
                let (target, numbers) = s.split_once(": ").unwrap();
                let target = target.parse().unwrap();
                let numbers = numbers
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect();
                (target, numbers)
            })
            .collect()
    }
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let data = Self::parse_input(input);
        data.into_par_iter()
            .filter_map(|(target, numbers)| {
                let mut numbers = numbers.into_iter();
                let mut results = HashSet::from([numbers.next()?]);
                numbers.for_each(|n| {
                    results = results
                        .iter()
                        .flat_map(|r| [r + n, r * n].into_iter())
                        .filter(|&r| r <= target)
                        .collect();
                });
                results.contains(&target).then_some(target)
            })
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let data = Self::parse_input(input);
        data.into_par_iter()
            .filter_map(|(target, numbers)| {
                let mut numbers = numbers.into_iter();
                let mut results = HashSet::from([numbers.next()?]);
                numbers.for_each(|n| {
                    results = results
                        .iter()
                        .flat_map(|r| {
                            [r + n, r * n, r * 10usize.pow(n.ilog10() + 1) + n].into_iter()
                        })
                        .filter(|&r| r <= target)
                        .collect();
                });
                results.contains(&target).then_some(target)
            })
            .sum::<usize>()
            .to_string()
    }
}
