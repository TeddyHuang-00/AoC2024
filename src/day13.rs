use nalgebra::{Matrix2, Vector2};
use regex::Regex;

use crate::solution::Solution;

pub struct Puzzle;

impl Puzzle {
    fn parse_input(input: &str) -> Vec<(Matrix2<f64>, Vector2<f64>)> {
        let x_pattern = Regex::new(r"X(?:\+|=)(\d+)").unwrap();
        let y_pattern = Regex::new(r"Y(?:\+|=)(\d+)").unwrap();
        let get = |s: &str, pat: &Regex| {
            pat.captures(s)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<f64>()
                .unwrap()
        };
        input
            .split("\n\n")
            .map(|c| {
                let mut c = c.lines();
                let (a, b, g) = (c.next().unwrap(), c.next().unwrap(), c.next().unwrap());
                let basis = Matrix2::new(
                    get(a, &x_pattern),
                    get(b, &x_pattern),
                    get(a, &y_pattern),
                    get(b, &y_pattern),
                );
                let goal = Vector2::new(get(g, &x_pattern), get(g, &y_pattern));
                (basis, goal)
            })
            .collect()
    }
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        Self::parse_input(input)
            .into_iter()
            .filter_map(|(basis, goal)| {
                let cnt = basis.try_inverse().unwrap() * goal;
                let x = cnt.x.round();
                let y = cnt.y.round();
                // Determine if the solution is an integer, be caution with floating point errors
                if (cnt.x - x).abs().fract() < 1e-3 && (cnt.y - y).abs().fract() < 1e-3 {
                    Some(x as u64 * 3 + y as u64)
                } else {
                    None
                }
            })
            .sum::<u64>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        Self::parse_input(input)
            .into_iter()
            .filter_map(|(basis, goal)| {
                let goal = goal.add_scalar(1e13);
                let cnt = basis.try_inverse().unwrap() * goal;
                let x = cnt.x.round();
                let y = cnt.y.round();
                // Determine if the solution is an integer, be caution with floating point errors
                if (cnt.x - x).abs().fract() < 1e-3 && (cnt.y - y).abs().fract() < 1e-3 {
                    Some(x as u64 * 3 + y as u64)
                } else {
                    None
                }
            })
            .sum::<u64>()
            .to_string()
    }
}
