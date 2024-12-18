use std::ops::Add;

use regex::Regex;

use crate::solution::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: i64,
    y: i64,
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub struct Puzzle;

impl Puzzle {
    fn parse_input(input: &str) -> Vec<(Coord, Coord, Coord)> {
        let x_pattern = Regex::new(r"X(?:\+|=)(\d+)").unwrap();
        let y_pattern = Regex::new(r"Y(?:\+|=)(\d+)").unwrap();
        let get = |s: &str, pat: &Regex| {
            pat.captures(s)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap()
        };
        input
            .split("\n\n")
            .map(|c| {
                let mut c = c.lines();
                let (a, b, g) = (c.next().unwrap(), c.next().unwrap(), c.next().unwrap());
                let a = Coord {
                    x: get(a, &x_pattern),
                    y: get(a, &y_pattern),
                };
                let b = Coord {
                    x: get(b, &x_pattern),
                    y: get(b, &y_pattern),
                };
                let g = Coord {
                    x: get(g, &x_pattern),
                    y: get(g, &y_pattern),
                };
                (a, b, g)
            })
            .collect()
    }

    fn linear_solution(a: Coord, b: Coord, g: Coord) -> Option<i64> {
        let _ = "
        | a.x b.x | | x |   | g.x |
        | a.y b.y | | y | = | g.y |

        | x |   | a.x b.x |^-1 | g.x |
        | y | = | a.y b.y |    | g.y |

        | x |   | b.y -b.x | | g.x |  /
        | y | = |-a.y  a.x | | g.y | / det
        ";
        let det = a.x * b.y - a.y * b.x;
        if det == 0 {
            return None;
        }
        let s = Coord {
            x: b.y * g.x - b.x * g.y,
            y: a.x * g.y - a.y * g.x,
        };
        // Check if the solution is an integer
        if s.x % det != 0 || s.y % det != 0 {
            return None;
        }
        Some(s.x / det * 3 + s.y / det)
    }
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        Self::parse_input(input)
            .into_iter()
            .filter_map(|(a, b, g)| Self::linear_solution(a, b, g))
            .sum::<i64>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let offset = Coord {
            x: 1e13 as i64,
            y: 1e13 as i64,
        };
        Self::parse_input(input)
            .into_iter()
            .filter_map(|(a, b, g)| Self::linear_solution(a, b, g + offset))
            .sum::<i64>()
            .to_string()
    }
}
