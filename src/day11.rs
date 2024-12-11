use std::collections::HashMap;

use crate::solution::Solution;

pub struct Puzzle;

impl Puzzle {
    fn parse_input(input: &str) -> HashMap<u64, u64> {
        input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            // As each stone is processed independently, the order doesn't matter
            .fold(HashMap::new(), |mut acc, x| {
                acc.entry(x).and_modify(|x| *x += 1).or_insert(1);
                acc
            })
    }
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let mut stones = Self::parse_input(input);
        (0..25).for_each(|_| {
            stones = stones.iter().fold(HashMap::new(), |mut acc, (&x, &cnt)| {
                match x {
                    // If the number is 0, turn it into 1
                    0 => vec![1],
                    // If the number has an even number of digits, split it into two numbers
                    // NOTE: numbers with an even number of digits are in range [10, 99], [1000, 9999], ...
                    //       so its log10 is always odd
                    n if n.ilog10() % 2 == 1 => {
                        let base = 10u64.pow((x.ilog10() + 1) / 2);
                        vec![x / base, x % base]
                    }
                    // Otherwise, multiply it by 2024
                    _ => vec![x * 2024],
                }
                .iter()
                .for_each(|&x| {
                    acc.entry(x).and_modify(|x| *x += cnt).or_insert(cnt);
                });
                acc
            });
        });
        stones.into_values().sum::<u64>().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut stones = Self::parse_input(input);
        (0..75).for_each(|_| {
            stones = stones.iter().fold(HashMap::new(), |mut acc, (&x, &cnt)| {
                match x {
                    // If the number is 0, turn it into 1
                    0 => vec![1],
                    // If the number has an even number of digits, split it into two numbers
                    // NOTE: numbers with an even number of digits are in range [10, 99], [1000, 9999], ...
                    //       so its log10 is always odd
                    n if n.ilog10() % 2 == 1 => {
                        let base = 10u64.pow((x.ilog10() + 1) / 2);
                        vec![x / base, x % base]
                    }
                    // Otherwise, multiply it by 2024
                    _ => vec![x * 2024],
                }
                .iter()
                .for_each(|&x| {
                    acc.entry(x).and_modify(|x| *x += cnt).or_insert(cnt);
                });
                acc
            });
        });
        stones.into_values().sum::<u64>().to_string()
    }
}
