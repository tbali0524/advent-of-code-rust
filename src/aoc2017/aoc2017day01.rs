// https://adventofcode.com/2017/day/1

use crate::aoc::PuzzleMetaData;
use crate::aoc::PuzzleResult;

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2017,
    day: 1,
    title: "Inverse Captcha",
    solutions: (1102, 1076),
    example_solutions: [(9, 0), (0, 4)],
    example_string_inputs: ["91212129", "12131415"],
};

pub fn solve(input: &[String]) -> PuzzleResult {
    // ---------- Check input
    if input.len() != 1 {
        return Err("Input must have a single line")
    }
    let line = &input[0];
    for c in line.chars() {
        if !c.is_ascii_digit() {
            return Err("Input must contain only digits")
        }
    }
    // ---------- Part 1
    let mut ans1 = 0;
    for i in 0..line.len() {
        let c = line.chars().nth(i).unwrap();
        if c == line.chars().nth((i + 1) % line.len()).unwrap() {
            ans1 += c.to_digit(10).unwrap();
        }
    }
    // ---------- Part 2
    let mut ans2 = 0;
    for i in 0..line.len() {
        let c = line.chars().nth(i).unwrap();
        if c == line.chars().nth((i + line.len() / 2) % line.len()).unwrap() {
            ans2 += c.to_digit(10).unwrap();
        }
    }
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
    fn invalid_only_digits() {
        test_invalid(&PUZZLE_METADATA, &[String::from("1a3")], solve);
    }

    #[test]
    fn invalid_single_line() {
        test_invalid(&PUZZLE_METADATA, &[String::from("123"), String::from("1")], solve);
    }
}
