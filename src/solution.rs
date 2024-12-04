/// Trait for a solution to an Advent of Code problem
pub trait Solution {
    /// Solve part 1 of the problem using the provided input
    fn part1(&self, input: &str) -> String;
    /// Solve part 2 of the problem using the provided input
    fn part2(&self, input: &str) -> String;
}
