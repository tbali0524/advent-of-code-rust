// https://adventofcode.com/2015/day/1

use crate::aoc::PuzzleMetaData;
use crate::aoc::PuzzleResult;

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2015,
    day: 25,
    title: "Let It Snow",
    solutions: (19980801, 0),
    example_solutions: [(31916031, 0), (27995004, 0)],
    example_string_inputs: ["", ""],
};

type ItemType = u64;

pub fn solve(input: &[String]) -> PuzzleResult {
    // ---------- Check input
    if input.len() != 1 {
        return Err("Input must have a single line");
    }
    let a = input[0].split_whitespace().collect::<Vec<_>>();
    if a.len() != 18 {
        return Err("Invalid input sentence");
    }
    let row = a[15][0..a[15].len() - 1]
        .parse::<ItemType>()
        .map_err(|_| "Invalid input")?;
    let column = a[17][0..a[17].len() - 1]
        .parse::<ItemType>()
        .map_err(|_| "Invalid input")?;
    let n = row + column - 2;
    let steps = (n * (n + 1)) / 2 + column - 1;
    // ---------- Part 1 + 2
    let mut ans1: ItemType = 20151125;
    for _ in 0..steps {
        ans1 = (ans1 * 252533) % 33554393;
    }
    let ans2 = 0;
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
    fn example2() {
        test_case(&PUZZLE_METADATA, 2, solve);
    }

    #[test]
    fn puzzle() {
        test_case(&PUZZLE_METADATA, 0, solve);
    }

    #[test]
    fn invalid_single_line() {
        test_invalid(
            &PUZZLE_METADATA,
            &[String::from("a"), String::from("b")],
            solve,
        );
    }

    #[test]
    fn invalid_input_sentence() {
        test_invalid(&PUZZLE_METADATA, &[String::from("a b c")], solve);
    }
}
