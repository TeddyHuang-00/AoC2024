use std::{
    collections::{HashMap, HashSet},
    iter,
};

use crate::solution::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn in_bounds(&self, nrows: usize, ncols: usize) -> bool {
        self.row < nrows && self.col < ncols
    }
}

#[derive(Debug, Clone)]
struct Board {
    antenna: HashMap<char, HashSet<Coord>>,
    nrows: usize,
    ncols: usize,
}

impl Board {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let nrows = grid.len();
        let ncols = grid[0].len();
        let antenna = grid
            .iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(move |(col, &cell)| match cell {
                        '.' => None,
                        _ => Some((cell, Coord { row, col })),
                    })
            })
            .fold(
                HashMap::<char, HashSet<Coord>>::new(),
                |mut acc, (cell, coord)| {
                    acc.entry(cell).or_default().insert(coord);
                    acc
                },
            );
        Self {
            antenna,
            nrows,
            ncols,
        }
    }
}

pub struct Puzzle;

impl Puzzle {
    fn parse_input(input: &str) -> Vec<Vec<char>> {
        input.lines().map(|line| line.chars().collect()).collect()
    }
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let board = Board::new(Self::parse_input(input));
        board
            .antenna
            .iter()
            .flat_map(|(_, coords)| {
                coords.iter().flat_map(|coord| {
                    coords.iter().filter_map(move |other| {
                        if coord != other {
                            // Pair two antennas to get possible antinode
                            let antinode = Coord {
                                row: (coord.row * 2).wrapping_sub(other.row),
                                col: (coord.col * 2).wrapping_sub(other.col),
                            };
                            antinode
                                .in_bounds(board.nrows, board.ncols)
                                .then_some(antinode)
                        } else {
                            None
                        }
                    })
                })
            })
            .collect::<HashSet<_>>()
            .len()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let board = Board::new(Self::parse_input(input));
        let max_steps = board.ncols.max(board.nrows) as isize;
        board
            .antenna
            .iter()
            .flat_map(|(_, coords)| {
                coords.iter().flat_map(|&coord| {
                    coords
                        .iter()
                        .filter(move |&&other| coord != other)
                        .flat_map(move |&other| {
                            // Get the slope between two antennas
                            let dc = coord.col as isize - other.col as isize;
                            let dr = coord.row as isize - other.row as isize;
                            let gcd = gcd(dc.abs().max(dr.abs()), dc.abs().min(dr.abs()));
                            let (dc, dr) = (dc / gcd, dr / gcd);
                            // Iterate over the line formed by the two antennas
                            iter::once(coord)
                                .chain((1..max_steps).map_while(move |i| {
                                    let antinode = Coord {
                                        row: coord.row.wrapping_add_signed(dr * i),
                                        col: coord.col.wrapping_add_signed(dc * i),
                                    };
                                    antinode
                                        .in_bounds(board.nrows, board.ncols)
                                        .then_some(antinode)
                                }))
                                .chain((1..max_steps).map_while(move |i| {
                                    let antinode = Coord {
                                        row: coord.row.wrapping_add_signed(dr * -i),
                                        col: coord.col.wrapping_add_signed(dc * -i),
                                    };
                                    antinode
                                        .in_bounds(board.nrows, board.ncols)
                                        .then_some(antinode)
                                }))
                        })
                })
            })
            .collect::<HashSet<_>>()
            .len()
            .to_string()
    }
}
