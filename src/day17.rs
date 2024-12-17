use crate::solution::Solution;

// This has to be larger as the result can be over u32::MAX
// It is approximately 2^50
type IntSize = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    /// Divide A by 2 to the COMBO operand
    Adv,
    /// Bitwise XOR B with the LITERAL operand
    Bxl,
    /// Write the COMBO operand mod 8 to B
    Bst,
    /// Does nothing if A is 0, otherwise set program counter to the LITERAL operand
    Jnz,
    /// Bitwise XOR between B and C to B, ignoring the operand
    Bxc,
    /// Output the COMBO operand mod 8
    Out,
    /// Divide A by 2 to the COMBO operand and store to B
    Bdv,
    /// Divide A by 2 to the COMBO operand and store to C
    Cdv,
}

impl From<u8> for Instruction {
    fn from(n: u8) -> Self {
        match n {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!(),
        }
    }
}

impl Instruction {
    fn execute(&self, pid: &mut usize, computer: &mut Computer, operand: Operand) -> Option<u8> {
        let mut next_ptr = *pid + 2;
        let mut output = None;
        match self {
            Self::Adv => {
                computer.a >>= operand.as_combo(computer);
            }
            Self::Bxl => {
                computer.b ^= operand.as_literal() as IntSize;
            }
            Self::Bst => {
                computer.b = operand.as_combo(computer) % 8;
            }
            Self::Jnz => {
                if computer.a != 0 {
                    next_ptr = operand.as_literal() as usize;
                }
            }
            Self::Bxc => {
                computer.b ^= computer.c;
            }
            Self::Out => {
                output = Some((operand.as_combo(computer) % 8) as u8);
            }
            Self::Bdv => {
                computer.b = computer.a >> operand.as_combo(computer);
            }
            Self::Cdv => {
                computer.c = computer.a >> operand.as_combo(computer);
            }
        }
        // Increment the program counter
        *pid = next_ptr;
        output
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operand {
    /// COMBO: 0
    Op0,
    /// COMBO: 1
    Op1,
    /// COMBO: 2
    Op2,
    /// COMBO: 3
    Op3,
    /// COMBO: A
    Op4,
    /// COMBO: B
    Op5,
    /// COMBO: C
    Op6,
    /// COMBO: invalid
    Op7,
}

impl From<u8> for Operand {
    fn from(n: u8) -> Self {
        match n {
            0 => Self::Op0,
            1 => Self::Op1,
            2 => Self::Op2,
            3 => Self::Op3,
            4 => Self::Op4,
            5 => Self::Op5,
            6 => Self::Op6,
            7 => Self::Op7,
            _ => unreachable!(),
        }
    }
}

impl Operand {
    fn as_literal(&self) -> u8 {
        *self as u8
    }

    fn as_combo(&self, computer: &Computer) -> IntSize {
        match self {
            Self::Op0 | Self::Op1 | Self::Op2 | Self::Op3 => *self as IntSize,
            Self::Op4 => computer.a,
            Self::Op5 => computer.b,
            Self::Op6 => computer.c,
            // Op7 is not valid as a combo operand
            Self::Op7 => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Computer {
    /// Register a
    a: IntSize,
    /// Register b
    b: IntSize,
    /// Register c
    c: IntSize,
}

pub struct Puzzle;

impl Puzzle {
    fn parse_input(input: &str) -> (Computer, Vec<u8>) {
        let get_number = |s: &str| s.split_once(": ").unwrap().1.parse().unwrap();
        let (registers, program) = input.split_once("\n\n").unwrap();
        let [a, b, c] = match registers
            .lines()
            .map(get_number)
            .collect::<Vec<_>>()
            .as_slice()
        {
            &[a, b, c] => [a, b, c],
            _ => unreachable!(),
        };
        let program = program
            .split_once(": ")
            .unwrap()
            .1
            .split(',')
            .filter_map(|c| c.parse::<u8>().ok())
            .collect();
        (Computer { a, b, c }, program)
    }
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let (mut computer, program) = Self::parse_input(input);
        let mut pid = 0;
        let mut output = Vec::new();
        while pid < program.len() {
            let (ins, opr) = (
                Instruction::from(program[pid]),
                Operand::from(program[pid + 1]),
            );
            if let Some(out) = ins.execute(&mut pid, &mut computer, opr) {
                output.push(out);
            }
        }
        output
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn part2(&self, input: &str) -> String {
        let (computer, program) = Self::parse_input(input);
        // The program only exists if the value of a is 0
        // At each cycle of the program, B and C gets overwritten by either literal or the value of A
        // So we only need to determine the value of A that makes the program produce the expected output
        // And by looking at the program, the value of A is always divided by 2^3, so we can just test the missing 3 bits for each cycle
        // This can be done in reverse order to reduce the search space
        let a_values = program.iter().rev().fold(vec![0], |a_values, &target| {
            a_values
                .into_iter()
                .flat_map(|a| {
                    ((a * 8)..(a * 8 + 8)).filter(|&a| {
                        let mut computer = computer;
                        computer.a = a;
                        let mut pid = 0;
                        while pid < program.len() {
                            let (ins, opr) = (
                                Instruction::from(program[pid]),
                                Operand::from(program[pid + 1]),
                            );
                            if let Some(out) = ins.execute(&mut pid, &mut computer, opr) {
                                return out == target;
                            }
                        }
                        false
                    })
                })
                .collect()
        });
        // As we expanded the a values in order and such order is kept, the a_values is always sorted
        a_values.first().unwrap().to_string()
    }
}
