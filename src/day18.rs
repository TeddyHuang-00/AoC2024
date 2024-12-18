use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fmt,
};

use ndarray::prelude::*;

use crate::solution::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn into_index(self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    coord: Coord,
    path_cost: usize,
    total_cost: usize,
}

impl State {
    fn new(coord: Coord, path_cost: usize, goal: Coord) -> Self {
        Self {
            coord,
            path_cost,
            // Path cost + Manhattan distance heuristic
            total_cost: path_cost + coord.x.abs_diff(goal.x) + coord.y.abs_diff(goal.y),
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_cost.cmp(&other.total_cost).reverse()
    }
}

pub struct Puzzle;

impl Puzzle {
    fn parse_input(input: &str) -> (Vec<Coord>, Array2<usize>, usize, usize) {
        let fallen = input
            .lines()
            .map(|l| {
                let (x, y) = l.split_once(',').unwrap();
                Coord {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                }
            })
            .collect::<Vec<_>>();
        let (num, dim) = if fallen.len() < 1000 {
            (12, 6 + 1)
        } else {
            (1024, 70 + 1)
        };
        // As the input is very dense, we can lose performance if using HashMap
        // so we are actually better just using matrix to record fallen time of every coordinate
        let mut board = Array2::from_shape_simple_fn((dim, dim), || usize::MAX);
        fallen.iter().enumerate().for_each(|(t, c)| {
            board[c.into_index()] = t;
        });
        (fallen, board, dim, num)
    }

    /// A* search algorithm
    ///
    /// We can save some search time when there is a path
    /// and retain same complexity when there isn't
    fn search(
        // Obstacles are recorded with index of fallen
        obstacles: &Array2<usize>,
        // Time is used to compare if any obstacle exists or not at the time
        time: usize,
        dim: usize,
    ) -> Option<usize> {
        let start = Coord { x: 0, y: 0 };
        let goal = Coord {
            x: dim - 1,
            y: dim - 1,
        };
        let mut visited = HashSet::new();
        let mut frontiers = BinaryHeap::from([State::new(start, 0, goal)]);
        // A* search
        while let Some(State {
            coord, path_cost, ..
        }) = frontiers.pop()
        {
            // Goal test on expansion to ensure optimality
            if coord == goal {
                return Some(path_cost);
            }
            // Skip visited nodes here as multiple nodes can co-exists
            // with same coord (due to order of expansion)
            if !visited.insert(coord) {
                continue;
            }
            // Insert valid neighbors to the frontiers
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let new_coord = Coord {
                    x: coord.x.wrapping_add_signed(dx),
                    y: coord.y.wrapping_add_signed(dy),
                };
                // The new coordinate is within board
                if (new_coord.x < dim && new_coord.y < dim)
                    // Not taken by any obstacles, or the obstacle hasn't fallen yet
                    && obstacles[new_coord.into_index()] >= time
                    // And we don't visit it yet
                    && !visited.contains(&new_coord)
                {
                    frontiers.push(State::new(new_coord, path_cost + 1, goal));
                }
            }
        }
        None
    }
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let (_, obstacles, dim, num) = Self::parse_input(input);
        if let Some(time) = Self::search(&obstacles, num, dim) {
            return time.to_string();
        }
        "Solution not found".to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (fallen, obstacles, dim, num) = Self::parse_input(input);
        // We know that a path exists at least for part 1, so we can skip the check
        let (mut lb, mut rb) = (num, obstacles.len());
        // Binary search
        while lb < rb {
            let mid = (lb + rb) / 2;
            if Self::search(&obstacles, mid, dim).is_some() {
                // Goal is still reachable at time mid
                // Update lb to search the right half
                lb = mid + 1;
            } else {
                // Goal is not reachable at time mid
                // Update rb to search the left half
                rb = mid;
            }
        }
        fallen[lb - 1].to_string()
    }
}
