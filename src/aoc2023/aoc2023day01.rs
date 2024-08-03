//! [aoc](https://adventofcode.com/2023/day/1)

use crate::aoc::{PuzzleMetaData, PuzzleInput, PuzzleResult};
use std::cmp;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 1,
        title: "Trebuchet?!",
        solution: ("56108", "55652"),
        example_solutions: vec![("142", "0"), ("0", "281")],
    }
}

const SPELLING: [(&str, &str); 9] = [
    ("1", "one"),
    ("2", "two"),
    ("3", "three"),
    ("4", "four"),
    ("5", "five"),
    ("6", "six"),
    ("7", "seven"),
    ("8", "eight"),
    ("9", "nine"),
];

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Part 1 + 2
    let ans1 = input.iter().map(|&line| border(line)).sum::<u32>();
    let ans2 = input.iter().map(|&line|{
        let mut s = line.to_owned();
        for i in 0..s.len() {
            for (value, spelling) in SPELLING {
                let end = cmp::min(s.len(), i + spelling.len());
                if s[i..end] == *spelling {
                    s = s[0..i].to_string() + value + &s[(i + 1)..];
                    break;
                }
            }
        }
        border(&s)
    }).sum::<u32>();
    Ok((ans1.to_string(), ans2.to_string()))
}

fn border(line: &str) -> u32 {
    let digits = line.chars().filter_map(|c|c.to_digit(10)).collect::<Vec<_>>();
    if digits.is_empty() {
        return 0;
    }
    10 * digits[0] + digits[digits.len() - 1]
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
    fn example2() {
        test_case(metadata, solve, 2);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }
}
