use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    iter::repeat,
};

use crate::solution::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    // We don't need backwards because it is always worse than
    // not taking previous steps
    fn get_turns(&self) -> [Self; 2] {
        match self {
            Self::North => [Self::West, Self::East],
            Self::East => [Self::North, Self::South],
            Self::South => [Self::East, Self::West],
            Self::West => [Self::South, Self::North],
        }
    }

    fn reverse(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    fn iter() -> impl Iterator<Item = Self> {
        [Self::North, Self::East, Self::South, Self::West].into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn next_towards(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => Self::new(self.x, self.y - 1),
            Direction::East => Self::new(self.x + 1, self.y),
            Direction::South => Self::new(self.x, self.y + 1),
            Direction::West => Self::new(self.x - 1, self.y),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    coord: Coord,
    direction: Direction,
    path_cost: usize,
    heuristic: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.path_cost + self.heuristic == other.path_cost + other.heuristic
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.path_cost + self.heuristic)
            .cmp(&(other.path_cost + other.heuristic))
            // Reverse the ordering to get the smallest element
            .reverse()
    }
}

impl State {
    fn new(coord: Coord, direction: Direction, path_cost: usize, goal: Coord) -> Self {
        Self {
            coord,
            direction,
            path_cost,
            heuristic: goal.x.abs_diff(coord.x) + goal.y.abs_diff(coord.y),
        }
    }
}

pub struct Puzzle;

impl Puzzle {
    fn parse_input(input: &str) -> (HashSet<Coord>, Coord, Coord) {
        input.lines().enumerate().fold(
            (HashSet::new(), Coord::new(0, 0), Coord::new(0, 0)),
            |(mut path, mut start, mut goal), (y, line)| {
                line.char_indices().for_each(|(x, c)| match c {
                    '#' => {}
                    '.' => {
                        path.insert(Coord::new(x, y));
                    }
                    'S' => {
                        start = Coord::new(x, y);
                        path.insert(Coord::new(x, y));
                    }
                    'E' => {
                        goal = Coord::new(x, y);
                        path.insert(Coord::new(x, y));
                    }
                    _ => unreachable!(),
                });
                (path, start, goal)
            },
        )
    }
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let (path, start, goal) = Self::parse_input(input);
        // Store the visited nodes by direction and coordinate
        let mut visited: HashMap<Direction, HashSet<Coord>> =
            HashMap::from_iter(Direction::iter().zip(repeat(HashSet::new())));
        let mut frontier = BinaryHeap::from([State::new(start, Direction::East, 0, goal)]);
        // A* search
        while let Some(state) = frontier.pop() {
            let State {
                coord,
                direction,
                path_cost,
                ..
            } = state;
            // Goal test on the current state
            if coord == goal {
                return path_cost.to_string();
            }
            visited.get_mut(&direction).unwrap().insert(coord);
            // Expand the current state
            let [left, right] = direction.get_turns();
            [direction, left, right]
                .into_iter()
                .for_each(|next_direction| {
                    let (next_coord, step_cost) = if next_direction == direction {
                        (coord.next_towards(next_direction), 1)
                    } else {
                        (coord, 1000)
                    };
                    if path.contains(&next_coord) && !visited[&next_direction].contains(&next_coord)
                    {
                        frontier.push(State::new(
                            next_coord,
                            next_direction,
                            path_cost + step_cost,
                            goal,
                        ));
                    }
                });
        }
        unreachable!()
    }

    fn part2(&self, input: &str) -> String {
        let (path, start, goal) = Self::parse_input(input);
        // Store the visited nodes by direction and coordinate
        // Value contains the best score
        let mut visited: HashMap<Direction, HashMap<Coord, usize>> =
            HashMap::from_iter(Direction::iter().zip(repeat(HashMap::new())));
        let mut frontier = BinaryHeap::from([State::new(start, Direction::East, 0, goal)]);
        let mut goal_cost = usize::MAX;
        // A* search
        while let Some(state) = frontier.pop() {
            let State {
                coord,
                direction,
                path_cost,
                ..
            } = state;
            // Goal test on the current state
            if path_cost > goal_cost {
                break;
            } else if coord == goal {
                goal_cost = path_cost;
            }
            visited
                .entry(direction)
                .or_default()
                .entry(coord)
                .and_modify(|cost| *cost = (*cost).min(path_cost))
                .or_insert(path_cost);
            // Expand the current state
            let [left, right] = direction.get_turns();
            [direction, left, right]
                .into_iter()
                .for_each(|next_direction| {
                    let (next_coord, step_cost) = if next_direction == direction {
                        (coord.next_towards(next_direction), 1)
                    } else {
                        (coord, 1000)
                    };
                    if path.contains(&next_coord)
                        && !visited[&next_direction].contains_key(&next_coord)
                    {
                        frontier.push(State::new(
                            next_coord,
                            next_direction,
                            path_cost + step_cost,
                            goal,
                        ));
                    }
                });
        }
        // Reconstruct the path
        let mut frontier = Direction::iter()
            .filter_map(|dir| {
                if visited[&dir].get(&goal).is_some_and(|&c| c == goal_cost) {
                    Some((goal, dir, goal_cost))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let mut seats = HashSet::from([goal]);
        while let Some((coord, direction, cost)) = frontier.pop() {
            if cost == 0 {
                continue;
            }
            // Search for parents whose cost matches
            let [left, right] = direction.get_turns();
            [direction, left, right].into_iter().for_each(|dir| {
                let (last_coord, step_cost) = if dir == direction {
                    // We are moving one step backwards
                    (coord.next_towards(dir.reverse()), 1)
                } else {
                    // Or turning at the same spot
                    (coord, 1000)
                };
                if (!seats.contains(&last_coord) || dir != direction)
                    && visited[&dir]
                        .get(&last_coord)
                        .is_some_and(|&c| c + step_cost == cost)
                {
                    let last_cost = visited[&dir][&last_coord];
                    frontier.push((last_coord, dir, last_cost));
                    seats.insert(last_coord);
                }
            });
        }
        seats.len().to_string()
    }
}
