//! [aoc](https://adventofcode.com/2016/day/6)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2016,
        day: 6,
        title: "Signals and Noise",
        solution: ("cyxeoccr", "batwpask"),
        example_solutions: vec![("easter", "advent")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Part 1 + 2
    let mut ans1 = String::new();
    let mut ans2 = String::new();
    let mut freqs = vec![HashMap::<char, usize>::new(); input[0].len()];
    for &row in input.iter() {
        if row.len() != input[0].len() {
            Err("each row must have same length")?;
        }
        for (i, c) in row.chars().enumerate() {
            freqs[i]
                .entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }
    for freq in freqs.iter() {
        ans1.push(*freq.iter().max_by_key(|(_, &v)| v).unwrap().0);
        ans2.push(*freq.iter().min_by_key(|(_, &v)| v).unwrap().0);
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
    fn invalid_must_have_multiple_of_3_rows() {
        test_invalid_msg(&[&"a", &"ab"], solve, "each row must have same length");
    }
}
