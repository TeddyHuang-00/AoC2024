use std::collections::HashSet;

use rayon::prelude::*;

use crate::solution::Solution;

pub struct Puzzle;

impl Puzzle {
    fn parse_input(input: &str) -> (HashSet<&str>, Vec<&str>, (usize, usize)) {
        let (towels, patterns) = input.split_once("\n\n").unwrap();
        let (mut min, mut max) = (usize::MAX, usize::MIN);
        let towels = towels
            .split(", ")
            .scan(0, |_, t| {
                min = min.min(t.len());
                max = max.max(t.len());
                Some(t)
            })
            .collect();
        let patterns = patterns.lines().collect();
        (towels, patterns, (min, max))
    }

    fn build_pattern(
        pattern: &str,
        towels: &HashSet<&str>,
        // Use the length of the available towels to limit the search space
        (min_len, max_len): (usize, usize),
    ) -> Option<usize> {
        // Dynamic programming to test if the pattern can be built from the towels
        // Total time complexity: O(pattern_len * (towel_max_len - towel_min_len))
        let mut dp = vec![0; pattern.len() + 1];
        dp[0] = 1;
        // O(n): Match from 1 to pattern_len to see
        // if at the certain position the pattern can be built
        (1..=pattern.len()).for_each(|i| {
            // O(m): Match from towel_min_len to towel_max_len to see
            // if the sub pattern can be built from the towels
            (min_len..=max_len.min(i)).for_each(|l| {
                // O(1): Check if the subpart of the pattern is in the towels
                // This only add a little overhead to the time complexity for part 1
                // As we could have early returned if consecutive 0s are found
                // so that no further computation is needed
                if towels.contains(&pattern[(i - l)..i]) {
                    dp[i] += dp[i - l];
                }
            });
        });
        match dp[pattern.len()] {
            0 => None,
            x => Some(x),
        }
    }
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let (towels, patterns, range) = Self::parse_input(input);
        patterns
            .into_par_iter()
            .filter_map(|p| Self::build_pattern(p, &towels, range))
            .count()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (towels, patterns, range) = Self::parse_input(input);
        patterns
            .into_par_iter()
            .filter_map(|p| Self::build_pattern(p, &towels, range))
            .sum::<usize>()
            .to_string()
    }
}
