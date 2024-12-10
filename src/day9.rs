use std::{cmp::Reverse, collections::BinaryHeap, iter};

use crate::solution::Solution;

type IndexLengthPairs = Vec<(usize, usize)>;

pub struct Puzzle;

impl Puzzle {
    fn parse_input(input: &str) -> (IndexLengthPairs, IndexLengthPairs) {
        let input = input
            .chars()
            .filter(|&c| c.is_ascii_digit())
            .map(|c| c as usize - '0' as usize)
            .collect::<Vec<_>>();
        let index = iter::once(0)
            .chain(input.iter().scan(0, |acc, &x| {
                *acc += x;
                Some(*acc)
            }))
            .collect::<Vec<_>>();
        let (files, frees): (Vec<_>, Vec<_>) = index
            .into_iter()
            .zip(input)
            .enumerate()
            .partition(|(i, _)| i % 2 == 0);
        let files = files.into_iter().map(|(_, c)| c).collect();
        let frees = frees.into_iter().map(|(_, c)| c).collect();
        (files, frees)
    }
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let (blocks, frees) = Self::parse_input(input);
        // We don't need the index for part 1
        let mut blocks = blocks.into_iter().map(|(_, x)| x).collect::<Vec<_>>();
        let mut frees = frees.into_iter().map(|(_, x)| x).collect::<Vec<_>>();
        let mut checksum = 0;
        // The position of the next block to write
        let mut pos = 0;
        // The index of the next file block to write
        let mut forward_id = 0;
        // The index of the next file block to move
        let mut backward_id = blocks.len() - 1;
        // Move all possible file blocks to free space
        while forward_id <= backward_id {
            match (blocks[forward_id], frees[forward_id]) {
                // Write file blocks first
                (x, _) if x > 0 => {
                    let next = pos + blocks[forward_id];
                    checksum += (pos + next - 1) * blocks[forward_id] / 2 * forward_id;
                    // All file blocks are written
                    blocks[forward_id] = 0;
                    // Move to the end of the file blocks written
                    pos = next;
                }
                // Move the file blocks at the end to current free space
                (0, x) if x > 0 => {
                    // Find the minimum writable blocks
                    let min_writable = frees[forward_id].min(blocks[backward_id]);
                    let next = pos + min_writable;
                    checksum += (pos + next - 1) * min_writable / 2 * backward_id;
                    // Update the file blocks
                    blocks[backward_id] -= min_writable;
                    frees[forward_id] -= min_writable;
                    if blocks[backward_id] == 0 {
                        // All file blocks are moved, still not enough free space
                        backward_id -= 1;
                    }
                    // Move to the end of the file blocks written
                    pos = next;
                }
                // Free space full, move to the next file block
                (0, 0) => {
                    forward_id += 1;
                }
                _ => {
                    unreachable!();
                }
            }
        }
        checksum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (mut blocks, frees) = Self::parse_input(input);
        let mut frees = frees
            .into_iter()
            .fold(vec![BinaryHeap::new(); 10], |mut acc, (i, x)| {
                if x > 0 {
                    acc[x].push(Reverse(i));
                }
                acc
            });
        let mut checksum = 0;
        blocks
            .iter_mut()
            .enumerate()
            .rev()
            .for_each(|(id, (b_idx, b_len))| {
                if let Some((mut f_len, _)) = frees
                    .iter()
                    .enumerate()
                    // Skip the free space that is too small
                    .skip(*b_len)
                    // Skip the length set that has no free space
                    .filter(|(_, ids)| !ids.is_empty() && ids.peek().unwrap().0 < *b_idx)
                    .min_by(|(_, a), (_, b)| a.peek().unwrap().0.cmp(&b.peek().unwrap().0))
                {
                    let f_idx = frees[f_len].pop().unwrap().0;
                    // Move file block to current free space
                    *b_idx = f_idx;
                    // Shrink the free space
                    f_len -= *b_len;
                    if f_len > 0 {
                        frees[f_len].push(Reverse(f_idx + *b_len));
                    }
                }
                checksum += (*b_idx + (*b_idx + *b_len - 1)) * *b_len / 2 * id;
            });
        checksum.to_string()
    }
}
