//! [aoc](https://adventofcode.com/2015/day/5)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 5,
        title: "Doesn't He Have Intern-Elves For This?",
        solution: ("238", "69"),
        example_solutions: vec![("2", "0"), ("0", "2")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Part 1
    let mut ans1 = 0;
    for line in input {
        if line.chars().filter(|c| "aeiou".contains(*c)).count() < 3 {
            continue;
        }
        let mut is_nice = false;
        for i in 0..26 {
            let c = (b'a' + i) as char;
            let cc = String::from(c) + &c.to_string();
            if line.contains(&cc) {
                is_nice = true;
                break;
            }
        }
        if !is_nice {
            continue;
        }
        for needle in ["ab", "cd", "pq", "xy"] {
            if line.contains(needle) {
                is_nice = false;
                break;
            }
        }
        if is_nice {
            ans1 += 1;
        }
    }
    // ---------- Part 2
    let mut ans2 = 0;
    for line in input {
        let mut first_pos = HashMap::new();
        let mut is_nice = false;
        for i in 1..line.len() {
            let pair = String::from(line.as_bytes()[i - 1] as char)
                + &String::from(line.as_bytes()[i] as char);
            if first_pos.contains_key(&pair) {
                if i - first_pos.get(&pair).unwrap() >= 2 {
                    is_nice = true;
                    break;
                }
                continue;
            }
            first_pos.insert(pair, i);
        }
        if !is_nice {
            continue;
        }
        is_nice = false;
        for i in 2..line.len() {
            if line.as_bytes()[i] == line.as_bytes()[i - 2] {
                is_nice = true;
                break;
            }
        }
        if is_nice {
            ans2 += 1;
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
    fn example2() {
        test_case(metadata, solve, 2);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }
}
