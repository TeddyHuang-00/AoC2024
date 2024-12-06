use std::collections::BTreeSet;

use ndarray::prelude::*;
use rayon::prelude::*;

use crate::solution::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Grid {
    Empty,
    Obstacle,
}

#[derive(Debug, Clone)]
struct Board {
    grid: Array2<Grid>,
    nrows: usize,
    ncols: usize,
}

impl Board {
    fn new(grid: Vec<Vec<Grid>>) -> Self {
        let nrows = grid.len();
        let ncols = grid[0].len();
        let grid = Array2::from_shape_fn((nrows, ncols), |(row, col)| grid[row][col]);
        Self { grid, nrows, ncols }
    }

    fn get(&self, coord: Coord) -> Grid {
        self.grid[coord.to_array()]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    row: usize,
    col: usize,
    direction: Direction,
}

impl Coord {
    fn new(row: usize, col: usize) -> Self {
        Self {
            row,
            col,
            direction: Direction::Up,
        }
    }

    fn to_tuple(&self) -> (usize, usize) {
        (self.row, self.col)
    }

    fn to_array(&self) -> [usize; 2] {
        [self.row, self.col]
    }

    fn step(&self, board: &Board) -> Option<Self> {
        let next_coord = match self.direction {
            Direction::Up => Coord {
                row: self.row.wrapping_sub(1),
                col: self.col,
                direction: self.direction,
            },
            Direction::Down => Coord {
                row: self.row + 1,
                col: self.col,
                direction: self.direction,
            },
            Direction::Left => Coord {
                row: self.row,
                col: self.col.wrapping_sub(1),
                direction: self.direction,
            },
            Direction::Right => Coord {
                row: self.row,
                col: self.col + 1,
                direction: self.direction,
            },
        };
        if next_coord.row >= board.nrows || next_coord.col >= board.ncols {
            // Out of bounds
            return None;
        }
        let next_coord = match board.get(next_coord) {
            Grid::Empty => next_coord,
            Grid::Obstacle => match self.direction {
                Direction::Up => Coord {
                    row: self.row,
                    col: self.col,
                    direction: Direction::Right,
                },
                Direction::Down => Coord {
                    row: self.row,
                    col: self.col,
                    direction: Direction::Left,
                },
                Direction::Left => Coord {
                    row: self.row,
                    col: self.col,
                    direction: Direction::Up,
                },
                Direction::Right => Coord {
                    row: self.row,
                    col: self.col,
                    direction: Direction::Down,
                },
            },
        };
        Some(next_coord)
    }
}

pub struct Puzzle;

impl Puzzle {
    fn parse_input(input: &str) -> (Coord, Board) {
        let mut start = Coord::new(0, 0);
        let board = Board::new(
            input
                .lines()
                .enumerate()
                .map(|(row, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(col, c)| match c {
                            '.' => Grid::Empty,
                            '#' => Grid::Obstacle,
                            '^' => {
                                start = Coord::new(row, col);
                                Grid::Empty
                            }
                            _ => panic!("Invalid input"),
                        })
                        .collect()
                })
                .collect(),
        );
        (start, board)
    }
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let (mut coord, board) = Self::parse_input(input);
        let mut visited = BTreeSet::from([coord.to_tuple()]);
        while let Some(next_coord) = coord.step(&board) {
            coord = next_coord;
            visited.insert(coord.to_tuple());
        }
        visited.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (start, board) = Self::parse_input(input);
        let mut coord = start;
        let mut candidates = BTreeSet::new();
        while let Some(next_coord) = coord.step(&board) {
            coord = next_coord;
            candidates.insert(coord);
        }
        // Make sure to remove the starting position
        candidates.remove(&start);
        candidates
            .into_par_iter()
            .filter_map(|c| {
                let mut coord = start;
                let mut board = board.clone();
                // Set the obstacle
                board.grid[c.to_array()] = Grid::Obstacle;
                // Record the full information of visited cells including direction
                let mut visited = BTreeSet::from([coord]);
                while let Some(next_coord) = coord.step(&board) {
                    coord = next_coord;
                    if visited.contains(&coord) {
                        // Forms a loop
                        return Some(c.to_tuple());
                    } else {
                        // otherwise, add to visited
                        visited.insert(coord);
                    }
                }
                // No loop found
                None
            })
            // Remove duplicates
            .collect::<BTreeSet<_>>()
            .len()
            .to_string()
    }
}
