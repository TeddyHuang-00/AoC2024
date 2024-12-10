use std::collections::{HashMap, HashSet};

use ndarray::prelude::*;
use rayon::prelude::*;

use crate::solution::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn to_index(&self) -> [usize; 2] {
        [self.row, self.col]
    }

    /// Yields the coordinates on the path from self to other
    fn path_to<'a>(&self, other: &'a Coord) -> impl Iterator<Item = Coord> + 'a {
        let (dx, dy) = match (
            self.row as isize - other.row as isize,
            self.col as isize - other.col as isize,
        ) {
            (0, dy) if dy > 0 => (0, -1),
            (0, dy) if dy < 0 => (0, 1),
            (dx, 0) if dx > 0 => (-1, 0),
            (dx, 0) if dx < 0 => (1, 0),
            _ => panic!("Invalid path: {:?} -> {:?}", self, other),
        };
        std::iter::successors(Some(*self), move |&coord| {
            Some(Coord {
                row: coord.row.wrapping_add_signed(dx),
                col: coord.col.wrapping_add_signed(dy),
            })
        })
        .take_while(|&coord| coord != *other)
    }
}

#[derive(Debug, Clone)]
struct Board {
    nrows: usize,
    ncols: usize,
    obstacles: Array2<bool>,
    // Pre-calculate the next empty position offset
    next: HashMap<Direction, Array2<isize>>,
}

