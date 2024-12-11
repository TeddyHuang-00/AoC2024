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
                let _target = target;
                let mut target = HashSet::from([target]);
                numbers.iter().skip(1).rev().for_each(|n| {
                    target = target
                        .iter()
                        .flat_map(|t| {
                            [(*t >= *n).then(|| t - n), (*t % *n == 0).then(|| t / n)]
                                .into_iter()
                                .flatten()
                        })
                        .collect();
                });
                target.contains(numbers.first().unwrap()).then_some(_target)
            })
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let data = Self::parse_input(input);
        data.into_par_iter()
            .filter_map(|(target, numbers)| {
                let _target = target;
                let mut target = HashSet::from([target]);
                numbers.iter().skip(1).rev().for_each(|n| {
                    target = target
                        .iter()
                        .flat_map(|t| {
                            let base = 10usize.pow(n.ilog10() + 1);
                            [
                                (*t >= *n).then(|| t - n),
                                (*t % *n == 0).then(|| t / n),
                                (*t % base == *n).then(|| t / base),
                            ]
                            .into_iter()
                            .flatten()
                        })
                        .collect();
                });
                target.contains(numbers.first().unwrap()).then_some(_target)
            })
            .sum::<usize>()
            .to_string()
    }
}
