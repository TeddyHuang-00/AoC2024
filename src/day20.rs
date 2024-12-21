use std::collections::{HashMap, HashSet};

use rayon::prelude::*;

use crate::solution::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn offset_by(&self, x: isize, y: isize) -> Self {
        Coord {
            x: self.x.wrapping_add_signed(x),
            y: self.y.wrapping_add_signed(y),
        }
    }

    fn is_valid(&self, dim: (usize, usize)) -> bool {
        self.x < dim.0 && self.y < dim.1
    }
}

pub struct Puzzle;

impl Puzzle {
    fn parse_input(input: &str) -> HashMap<Coord, usize> {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let (path, start, end) = input.lines().enumerate().fold(
            (HashSet::new(), Coord { x: 0, y: 0 }, Coord { x: 0, y: 0 }),
            |(mut paths, mut start, mut end), (y, line)| {
                line.chars().enumerate().for_each(|(x, c)| match c {
                    '.' => {
                        paths.insert(Coord { x, y });
                    }
                    'S' => {
                        start = Coord { x, y };
                        paths.insert(start);
                    }
                    'E' => {
                        end = Coord { x, y };
                        paths.insert(end);
                    }
                    _ => {}
                });
                (paths, start, end)
            },
        );
        Self::calculate_visit_order(&path, start, end, (width, height))
    }

    fn calculate_visit_order(
        path: &HashSet<Coord>,
        start: Coord,
        end: Coord,
        dim: (usize, usize),
    ) -> HashMap<Coord, usize> {
        let mut visit_order = HashMap::from([(start, 0)]);
        let mut curr = start;
        let mut steps = 0;
        while curr != end {
            steps += 1;
            if let Some(c) = [(1, 0), (0, 1), (-1, 0), (0, -1)]
                .into_iter()
                .map(|(dx, dy)| curr.offset_by(dx, dy))
                .find(|coord| {
                    coord.is_valid(dim) && path.contains(coord) && !visit_order.contains_key(coord)
                })
            {
                curr = c;
                visit_order.insert(curr, steps);
            } else {
                unreachable!()
            }
        }
        visit_order
    }

    fn calculate_short_cut(
        num_steps: isize,
        visit_order: &HashMap<Coord, usize>,
    ) -> HashMap<usize, usize> {
        // Generate all possible offsets for checking
        let offsets = (-num_steps..=num_steps)
            .flat_map(|dy| {
                ((dy.abs() - num_steps)..=(num_steps - dy.abs())).map(move |dx| (dx, dy))
            })
            .collect::<Vec<_>>();
        visit_order
            .keys()
            .par_bridge()
            .into_par_iter()
            .map(|&c| {
                offsets
                    .iter()
                    .fold(HashMap::new(), |mut short_cut, &(dx, dy)| {
                        let n: Coord = c.offset_by(dx, dy);
                        // Check if the new coordinate is valid and the difference is large enough
                        let diff = visit_order
                            .get(&n)
                            .unwrap_or(&0)
                            .saturating_sub(visit_order[&c] + (dx.abs() + dy.abs()) as usize);
                        if diff >= 100 {
                            short_cut
                                .entry(diff)
                                .and_modify(|cnt| *cnt += 1)
                                .or_insert(1);
                        }
                        short_cut
                    })
            })
            .reduce(HashMap::new, |mut acc, short_cut| {
                short_cut.into_iter().for_each(|(k, v)| {
                    acc.entry(k).and_modify(|cnt| *cnt += v).or_insert(v);
                });
                acc
            })
    }
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let visit_order = Self::parse_input(input);
        let short_cut = Self::calculate_short_cut(2, &visit_order);
        short_cut.values().sum::<usize>().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let visit_order = Self::parse_input(input);
        let short_cut = Self::calculate_short_cut(20, &visit_order);
        short_cut.values().sum::<usize>().to_string()
    }
}
