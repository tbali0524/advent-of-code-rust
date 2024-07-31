//! [aoc](https://adventofcode.com/2017/day/15)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2017,
        day: 15,
        title: "Dueling Generators",
        solution: ("650", "336"),
        example_solutions: vec![("588", "309")],
    }
}

type ItemType = u64;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Check input
    if input.len() != 2 {
        return Err(PuzzleError("Input must have contain two lines".into()));
    }
    let mut starts = [0; 2];
    for idx in 0..2 {
        if !input[idx].starts_with("Generator ") {
            return Err(PuzzleError("Invalid input".into()));
        }
        starts[idx] = input[idx][24..]
            .parse::<ItemType>()
            .map_err(|_| PuzzleError("Input must contain only integers".into()))?
    }
    const MULTIPLIERS: [ItemType; 2] = [16807, 48271];
    const MODULUS: ItemType = 2147483647;
    // ---------- Part 1
    const MAX_STEPS_PART1: ItemType = 40_000_000;
    let mut ans1 = 0;
    let mut values = starts;
    for _ in 0..MAX_STEPS_PART1 {
        for idx in 0..2 {
            values[idx] = (values[idx] * MULTIPLIERS[idx]) % MODULUS;
        }
        if values[0] & 0xFFFF == values[1] & 0xFFFF {
            ans1 += 1;
        }
    }
    // ---------- Part 2
    const MAX_STEPS_PART2: ItemType = 5_000_000;
    let mut ans2 = 0;
    let mut values = starts;
    const MASKS: [ItemType; 2] = [0b11, 0b111];
    for _ in 0..MAX_STEPS_PART2 {
        for idx in 0..2 {
            loop {
                values[idx] = (values[idx] * MULTIPLIERS[idx]) % MODULUS;
                if values[idx] & MASKS[idx] == 0 {
                    break;
                }
            }
        }
        if values[0] & 0xFFFF == values[1] & 0xFFFF {
            ans2 += 1;
        }
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
    fn invalid_not_two_lines() {
        test_invalid(&[&"Generator A starts with 1"], solve);
    }

    #[test]
    fn invalid_input_format() {
        test_invalid(&["Generator A starts with 1", &"a"], solve);
    }

    #[test]
    fn invalid_parameter_mas_be_int() {
        test_invalid(
            &[&"Generator B starts with 1", &"Generator A starts with a"],
            solve,
        );
    }
}
