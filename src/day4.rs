use ndarray::{prelude::*, stack};

use crate::solution::Solution;

pub struct Puzzle;

enum Character {
    X,
    M,
    A,
    S,
}

impl Puzzle {
    fn parse_input(input: &str) -> Array<isize, Ix2> {
        // Create a 2D array from the input
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();
        let input = input
            .chars()
            .filter_map(|c| match c {
                'X' => Some(Character::X as isize),
                'M' => Some(Character::M as isize),
                'A' => Some(Character::A as isize),
                'S' => Some(Character::S as isize),
                _ => None,
            })
            .collect();
        Array::from_vec(input)
            .to_shape((height, width))
            .unwrap()
            .to_owned()
    }
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let input = Self::parse_input(input);
        let pattern = Array::from_iter(
            vec![Character::X, Character::M, Character::A, Character::S]
                .into_iter()
                .map(|x| x as isize),
        )
        .insert_axis(Axis(0))
        .insert_axis(Axis(0));
        let (r, c) = (input.nrows(), input.ncols());
        let diff = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        diff.into_iter()
            .map(|(dx, dy)| {
                let dx = match dx {
                    0 => vec![0; 4],
                    1 => (0..4).collect(),
                    -1 => (0..4).rev().collect(),
                    _ => unreachable!(),
                };
                let dy = match dy {
                    0 => vec![0; 4],
                    1 => (0..4).collect(),
                    -1 => (0..4).rev().collect(),
                    _ => unreachable!(),
                };
                let stacked = stack![
                    Axis(2),
                    input.slice(s![dx[0]..r - dx[3], dy[0]..c - dy[3]]),
                    input.slice(s![dx[1]..r - dx[2], dy[1]..c - dy[2]]),
                    input.slice(s![dx[2]..r - dx[1], dy[2]..c - dy[1]]),
                    input.slice(s![dx[3]..r - dx[0], dy[3]..c - dy[0]]),
                ];
                let diff = stacked - pattern.clone();
                let diff = diff.mapv(|x| x.abs()).sum_axis(Axis(2));
                diff.iter().filter(|&&x| x == 0).count()
            })
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let input = Self::parse_input(input);
        let pattern = Array::from_iter(
            vec![
                Character::M,
                Character::M,
                Character::A,
                Character::S,
                Character::S,
            ]
            .into_iter()
            .map(|x| x as isize),
        )
        .insert_axis(Axis(0))
        .insert_axis(Axis(0));
        let (r, c) = (input.nrows(), input.ncols());
        let diff = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
        diff.into_iter()
            .map(|(dx, dy)| {
                let dx = match dx {
                    0 => vec![0, 2, 1, 0, 2],
                    1 => vec![0, 0, 1, 2, 2],
                    -1 => vec![2, 2, 1, 0, 0],
                    _ => unreachable!(),
                };
                let dy = match dy {
                    0 => vec![0, 2, 1, 0, 2],
                    1 => vec![0, 0, 1, 2, 2],
                    -1 => vec![2, 2, 1, 0, 0],
                    _ => unreachable!(),
                };
                let stacked = stack![
                    Axis(2),
                    input.slice(s![dx[0]..r - dx[4], dy[0]..c - dy[4]]),
                    input.slice(s![dx[1]..r - dx[3], dy[1]..c - dy[3]]),
                    input.slice(s![dx[2]..r - dx[2], dy[2]..c - dy[2]]),
                    input.slice(s![dx[3]..r - dx[1], dy[3]..c - dy[1]]),
                    input.slice(s![dx[4]..r - dx[0], dy[4]..c - dy[0]]),
                ];
                let diff = stacked - pattern.clone();
                let diff = diff.mapv(|x| x.abs()).sum_axis(Axis(2));
                diff.iter().filter(|&&x| x == 0).count()
            })
            .sum::<usize>()
            .to_string()
    }
}
