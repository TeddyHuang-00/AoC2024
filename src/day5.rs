use std::collections::{HashMap, HashSet};

use crate::solution::Solution;

pub struct Puzzle;

impl Puzzle {
    fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
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
            .filter_map(|update| {
                if rules.iter().any(|(first, second)| {
                    // Check if the first and second numbers are in the update
                    update.contains(first)
                        && update.contains(second)
                    // and the first number doesn't come before the second
                        && update.iter().position(|&n| n == *first)
                            > update.iter().position(|&n| n == *second)
                }) {
                    None
                } else {
                    Some(update[update.len() / 2])
                }
            })
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (rules, updates) = Self::parse_input(input);
        updates
            .into_iter()
            .filter_map(|update| {
                if rules.iter().any(|(first, second)| {
                    // Check if the first and second numbers are in the update
                    update.contains(first)
                        && update.contains(second)
                    // and the first number doesn't come before the second
                        && update.iter().position(|&n| n == *first)
                            > update.iter().position(|&n| n == *second)
                }) {
                    let mut edges = HashMap::new();
                    rules.iter().for_each(|(a, b)| {
                        if update.contains(a) && update.contains(b) {
                            edges.entry(*b).or_insert_with(HashSet::new).insert(*a);
                        }
                    });
                    // Topological sort
                    let mut sorted = vec![];
                    while let Some(parent) = update
                        .iter()
                        .filter(|&&node| !sorted.contains(&node))
                        .find(|&&node| edges.entry(node).or_insert_with(HashSet::new).is_empty())
                    {
                        sorted.push(*parent);
                        for (_, children) in edges.iter_mut() {
                            children.remove(parent);
                        }
                    }
                    Some(sorted[sorted.len() / 2])
                } else {
                    None
                }
            })
            .sum::<usize>()
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
