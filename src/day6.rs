use std::collections::HashSet;

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
    fn step(&self, direction: Direction) -> Self {
        let (dr, dc) = match direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };
        Self {
            row: self.row.wrapping_add_signed(dr),
            col: self.col.wrapping_add_signed(dc),
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    obstacles: HashSet<Coord>,
    nrows: usize,
    ncols: usize,
}

impl Board {
    fn new(grid: Vec<Vec<bool>>) -> Self {
        let nrows = grid.len();
        let ncols = grid[0].len();
        let obstacles = grid
            .iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(move |(col, &cell)| cell.then(|| Coord { row, col }))
            })
            .collect();
        Self {
            obstacles,
            nrows,
            ncols,
        }
    }

    fn is_obstacle(&self, coord: Coord) -> bool {
        self.obstacles.contains(&coord)
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

    fn step(&self, board: &Board) -> Option<Self> {
        let next = Guard {
            pos: self.pos.step(self.direction),
            direction: self.direction,
        };
        if next.pos.row >= board.nrows || next.pos.col >= board.ncols {
            // Out of board
            return None;
        }
        board
            .is_obstacle(next.pos)
            // If the next cell is an obstacle, turn right
            .then(|| Guard {
                pos: self.pos,
                direction: match self.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                },
            })
            .or(Some(next))
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
                                start = Guard::new(row, col);
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
        let mut visited = HashSet::from([guard.pos]);
        while let Some(next) = guard.step(&board) {
            guard = next;
            visited.insert(guard.pos);
        }
        visited.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (start, board) = Self::parse_input(input);
        let mut guard = start;
        let mut candidates = HashSet::new();
        while let Some(next) = guard.step(&board) {
            guard = next;
            candidates.insert(guard.pos);
        }
        // Make sure to remove the starting position
        candidates.remove(&start.pos);
        candidates
            .into_par_iter()
            .filter_map(|c| {
                let mut guard = start;
                let mut board = board.clone();
                // Set the obstacle
                board.obstacles.insert(c);
                // Record the full information of visited cells including direction
                let mut visited = HashSet::from([guard]);
                while let Some(next) = guard.step(&board) {
                    guard = next;
                    if visited.contains(&guard) {
                        // Forms a loop
                        return Some(c);
                    } else {
                        // otherwise, add to visited
                        visited.insert(guard);
                    }
                }
                // No loop found
                None
            })
            .count()
            .to_string()
    }
}
