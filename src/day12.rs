use std::{collections::HashMap, iter};

use ndarray::prelude::*;

use crate::solution::Solution;

type Coord = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DisjointSet {
    parent: Array2<Coord>,
}

impl DisjointSet {
    fn new(nrows: usize, ncols: usize) -> Self {
        DisjointSet {
            parent: Array2::from_shape_fn((nrows, ncols), |c| c),
        }
    }

    fn find(&mut self, (r, c): Coord) -> Coord {
        // Find the root
        let mut root = (r, c);
        while self.parent[root] != root {
            root = self.parent[root];
        }
        // Path compression
        let mut current = (r, c);
        while current != root {
            let next = self.parent[current];
            self.parent[current] = root;
            current = next;
        }
        root
    }

    fn union(&mut self, a: Coord, b: Coord) {
        // Union by merging the roots
        let root_a = self.find(a);
        let root_b = self.find(b);
        self.parent[root_a] = root_b;
    }
}

pub struct Puzzle;

impl Puzzle {
    fn parse_input(input: &str) -> ((usize, usize), Array2<char>) {
        let input = input
            .lines()
            .map(|l| l.chars().filter(|c| c.is_alphabetic()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let nrows = input.len();
        let ncols = input[0].len();
        (
            (nrows, ncols),
            // Pad the board to avoid boundary checks
            Array2::from_shape_fn((nrows + 2, ncols + 2), |(r, c)| {
                if r > 0 && r <= nrows && c > 0 && c <= ncols {
                    input[r - 1][c - 1]
                } else {
                    '.'
                }
            }),
        )
    }

    fn coord_to_vertices((r, c): Coord) -> [Coord; 4] {
        [(r, c), (r + 1, c), (r, c + 1), (r + 1, c + 1)]
    }
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let ((nrows, ncols), plots) = Self::parse_input(input);
        // First pass: Build disjoint set to keep track of connected regions
        let mut regions = DisjointSet::new(nrows + 2, ncols + 2);
        (1..=nrows).for_each(|r| {
            (1..=ncols).for_each(|c| {
                if plots[(r - 1, c)] == plots[(r, c)] {
                    regions.union((r, c), (r - 1, c))
                }
                if plots[(r, c - 1)] == plots[(r, c)] {
                    regions.union((r, c), (r, c - 1));
                }
            })
        });
        // Second pass: Calculate area and perimeter for each region
        let mut area = HashMap::new();
        let mut perimeter = HashMap::new();
        (1..=nrows).for_each(|r| {
            (1..=ncols).for_each(|c| {
                area.entry(regions.find((r, c)))
                    .and_modify(|a| *a += 1)
                    .or_insert(1);
                perimeter
                    .entry(regions.find((r, c)))
                    .and_modify(|p| {
                        match (
                            regions.find((r - 1, c)) == regions.find((r, c)),
                            regions.find((r, c - 1)) == regions.find((r, c)),
                        ) {
                            // Isolated region
                            (false, false) => *p += 4,
                            // Connected to one side
                            (true, false) | (false, true) => *p += 2,
                            // Connected to both sides
                            (true, true) => {}
                        }
                    })
                    .or_insert(4);
            })
        });
        area.keys()
            .map(|k| area[k] * perimeter[k])
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let ((nrows, ncols), plots) = Self::parse_input(input);
        // First pass: Build disjoint set to keep track of connected regions
        let mut regions = DisjointSet::new(nrows + 2, ncols + 2);
        (1..=nrows).for_each(|r| {
            (1..=ncols).for_each(|c| {
                if plots[(r - 1, c)] == plots[(r, c)] {
                    regions.union((r, c), (r - 1, c))
                }
                if plots[(r, c - 1)] == plots[(r, c)] {
                    regions.union((r, c), (r, c - 1));
                }
            })
        });
        // Second pass: Calculate area and corners for each region
        let mut area = HashMap::new();
        let mut corner = HashMap::new();
        (1..=nrows).for_each(|r| {
            (1..=ncols).for_each(|c| {
                area.entry(regions.find((r, c)))
                    .and_modify(|a| *a += 1)
                    .or_insert(1);
                corner
                    .entry(regions.find((r, c)))
                    .and_modify(|p: &mut HashMap<Coord, Vec<Coord>>| {
                        Self::coord_to_vertices((r, c)).iter().for_each(|v| {
                            p.entry(*v).or_default().push((r, c));
                        });
                    })
                    .or_insert(HashMap::from_iter(
                        Self::coord_to_vertices((r, c))
                            .into_iter()
                            .zip(iter::repeat(vec![(r, c)])),
                    ));
            })
        });
        // Convert corner to count of sides
        // The number of sides is equal to the number of corners
        let side = corner
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    v.into_values()
                        .filter_map(|c| match c.len() {
                            // Outward / inward corner
                            1 | 3 => Some(1),
                            // Side or two diagonal corners
                            // depending on how the region is connected
                            2 => {
                                let (r1, c1) = c[0];
                                let (r2, c2) = c[1];
                                if r1 != r2 && c1 != c2 {
                                    // Connected to the corner, two distinct corners
                                    Some(2)
                                } else {
                                    // Connected to the side, no corner
                                    None
                                }
                            }
                            // No corner
                            4 => None,
                            _ => unreachable!(),
                        })
                        .sum::<usize>(),
                )
            })
            .collect::<HashMap<_, _>>();
        area.keys()
            .map(|k| area[k] * side[k])
            .sum::<usize>()
            .to_string()
    }
}
