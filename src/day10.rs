use std::{
    array,
    collections::{HashMap, HashSet},
};

use crate::solution::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn neighbors(&self) -> impl Iterator<Item = Self> + '_ {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(|(dx, dy)| Coord {
                x: self.x.wrapping_add_signed(dx),
                y: self.y.wrapping_add_signed(dy),
            })
    }
}

pub struct Puzzle;

impl Puzzle {
    fn parse_input(input: &str) -> [HashSet<Coord>; 10] {
        input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(x, c)| c.to_digit(10).map(|d| (d as usize, Coord { x, y })))
            })
            .fold(array::from_fn(|_| HashSet::new()), |mut acc, (d, coord)| {
                acc[d].insert(coord);
                acc
            })
    }
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        Self::parse_input(input)
            .into_iter()
            // From the highest digit to the lowest
            .rev()
            // Records the distinct viable trail ends at each position
            .fold(
                None,
                |acc: Option<HashMap<Coord, HashSet<Coord>>>, position| {
                    if let Some(map) = acc {
                        Some(
                            // Add viable positions to the neighbors
                            map.into_iter()
                                .fold(HashMap::new(), |mut acc, (coord, pos)| {
                                    coord.neighbors().for_each(|neighbor| {
                                        position
                                            .contains(&neighbor)
                                            .then(|| acc.entry(neighbor).or_default().extend(&pos));
                                    });
                                    acc
                                }),
                        )
                    } else {
                        // Initialize the positions for the highest digit
                        Some(
                            position
                                .into_iter()
                                .map(|coord| (coord, HashSet::from([coord])))
                                .collect::<HashMap<_, _>>(),
                        )
                    }
                },
            )
            .map_or(0, |maps| {
                maps.into_values().map(|pos| pos.len()).sum::<usize>()
            })
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        Self::parse_input(input)
            .into_iter()
            // From the highest digit to the lowest
            .rev()
            // Records the number of trail ends at each position
            .fold(None, |acc: Option<HashMap<Coord, usize>>, position| {
                if let Some(map) = acc {
                    // Add current count to the neighbors
                    Some(
                        map.into_iter()
                            .fold(HashMap::new(), |mut acc, (coord, pos)| {
                                coord.neighbors().for_each(|neighbor| {
                                    position
                                        .contains(&neighbor)
                                        .then(|| *acc.entry(neighbor).or_default() += pos);
                                });
                                acc
                            }),
                    )
                } else {
                    // Initialize the counts to 1 for the highest digit
                    Some(
                        position
                            .into_iter()
                            .map(|coord| (coord, 1))
                            .collect::<HashMap<_, _>>(),
                    )
                }
            })
            .map_or(0, |maps| maps.into_values().sum::<usize>())
            .to_string()
    }
}
