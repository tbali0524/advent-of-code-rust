//! [aoc](https://adventofcode.com/2022/day/1)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2022,
        day: 1,
        title: "Calorie Counting",
        solution: ("72070", "211805"),
        example_solutions: vec![("24000", "45000")],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut calories = Vec::new();
    calories.push(Vec::new());
    for &line in input {
        if line.is_empty() {
            calories.push(Vec::new());
            continue;
        }
        calories.last_mut().unwrap().push(
            line.parse::<ItemType>()
                .map_err(|_| format!("input must contain only integers, found `{}`", line))?,
        );
    }
    // ---------- Part 1 + 2
    let mut cals = calories
        .iter()
        .map(|row| row.iter().sum::<ItemType>())
        .collect::<Vec<_>>();
    let ans1 = *cals.iter().max().unwrap();
    cals.sort();
    cals.reverse();
    let ans2 = cals[0] + cals.get(1).unwrap_or(&0) + cals.get(2).unwrap_or(&0);
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
    fn invalid_only_contains_int() {
        test_invalid_msg(&[&"a"], solve, "input must contain only integers");
    }
}
