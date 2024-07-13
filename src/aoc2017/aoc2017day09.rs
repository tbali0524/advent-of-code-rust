// https://adventofcode.com/2017/day/9

use crate::aoc::PuzzleMetaData;
use crate::aoc::PuzzleResult;

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2017,
    day: 9,
    title: "Stream Processing",
    solution: (14204, 6622),
    example_solutions: [(50, 0), (0, 32)],
    string_solution: None,
    example_string_solutions: None,
    example_string_inputs: None,
};

pub fn solve(input: &[String]) -> PuzzleResult {
    // ---------- Part 1 + 2
    let mut ans1 = 0;
    let mut ans2 = 0;
    for line in input {
        let mut depth = 0;
        let mut in_ignore = false;
        let mut in_garbage = false;
        for c in line.chars() {
            if in_ignore {
                in_ignore = false;
            } else if c == '!' {
                in_ignore = true;
                continue;
            } else if in_garbage {
                if c == '>' {
                    in_garbage = false;
                    continue;
                }
                ans2 += 1;
                continue;
            } else {
                match c {
                    '<' => {
                        in_garbage = true;
                    }
                    '{' => {
                        depth += 1;
                    }
                    '}' => {
                        ans1 += depth;
                        depth -= 1
                    }
                    _ => (),
                };
            }
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
    fn puzzle() {
        test_case(&PUZZLE_METADATA, 0, solve);
    }
}
