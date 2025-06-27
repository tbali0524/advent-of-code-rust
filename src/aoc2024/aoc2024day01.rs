//! [aoc](https://adventofcode.com/2024/day/1)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::iter::zip;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 1,
        title: "Historian Hysteria",
        solution: ("936063", "23150395"),
        example_solutions: vec![("11", "31")],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut left = Vec::new();
    let mut right = Vec::new();
    for &row in input.iter() {
        let mut row_iter = row.split_whitespace();
        let left_item = row_iter
            .next()
            .unwrap()
            .parse::<ItemType>()
            .map_err(|_| "input must contain only integers")?;
        let right_item = row_iter
            .next()
            .ok_or("input lines must contain two items, one found")?
            .parse::<ItemType>()
            .map_err(|_| "input must contain only integers")?;
        if row_iter.next().is_some() {
            Err("input lines must contain two items, more found")?;
        }
        left.push(left_item);
        right.push(right_item);
    }
    // ---------- Part 1
    left.sort();
    right.sort();
    let ans1 = zip(left.iter(), right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<ItemType>();
    // ---------- Part 2
    let mut ans2 = 0;
    let mut start = 0;
    for &left_item in &left {
        let mut count = 0;
        let mut j = start;
        while j < right.len() && right[j] < left_item {
            j += 1;
        }
        start = j;
        while j < right.len() && right[j] == left_item {
            count += 1;
            j += 1;
        }
        ans2 += left_item * count;
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
    fn invalid_must_have_contain_integers_first() {
        test_invalid_msg(&["a 2"], solve, "input must contain only integers");
    }

    #[test]
    fn invalid_must_have_contain_integers_second() {
        test_invalid_msg(&["1 a"], solve, "input must contain only integers");
    }

    #[test]
    fn invalid_must_have_2_items_1_found() {
        test_invalid_msg(
            &["1"],
            solve,
            "input lines must contain two items, one found",
        );
    }

    #[test]
    fn invalid_must_have_2_items_more_found() {
        test_invalid_msg(
            &["1 2 3"],
            solve,
            "input lines must contain two items, more found",
        );
    }
}
