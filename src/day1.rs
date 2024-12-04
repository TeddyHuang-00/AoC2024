use crate::solution::Solution;
use std::collections::BTreeMap;

pub struct Day1;

impl Day1 {
    fn parse_input(input: &str) -> (BTreeMap<i32, i32>, BTreeMap<i32, i32>) {
        let input = input.lines();
        let mut left = BTreeMap::new();
        let mut right = BTreeMap::new();
        input.into_iter().for_each(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let lv = parts[0].parse::<i32>().unwrap();
            let rv = parts[1].parse::<i32>().unwrap();
            left.insert(lv, left.get(&lv).unwrap_or(&0) + 1);
            right.insert(rv, right.get(&rv).unwrap_or(&0) + 1);
        });
        (left, right)
    }
}

impl Solution for Day1 {
    fn part1(&self, input: &str) -> String {
        let (mut left, mut right) = Self::parse_input(input);
        let mut sum = 0;
        while let (Some((lk, lv)), Some((rk, rv))) = (left.pop_first(), right.pop_first()) {
            // Add the minimum of the two counts to the sum
            sum += (lk - rk).abs() * lv.min(rv);
            // Update the counts if there are any left
            match lv.cmp(&rv) {
                std::cmp::Ordering::Less => {
                    right.insert(rk, rv - lv);
                }
                std::cmp::Ordering::Greater => {
                    left.insert(lk, lv - rv);
                }
                std::cmp::Ordering::Equal => {}
            }
        }
        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (left, right) = Self::parse_input(input);
        left.into_iter()
            .map(|(k, v)| k * v * right.get(&k).unwrap_or(&0))
            .sum::<i32>()
            .to_string()
    }
}
