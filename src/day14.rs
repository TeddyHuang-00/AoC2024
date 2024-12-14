use std::collections::HashSet;

use rayon::prelude::*;
use regex::Regex;

use crate::solution::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

pub struct Puzzle;

impl Puzzle {
    fn parse_input(input: &str) -> Vec<(Coord, Coord)> {
        let position_pattern = Regex::new(r"p=(\d+),(\d+)").unwrap();
        let velocity_pattern = Regex::new(r"v=(-?\d+),(-?\d+)").unwrap();
        input
            .lines()
            .map(|line| {
                let position = position_pattern.captures(line).unwrap();
                let velocity = velocity_pattern.captures(line).unwrap();
                (
                    Coord {
                        x: position[1].parse().unwrap(),
                        y: position[2].parse().unwrap(),
                    },
                    Coord {
                        x: velocity[1].parse().unwrap(),
                        y: velocity[2].parse().unwrap(),
                    },
                )
            })
            .collect()
    }
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let robots = Self::parse_input(input);
        // Naive solution for determine the board type is example or actual input
        let (width, height) = if robots.len() < 100 {
            (11, 7)
        } else {
            (101, 103)
        };
        robots
            .into_iter()
            .map(|(pos, vel)| Coord {
                x: (pos.x + vel.x * 100).rem_euclid(width),
                y: (pos.y + vel.y * 100).rem_euclid(height),
            })
            .fold(vec![0; 4], |mut acc, pos| {
                if pos.x == width / 2 || pos.y == height / 2 {
                    // Ignore robots that are on the axis
                    acc
                } else {
                    // Determine which quadrant the robot is in
                    let idx = if pos.x > width / 2 { 1 } else { 0 }
                        + if pos.y > height / 2 { 2 } else { 0 };
                    acc[idx] += 1;
                    acc
                }
            })
            .into_iter()
            .product::<isize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let robots = Self::parse_input(input);
        // Naive solution for determine the board type is example or actual input
        let (width, height) = if robots.len() < 100 {
            // No solution for example input
            return "No solution".to_string();
        } else {
            (101, 103)
        };
        // This is an arbitrary large enough range to find the answer
        (0..1_000_000)
            .into_par_iter()
            .by_exponential_blocks()
            .find_first(|&i| {
                let robots = robots
                    .iter()
                    .map(|&(pos, vel)| Coord {
                        x: (pos.x + vel.x * i).rem_euclid(width),
                        y: (pos.y + vel.y * i).rem_euclid(height),
                    })
                    .collect::<HashSet<Coord>>();
                // It is highly unlikely that 9 robots will be in the same 3x3 area by chance
                // So, we can safely assume that it forms some pattern
                // Since the pattern is large, we can also increase the range to 5x5 or above
                // to reduce the chance of false positive
                // It just so happens that 3x3 is enough to find the answer
                robots.iter().any(|&pos| {
                    (-1..=1).all(|dx| {
                        (-1..=1).all(|dy| {
                            robots.contains(&Coord {
                                x: pos.x + dx,
                                y: pos.y + dy,
                            })
                        })
                    })
                })
            })
            .unwrap()
            .to_string()
    }
}
