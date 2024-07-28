//! [aoc](https://adventofcode.com/2015/day/4)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use md5;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 4,
        title: "The Ideal Stocking Stuffer",
        solution: ("254575", "1038736"),
        // examples excluded because taking +8 sec in release mode
        example_solutions: vec![],
        // example_solutions: vec![("609043", "0"), ("1048970", "0")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Check input
    if input.len() != 1 {
        return Err(PuzzleError("Input must have a single line".into()));
    }
    // ---------- Part 1
    let mut ans1 = 1;
    loop {
        let item = input[0].to_owned() + &ans1.to_string();
        let hash = format!("{:x}", md5::compute(&item));
        if &hash[0..5] == "00000" {
            break;
        }
        ans1 += 1;
    }
    // ---------- Part 2
    let mut ans2 = 1;
    loop {
        let item = input[0].to_owned() + &ans2.to_string();
        let hash = format!("{:x}", md5::compute(&item));
        if &hash[0..6] == "000000" {
            break;
        }
        ans2 += 1;
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::runner::tests::*;

    #[test]
    #[ignore]
    fn example1() {
        test_case(metadata, solve, 1);
    }

    #[test]
    #[ignore]
    fn example2() {
        test_case(metadata, solve, 2);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_single_line() {
        test_invalid(&vec![String::from("a"), String::from("b")], solve);
    }
}
