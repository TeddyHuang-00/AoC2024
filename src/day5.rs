use std::{cmp::Ordering, collections::HashMap};

use crate::solution::Solution;

type OrderingMap = HashMap<(usize, usize), Ordering>;
type Update = Vec<usize>;

pub struct Puzzle;

impl Puzzle {
    fn parse_input(input: &str) -> (OrderingMap, Vec<Update>) {
        let parts: Vec<&str> = input.split("\n\n").collect();
        let (rules, updates) = match parts.as_slice() {
            &[rules, updates] => (rules, updates),
            _ => panic!("Invalid input"),
        };
        let rules = rules
            .lines()
            .map(|line| {
                let (first, second) = line.split_once('|').unwrap();
                (
                    first.parse::<usize>().unwrap(),
                    second.parse::<usize>().unwrap(),
                )
            })
            .flat_map(|(a, b)| vec![((a, b), Ordering::Less), ((b, a), Ordering::Greater)])
            .collect();
        let updates = updates
            .lines()
            .map(|line| {
                line.split(",")
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();
        (rules, updates)
    }
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let (rules, updates) = Self::parse_input(input);
        updates
            .into_iter()
            // Get the sorted updates
            .filter(|update| {
                update.is_sorted_by(|a, b| !matches!(rules.get(&(*a, *b)), Some(Ordering::Greater)))
            })
            .map(|update| update[update.len() / 2])
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (rules, mut updates) = Self::parse_input(input);
        updates
            .iter_mut()
            // Get the unsorted updates
            .filter(|update| {
                !update
                    .is_sorted_by(|a, b| !matches!(rules.get(&(*a, *b)), Some(Ordering::Greater)))
            })
            .map(|update| {
                update.sort_by(|&a, &b| *rules.get(&(a, b)).unwrap_or(&Ordering::Equal));
                update[update.len() / 2]
            })
            .sum::<usize>()
            .to_string()
    }
}
