use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt::{Debug, Formatter, Result, Write},
};

use cached::proc_macro::cached;

use crate::solution::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Coord { x, y }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum DirectionPad {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

impl From<DirectionPad> for Coord {
    fn from(val: DirectionPad) -> Self {
        match val {
            DirectionPad::Up => Coord::new(1, 0),
            DirectionPad::Activate => Coord::new(2, 0),
            DirectionPad::Left => Coord::new(0, 1),
            DirectionPad::Down => Coord::new(1, 1),
            DirectionPad::Right => Coord::new(2, 1),
        }
    }
}

impl Default for DirectionPad {
    fn default() -> Self {
        Self::Activate
    }
}

impl Debug for DirectionPad {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_char(match self {
            Self::Activate => 'A',
            Self::Right => '>',
            Self::Left => '<',
            Self::Up => '^',
            Self::Down => 'v',
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NumberPad {
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Activate,
}

impl From<NumberPad> for Coord {
    fn from(val: NumberPad) -> Self {
        match val {
            NumberPad::Num7 => Coord::new(0, 0),
            NumberPad::Num8 => Coord::new(1, 0),
            NumberPad::Num9 => Coord::new(2, 0),
            NumberPad::Num4 => Coord::new(0, 1),
            NumberPad::Num5 => Coord::new(1, 1),
            NumberPad::Num6 => Coord::new(2, 1),
            NumberPad::Num1 => Coord::new(0, 2),
            NumberPad::Num2 => Coord::new(1, 2),
            NumberPad::Num3 => Coord::new(2, 2),
            NumberPad::Num0 => Coord::new(1, 3),
            NumberPad::Activate => Coord::new(2, 3),
        }
    }
}

impl Default for NumberPad {
    fn default() -> Self {
        Self::Activate
    }
}

trait Pad {
    fn disabled_coord() -> Coord;
}

impl Pad for NumberPad {
    fn disabled_coord() -> Coord {
        Coord::new(0, 3)
    }
}

impl Pad for DirectionPad {
    fn disabled_coord() -> Coord {
        Coord::new(0, 0)
    }
}

/// Get possible control sequence from `from` to `to` with `disable` as a disabled coordinate which should not be passed through
#[cached]
fn to_control(from: Coord, to: Coord, disable: Coord) -> Vec<Vec<DirectionPad>> {
    let x = match from.x.cmp(&to.x) {
        Ordering::Greater => vec![DirectionPad::Left; from.x.abs_diff(to.x)],
        Ordering::Less => vec![DirectionPad::Right; from.x.abs_diff(to.x)],
        Ordering::Equal => vec![],
    };
    let y = match from.y.cmp(&to.y) {
        Ordering::Greater => vec![DirectionPad::Up; from.y.abs_diff(to.y)],
        Ordering::Less => vec![DirectionPad::Down; from.y.abs_diff(to.y)],
        Ordering::Equal => vec![],
    };
    let a = vec![DirectionPad::Activate];
    if Coord::new(from.x, to.y) == disable {
        // We cannot go vertical first, so only horizontal => vertical
        vec![[x, y, a].concat()]
    } else if Coord::new(to.x, from.y) == disable {
        // We cannot go horizontal first, so only vertical => horizontal
        vec![[y, x, a].concat()]
    } else {
        // We can go either way, but duplicate may occur
        HashSet::from([
            [x.clone(), y.clone(), a.clone()].concat(),
            [y, x, a].concat(),
        ])
        .into_iter()
        .collect()
    }
}

/// Get the minimum length of control sequence recursively.
/// The only difference from the uncached version is the sequence type is `DirectionPad` instead of `NumberPad`.
/// We could have used a generic type, but cached macro doesn't seem to work with generic types.
#[cached]
fn get_control_length(sequence: Vec<DirectionPad>, layer: usize) -> usize {
    sequence
        .into_iter()
        .fold((0, DirectionPad::default()), |(acc, last), x| {
            let len = to_control(last.into(), x.into(), DirectionPad::disabled_coord())
                .into_iter()
                .map(|s| {
                    if layer > 0 {
                        get_control_length(s, layer - 1)
                    } else {
                        s.len()
                    }
                })
                .min()
                .unwrap();
            (acc + len, x)
        })
        .0
}

pub struct Puzzle;

impl Puzzle {
    fn parse_input(input: &str) -> Vec<(Vec<NumberPad>, usize)> {
        input
            .lines()
            .map(|line| {
                (
                    line.chars()
                        .map(|ch| match ch {
                            '0' => NumberPad::Num0,
                            '1' => NumberPad::Num1,
                            '2' => NumberPad::Num2,
                            '3' => NumberPad::Num3,
                            '4' => NumberPad::Num4,
                            '5' => NumberPad::Num5,
                            '6' => NumberPad::Num6,
                            '7' => NumberPad::Num7,
                            '8' => NumberPad::Num8,
                            '9' => NumberPad::Num9,
                            'A' => NumberPad::Activate,
                            _ => unreachable!(),
                        })
                        .collect(),
                    line.strip_suffix('A').unwrap().parse().unwrap(),
                )
            })
            .collect()
    }

    fn get_control_length(sequence: Vec<NumberPad>, layer: usize) -> usize {
        sequence
            .into_iter()
            .fold((0, NumberPad::default()), |(acc, last), x| {
                let len = to_control(last.into(), x.into(), NumberPad::disabled_coord())
                    .into_iter()
                    .map(|s| {
                        if layer > 0 {
                            get_control_length(s, layer - 1)
                        } else {
                            s.len()
                        }
                    })
                    .min()
                    .unwrap();
                (acc + len, x)
            })
            .0
    }
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        Self::parse_input(input)
            .into_iter()
            .map(|(s, n)| Self::get_control_length(s, 2) * n)
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        Self::parse_input(input)
            .into_iter()
            .map(|(s, n)| Self::get_control_length(s, 25) * n)
            .sum::<usize>()
            .to_string()
    }
}
