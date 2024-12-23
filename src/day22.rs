use std::collections::HashMap;

use rayon::prelude::*;

use crate::solution::Solution;

type Integer = u64;

#[derive(Debug, Clone, Copy)]
struct SecretNumber {
    value: Integer,
}

impl SecretNumber {
    fn new(value: Integer) -> Self {
        Self { value }
    }

    fn mix(&mut self, other: Integer) {
        self.value ^= other;
    }

    fn prune(&mut self) {
        self.value %= 16777216;
    }

    fn hash(&mut self) {
        self.mix(self.value << 6);
        self.prune();
        self.mix(self.value >> 5);
        self.prune();
        self.mix(self.value << 11);
        self.prune();
    }
}

impl From<SecretNumber> for Integer {
    fn from(number: SecretNumber) -> Self {
        number.value
    }
}

pub struct Puzzle;

impl Puzzle {
    fn parse_input(input: &str) -> Vec<SecretNumber> {
        input
            .lines()
            .map(|l| SecretNumber::new(l.parse().unwrap()))
            .collect()
    }
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let mut numbers = Self::parse_input(input);
        (0..2000).for_each(|_| numbers.iter_mut().for_each(SecretNumber::hash));
        numbers
            .into_iter()
            .map(Integer::from)
            .sum::<Integer>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        Self::parse_input(input)
            .into_par_iter()
            .map(|mut n| {
                (0..2000)
                    .fold(
                        (n.value % 10, 0, HashMap::new()),
                        |(last, mut key, mut cnt), i| {
                            n.hash();
                            let price = n.value % 10;
                            // Combine the last 4 values to calculate the key
                            key <<= 4;
                            key |= (price.wrapping_sub(last) & 0b1111) as u16;
                            // Skip the first 3 iterations as we need 4 values to calculate the key
                            if i >= 3 {
                                // Only store the price if the key is never seen before
                                cnt.entry(key).or_insert(price);
                            }
                            (price, key, cnt)
                        },
                    )
                    .2
            })
            .reduce(HashMap::new, |mut acc, x| {
                // Combine all possible values for each pattern
                x.into_iter().for_each(|(k, v)| {
                    acc.entry(k).and_modify(|e| *e += v).or_insert(v);
                });
                acc
            })
            .into_values()
            .max()
            .unwrap()
            .to_string()
    }
}
