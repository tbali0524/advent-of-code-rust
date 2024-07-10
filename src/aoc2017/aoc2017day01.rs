// https://adventofcode.com/2017/day/1

use crate::aoc::runner::PuzzleMetaData;

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2017,
    day: 1,
    title: "Inverse Captcha",
    solutions: (1102, 1076),
    example_solutions: [(9, 0), (0, 4)],
    example_string_inputs: ["91212129", "12131415"],
};

pub fn solve(input: &[String]) -> (String, String) {
    let line = &input[0];
    // ---------- Part 1
    let mut ans1 = 0;
    for i in 0..line.len() {
        if line.chars().nth(i) == line.chars().nth((i + 1) % line.len()) {
            ans1 += line.chars().nth(i).unwrap().to_digit(10).unwrap();
        }
    }
    // ---------- Part 2
    let mut ans2 = 0;
    for i in 0..line.len() {
        if line.chars().nth(i) == line.chars().nth((i + line.len() / 2) % line.len()) {
            ans2 += line.chars().nth(i).unwrap().to_digit(10).unwrap();
        }
    }
    (ans1.to_string(), ans2.to_string())
}

// --- boilerplate below ---

pub fn run() -> bool {
    crate::aoc::runner::run_puzzle(&PUZZLE_METADATA, solve)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::runner::tests::test_case;

    #[test]
    fn example1_works() {
        test_case(&PUZZLE_METADATA, 1, solve);
    }

    #[test]
    fn example2_works() {
        test_case(&PUZZLE_METADATA, 2, solve);
    }

    #[test]
    fn puzzle_works() {
        test_case(&PUZZLE_METADATA, 0, solve);
    }
}
