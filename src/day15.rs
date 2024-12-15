use std::collections::HashSet;

use crate::solution::Solution;

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
            Direction::Up => Self::new(self.x, self.y - 1),
            Direction::Down => Self::new(self.x, self.y + 1),
            Direction::Left => Self::new(self.x - 1, self.y),
            Direction::Right => Self::new(self.x + 1, self.y),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Puzzle;

impl Puzzle {
    fn parse_input(input: &str) -> (HashSet<Coord>, HashSet<Coord>, Coord, Vec<Direction>) {
        let (board, moves) = input.split_once("\n\n").unwrap();
        let (obstacles, boxes, start) = board.lines().enumerate().fold(
            (HashSet::new(), HashSet::new(), Coord::new(0, 0)),
            |(mut obstacles, mut boxes, mut start), (y, line)| {
                line.chars().enumerate().for_each(|(x, c)| match c {
                    '#' => {
                        obstacles.insert(Coord::new(x, y));
                    }
                    'O' => {
                        boxes.insert(Coord::new(x, y));
                    }
                    '@' => {
                        start = Coord::new(x, y);
                    }
                    _ => {}
                });
                (obstacles, boxes, start)
            },
        );
        let moves = moves
            .lines()
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    _ => unreachable!(),
                })
            })
            .collect();
        (obstacles, boxes, start, moves)
    }

    fn is_pushable(
        coord: Coord,
        direction: Direction,
        obstacles: &HashSet<Coord>,
        boxes: &HashSet<Coord>,
    ) -> bool {
        // Coordinate of box after pushing
        let next = coord.next_towards(direction);
        // Fails immediately if the next position is an obstacle
        if obstacles.contains(&next) {
            return false;
        }
        // The box is pushable if the next position is empty
        // or if there is an empty space along the way
        !boxes.contains(&next) || Self::is_pushable(next, direction, obstacles, boxes)
    }

    /// Returns `None` if the box is not pushable, otherwise returns the left parts of boxes that are affected by the push
    ///
    /// I find this part to be the most challenging part of the puzzle,
    /// as it is hard to find a good way to optimize the logic
    ///
    /// And the complexity will be O(2^n) unlike the previous part which is O(n),
    /// due to the fact that boxes can be chained
    /// and the number of boxes affected in a single push can cascade exponentially
    fn is_double_wide_pushable(
        coord: Coord,
        direction: Direction,
        obstacles: &HashSet<Coord>,
        left_boxes: &HashSet<Coord>,
    ) -> Option<HashSet<Coord>> {
        // Get counter part
        let (mut left, mut right) = (coord, coord);
        if left_boxes.contains(&coord) {
            right = left.next_towards(Direction::Right);
        } else if left_boxes.contains(&coord.next_towards(Direction::Left)) {
            left = right.next_towards(Direction::Left);
        } else {
            // We should always run this check on a box that always has a counter part
            unreachable!()
        }
        // Coordinate of box after pushing
        let left_next = left.next_towards(direction);
        let right_next = right.next_towards(direction);
        // Fails immediately if the next position is an obstacle
        if obstacles.contains(&left_next) || obstacles.contains(&right_next) {
            return None;
        }
        match direction {
            Direction::Left => {
                // We skip the check for the right box because we are pushing to the left
                if !left_boxes.contains(&left_next.next_towards(Direction::Left)) {
                    Some(HashSet::from([left]))
                } else if let Some(mut bs) =
                    Self::is_double_wide_pushable(left_next, direction, obstacles, left_boxes)
                {
                    bs.insert(left);
                    Some(bs)
                } else {
                    None
                }
            }
            Direction::Right => {
                // We skip the check for the left box because we are pushing to the right
                if !left_boxes.contains(&right_next) {
                    Some(HashSet::from([left]))
                } else if let Some(mut bs) =
                    Self::is_double_wide_pushable(right_next, direction, obstacles, left_boxes)
                {
                    bs.insert(left);
                    Some(bs)
                } else {
                    None
                }
            }
            Direction::Down | Direction::Up => {
                match (
                    !left_boxes.contains(&left_next.next_towards(Direction::Left))
                        && !left_boxes.contains(&left_next),
                    !left_boxes.contains(&right_next),
                ) {
                    // Both sides are empty, the box is pushable
                    (true, true) => Some(HashSet::from([left])),
                    // One of the sides is empty, depending on whether the other side is pushable
                    (true, false) => {
                        if let Some(mut bs) = Self::is_double_wide_pushable(
                            right_next, direction, obstacles, left_boxes,
                        ) {
                            bs.insert(left);
                            Some(bs)
                        } else {
                            None
                        }
                    }
                    (false, true) => {
                        if let Some(mut bs) = Self::is_double_wide_pushable(
                            left_next, direction, obstacles, left_boxes,
                        ) {
                            bs.insert(left);
                            Some(bs)
                        } else {
                            None
                        }
                    }
                    // Both sides are not empty, check if both sides are pushable
                    (false, false) => {
                        if let (Some(mut left_bs), Some(mut right_bs)) = (
                            Self::is_double_wide_pushable(
                                left_next, direction, obstacles, left_boxes,
                            ),
                            Self::is_double_wide_pushable(
                                right_next, direction, obstacles, left_boxes,
                            ),
                        ) {
                            left_bs.insert(left);
                            left_bs.extend(right_bs.drain());
                            Some(left_bs)
                        } else {
                            None
                        }
                    }
                }
            }
        }
    }
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let (obstacles, mut boxes, start, moves) = Self::parse_input(input);
        moves.into_iter().fold(start, |mut pos, direction| {
            let next = pos.next_towards(direction);
            // No further action if the next position is an obstacle
            if !obstacles.contains(&next) {
                if !boxes.contains(&next) {
                    // Move to the next position if it is totally empty
                    pos = next;
                } else if Self::is_pushable(next, direction, &obstacles, &boxes) {
                    // Move to the next position if there is a box that can be pushed
                    let mut new_box = next;
                    // Locate the position of empty space for pushing
                    while boxes.contains(&new_box) {
                        new_box = new_box.next_towards(direction);
                    }
                    // One-step push
                    boxes.remove(&next);
                    boxes.insert(new_box);
                    pos = next;
                }
            }
            pos
        });
        boxes
            .into_iter()
            .map(|Coord { x, y }| x + 100 * y)
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (mut obstacles, boxes, mut start, moves) = Self::parse_input(input);
        // Doubling width of everything
        obstacles = obstacles
            .into_iter()
            .flat_map(|Coord { x, y }| [Coord::new(2 * x, y), Coord::new(2 * x + 1, y)])
            .collect();
        let mut left_boxes = boxes
            .into_iter()
            .map(|Coord { x, y }| Coord::new(2 * x, y))
            .collect::<HashSet<Coord>>();
        start = Coord::new(2 * start.x, start.y);
        // Freeze the obstacles and start position
        let start = start;
        let obstacles = obstacles;
        moves.into_iter().fold(start, |mut pos, direction| {
            let next = pos.next_towards(direction);
            // No further action if the next position is an obstacle
            if !obstacles.contains(&next) {
                if !left_boxes.contains(&next)
                    && !left_boxes.contains(&next.next_towards(Direction::Left))
                {
                    // Move to the next position if it is totally empty
                    pos = next;
                } else if let Some(lefts) =
                    Self::is_double_wide_pushable(next, direction, &obstacles, &left_boxes)
                {
                    // Move to the next position if there is a box that can be pushed
                    // Then remove the left and right boxes that are pushed
                    left_boxes.retain(|coord| !lefts.contains(coord));
                    // Then insert the left and right boxes that are pushed with the new positions
                    left_boxes.extend(lefts.into_iter().map(|coord| coord.next_towards(direction)));
                    pos = next;
                }
            }
            pos
        });
        left_boxes
            .into_iter()
            .map(|Coord { x, y }| x + 100 * y)
            .sum::<usize>()
            .to_string()
    }
}
