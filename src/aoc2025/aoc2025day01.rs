//! [aoc](https://adventofcode.com/2025/day/1)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2025,
        day: 1,
        title: "Secret Entrance",
        solution: ("1092", "6616"),
        example_solutions: vec![("3", "6")],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut rotations = Vec::new();
    for &row in input.iter() {
        let mut chars = row.chars();
        let sign = match chars.next().ok_or("input lines must not be empty")? {
            'L' => -1,
            'R' => 1,
            _ => Err("input lines must start with L or R")?,
        };
        let value = chars
            .as_str()
            .parse::<ItemType>()
            .map_err(|_| "input lines must be integers after first char")?;
        rotations.push(sign * value);
    }
    // ---------- Part 1 + 2
    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut dial = 50;
    for &delta in &rotations {
        ans2 += (dial + delta).abs() / 100;
        if dial != 0 && dial + delta <= 0 {
            ans2 += 1;
        }
        dial = (dial + delta).rem_euclid(100);
        if dial == 0 {
            ans1 += 1;
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
    fn invalid_must_not_be_empty() {
        test_invalid_msg(&[&"L1", &"", &"R1"], solve, "input lines must not be empty");
    }

    #[test]
    fn invalid_must_start_with_l_or_r() {
        test_invalid_msg(&[&"L1", &"A2"], solve, "input lines must start with L or R");
    }

    #[test]
    fn invalid_must_continue_with_integer() {
        test_invalid_msg(
            &[&"L1", &"RA"],
            solve,
            "input lines must be integers after first char",
        );
    }
}
