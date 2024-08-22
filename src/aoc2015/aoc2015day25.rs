//! [aoc](https://adventofcode.com/2015/day/25)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 25,
        title: "Let It Snow",
        solution: ("19980801", "0"),
        example_solutions: vec![("31916031", "0"), ("27995004", "0")],
    }
}

type ItemType = u64;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() != 1 {
        Err("input must have a single line")?;
    }
    let a = input[0].split_whitespace().collect::<Vec<_>>();
    if a.len() != 18 {
        Err("invalid input sentence")?;
    }
    let row = a[15][0..a[15].len() - 1]
        .parse::<ItemType>()
        .map_err(|_| "invalid input")?;
    let column = a[17][0..a[17].len() - 1]
        .parse::<ItemType>()
        .map_err(|_| "invalid input")?;
    // ---------- Part 1 + 2
    let n = row + column - 2;
    let steps = (n * (n + 1)) / 2 + column - 1;
    let mut ans1: ItemType = 20151125;
    for _ in 0..steps {
        ans1 = (ans1 * 252533) % 33554393;
    }
    let ans2 = 0;
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

    #[test]
    fn invalid_single_line() {
        test_invalid(&[&"a", &"b"], solve);
    }

    #[test]
    fn invalid_input_sentence() {
        test_invalid(&[&"a b c"], solve);
    }
}
