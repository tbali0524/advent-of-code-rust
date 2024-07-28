//! [aoc](https://adventofcode.com/2017/day/9)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2017,
        day: 9,
        title: "Stream Processing",
        solution: ("14204", "6622"),
        example_solutions: vec![("50", "0"), ("0", "32")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
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
}
