//! [aoc](https://adventofcode.com/2025/day/2)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashSet;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2025,
        day: 2,
        title: "Gift Shop",
        solution: ("40055209690", "50857215650"),
        example_solutions: vec![("1227775554", "4174379265")],
    }
}

type ItemType = u64;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() != 1 {
        Err("input must have a single line")?;
    }
    let mut ranges = Vec::new();
    for range in input[0].split(',') {
        let mut range_iter = range.split('-');
        let from = range_iter
            .next()
            .ok_or("invalid input")?
            .parse::<ItemType>()
            .map_err(|_| "range `from` must be an integer")?;
        let to = range_iter
            .next()
            .ok_or("missing `to` in range")?
            .parse::<ItemType>()
            .map_err(|_| "range `to` must be an integer")?;
        if range_iter.next().is_some() {
            Err("range must have only two parts")?;
        }
        ranges.push((from, to));
    }
    // ---------- Part 1 + 2
    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut memo = HashSet::new();
    for &(from, to) in ranges.iter() {
        let from_digits = from.checked_ilog10().unwrap_or(0) + 1;
        let to_digits = to.checked_ilog10().unwrap_or(0) + 1;
        let max_repeat = to_digits;
        for repeat in 2..=max_repeat {
            let from_chunk = from / 10u64.pow(from_digits - from_digits / repeat);
            let to_chunk = 10u64.pow(to_digits / repeat);
            for chunk in from_chunk..=to_chunk {
                let mut candidate = chunk;
                for _ in 1..repeat {
                    let mut n = chunk;
                    while n > 0 {
                        candidate *= 10;
                        n /= 10;
                    }
                    candidate += chunk;
                }
                if (from..=to).contains(&candidate) && !memo.contains(&candidate) {
                    ans2 += candidate;
                    if repeat == 2 {
                        ans1 += candidate;
                    }
                    memo.insert(candidate.to_owned());
                }
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

    #[test]
    fn invalid_from_must_be_integer() {
        test_invalid_msg(&[&"a-2"], solve, "range `from` must be an integer");
    }

    #[test]
    fn invalid_to_must_be_integer() {
        test_invalid_msg(&[&"1-a"], solve, "range `to` must be an integer");
    }

    #[test]
    fn invalid_range_must_have_2_parts() {
        test_invalid_msg(&[&"1-2-3"], solve, "range must have only two parts");
    }
}