impl Board {
    fn new(grid: Vec<Vec<bool>>) -> Self {
        let nrows = grid.len();
        let ncols = grid[0].len();
        // Pad the obstacles with false
        let obstacles = Array2::from_shape_fn((nrows + 2, ncols + 2), |(i, j)| {
            if i == 0 || i == nrows + 1 || j == 0 || j == ncols + 1 {
                true
            } else {
                grid[i - 1][j - 1]
            }
        });
        // Pre-calculate the next position offset
        let mut up = Array2::from_elem((nrows + 2, ncols + 2), 0);
        let mut left = Array2::from_elem((nrows + 2, ncols + 2), 0);
        (1..=nrows).for_each(|r| {
            (1..=ncols).for_each(|c| {
                if obstacles[(r, c)] {
                    // Reset the free steps
                    up[(r, c)] = -1;
                    left[(r, c)] = -1;
                } else {
                    // One more free step from next obstacle
                    up[(r, c)] = up[(r - 1, c)] + 1;
                    left[(r, c)] = left[(r, c - 1)] + 1;
                }
            });
        });
        let mut down = Array2::from_elem((nrows + 2, ncols + 2), 0);
        let mut right = Array2::from_elem((nrows + 2, ncols + 2), 0);
        (1..=nrows).rev().for_each(|r| {
            (1..=ncols).rev().for_each(|c| {
                if obstacles[(r, c)] {
                    // Reset the free steps
                    down[(r, c)] = -1;
                    right[(r, c)] = -1;
                } else {
                    // One more free step from next obstacle
                    down[(r, c)] = down[(r + 1, c)] + 1;
                    right[(r, c)] = right[(r, c + 1)] + 1;
                }
            });
        });
        let next = [
            (Direction::Up, up),
            (Direction::Down, down),
            (Direction::Left, left),
            (Direction::Right, right),
        ]
        .into_iter()
        .collect();

        Self {
            nrows,
            ncols,
            obstacles,
            next,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    pos: Coord,
    direction: Direction,
}

impl Guard {
    fn new(row: usize, col: usize) -> Self {
        Self {
            pos: Coord { row, col },
            direction: Direction::Up,
        }
    }

    fn out_of_board(&self, board: &Board) -> bool {
        self.pos.row > board.nrows
            || self.pos.col > board.ncols
            || self.pos.row == 0
            || self.pos.col == 0
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }

    /// Move the guard until the next obstacle
    fn step(&mut self, board: &Board) {
        match self.direction {
            Direction::Up => {
                self.pos.row -= board.next[&self.direction][self.pos.to_index()] as usize;
            }
            Direction::Down => {
                self.pos.row += board.next[&self.direction][self.pos.to_index()] as usize;
            }
            Direction::Left => {
                self.pos.col -= board.next[&self.direction][self.pos.to_index()] as usize;
            }
            Direction::Right => {
                self.pos.col += board.next[&self.direction][self.pos.to_index()] as usize;
            }
        }
    }
}

pub struct Puzzle;

impl Puzzle {
    fn parse_input(input: &str) -> (Guard, Board) {
        let mut start = Guard::new(0, 0);
        let board = Board::new(
            input
                .lines()
                .enumerate()
                .map(|(row, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(col, c)| match c {
                            '.' => false,
                            '#' => true,
                            '^' => {
                                // We use 1-based index to allow latter padding
                                start = Guard::new(row + 1, col + 1);
                                false
                            }
                            _ => panic!("Invalid input: {}", c),
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
        let (mut guard, board) = Self::parse_input(input);
        let mut visited = HashSet::new();
        while !guard.out_of_board(&board) {
            let last = guard.pos;
            guard.step(&board);
            visited.extend(last.path_to(&guard.pos));
            guard.turn_right()
        }
        visited.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (start, board) = Self::parse_input(input);
        let mut guard = start;
        let mut candidates = HashSet::new();
        while !guard.out_of_board(&board) {
            let last = guard.pos;
            guard.step(&board);
            candidates.extend(last.path_to(&guard.pos));
            guard.turn_right()
        }
        // Make sure to remove the starting position
        candidates.remove(&start.pos);
        candidates
            .into_par_iter()
            .filter_map(|coord| {
                let mut guard = start;
                let mut board = board.clone();
                // Set the obstacle
                board.obstacles[coord.to_index()] = true;
                // Partial update the next position offset
                board
                    .next
                    .iter_mut()
                    .for_each(|(_, next)| next[coord.to_index()] = -1);
                // We only need to update a local area until the next obstacle
                (coord.row + 1..=board.nrows)
                    .take_while(|&r| !board.obstacles[(r, coord.col)])
                    .for_each(|r| {
                        // One more free step from next obstacle
                        board
                            .next
                            .entry(Direction::Up)
                            .and_modify(|next| next[(r, coord.col)] = next[(r - 1, coord.col)] + 1);
                    });
                (1..=coord.row - 1)
                    .rev()
                    .take_while(|&r| !board.obstacles[(r, coord.col)])
                    .for_each(|r| {
                        // One more free step from next obstacle
                        board
                            .next
                            .entry(Direction::Down)
                            .and_modify(|next| next[(r, coord.col)] = next[(r + 1, coord.col)] + 1);
                    });
                (coord.col + 1..=board.ncols)
                    .take_while(|&c| !board.obstacles[(coord.row, c)])
                    .for_each(|c| {
                        // One more free step from next obstacle
                        board
                            .next
                            .entry(Direction::Left)
                            .and_modify(|next| next[(coord.row, c)] = next[(coord.row, c - 1)] + 1);
                    });
                (1..=coord.col - 1)
                    .rev()
                    .take_while(|&c| !board.obstacles[(coord.row, c)])
                    .for_each(|c| {
                        // One more free step from next obstacle
                        board
                            .next
                            .entry(Direction::Right)
                            .and_modify(|next| next[(coord.row, c)] = next[(coord.row, c + 1)] + 1);
                    });

                // We only need to record the turnings to detect the loop
                let mut visited: HashSet<Guard> = HashSet::new();
                while !guard.out_of_board(&board) {
                    guard.step(&board);
                    if visited.contains(&guard) {
                        // Loop found
                        return Some(coord);
                    }
                    visited.insert(guard);
                    guard.turn_right()
                }
                // No loop found
                None
            })
            .count()
            .to_string()
    }
}
