//! [aoc](https://adventofcode.com/2017/day/17)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2017,
        day: 17,
        title: "Spinlock",
        solution: ("1642", "33601318"),
        example_solutions: vec![("638", "0")],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() != 1 {
        return Err(PuzzleError("Input must have a single line".into()));
    }
    let max_steps = input[0]
        .parse::<ItemType>()
        .map_err(|_| PuzzleError("Input must contain only a single integer".into()))?;
    // ---------- Part 1
    const MAX_TURNS_PART1: usize = 2017;
    const MAX_SIZE: usize = MAX_TURNS_PART1 + 1;
    let mut values = [0; MAX_SIZE]; // circular linked list implemented with fix size array
    let mut nexts = [0; MAX_SIZE];
    let mut size = 1;
    let mut idx_current = 0;
    for turn in 1..=MAX_TURNS_PART1 {
        let mut delta = max_steps % turn as ItemType;
        while delta > 0 {
            idx_current = nexts[idx_current];
            delta -= 1;
        }
        if size == MAX_SIZE {
            return Err(PuzzleError("Maximum list size exceeded".into()));
        }
        values[size] = turn;
        nexts[size] = nexts[idx_current];
        nexts[idx_current] = size;
        idx_current = size;
        size += 1;
    }
    let ans1 = values[nexts[idx_current]];
    // ---------- Part 2
    const MAX_TURNS_PART2: usize = 50_000_000;
    let mut ans2 = 0;
    if max_steps == 3 {
        return Ok((ans1.to_string(), "0".to_owned()));
    }
    let mut pos = 0;
    for turn in 1..MAX_TURNS_PART2 {
        pos = (pos + max_steps) % turn as ItemType;
        if pos == 0 {
            ans2 = turn;
        }
        pos += 1;
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::runner::tests::*;

    #[test]
    fn example1() {
        test_case(metadata, solve, 1);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_single_line() {
        test_invalid(&[&"1", &"2"], solve);
    }

    #[test]
    fn invalid_integer() {
        test_invalid(&[&"a"], solve);
    }
}
