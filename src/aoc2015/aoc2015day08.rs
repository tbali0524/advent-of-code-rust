//! [aoc](https://adventofcode.com/2015/day/8)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 8,
        title: "Matchsticks",
        solution: ("1371", "2117"),
        example_solutions: vec![("12", "19")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Part 1
    let mut ans1 = 0;
    for line in input {
        ans1 += line.len();
        let mut i = 0;
        while i < line.len() - 2 {
            i += 1;
            ans1 -= 1;
            if line.as_bytes()[i] == b'\\' {
                if line.as_bytes()[i + 1] == b'\\' || line.as_bytes()[i + 1] == b'\"' {
                    i += 1;
                    continue;
                }
                if line.as_bytes()[i + 1] == b'x' {
                    i += 3;
                    continue;
                }
            }
        }
    }
    // ---------- Part 2
    let mut ans2 = 0;
    for line in input {
        ans2 += line.as_bytes().iter().filter(|&x| *x == b'\"').count()
            + line.as_bytes().iter().filter(|&x| *x == b'\\').count()
            + 2;
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
