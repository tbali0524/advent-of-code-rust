//! [aoc](https://adventofcode.com/2022/day/1)

use crate::aoc::PuzzleMetaData;
use crate::aoc::PuzzleResult;

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2022,
    day: 1,
    title: "Calorie Counting",
    solution: (72070, 211805),
    example_solutions: [(24000, 45000), (0, 0)],
    string_solution: None,
    example_string_solutions: None,
    example_string_inputs: None,
};

type ItemType = i32;

pub fn solve(input: &[String]) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut calories = Vec::new();
    calories.push(Vec::new());
    for line in input {
        if line.is_empty() {
            calories.push(Vec::new());
            continue;
        }
        calories.last_mut().unwrap().push(
            line.parse::<ItemType>()
                .map_err(|_| "Input must contain only integers")?,
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
// --- boilerplate below ---

pub fn run() -> bool {
    crate::aoc::runner::run_puzzle(&PUZZLE_METADATA, solve)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::runner::tests::*;

    #[test]
    fn example1() {
        test_case(&PUZZLE_METADATA, 1, solve);
    }

    #[test]
    fn puzzle() {
        test_case(&PUZZLE_METADATA, 0, solve);
    }

    #[test]
    fn invalid_only_contains_int() {
        test_invalid(&PUZZLE_METADATA, &[String::from("a")], solve);
    }
}
