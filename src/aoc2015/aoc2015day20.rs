//! [aoc](https://adventofcode.com/2015/day/20)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 20,
        title: "Infinite Elves and Infinite Houses",
        solution: ("831600", "884520"),
        example_solutions: vec![("8", "0"), ("6", "0")],
    }
}

type ItemType = usize;

const MAX_PART2: usize = 50;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Check input
    if input.len() != 1 {
        Err("input must have a single line")?;
    }
    let data = input[0]
        .parse::<ItemType>()
        .map_err(|_| format!("input must be an integer, found `{}`", input[0]))?;
    // ---------- Part 1
    let n = data / 10;
    let mut h = vec![0 as ItemType; n];
    for i in 1..n {
        let mut j = i;
        while j < n {
            h[j] += i;
            j += i
        }
    }
    let mut i = 1;
    while h[i] < n {
        i += 1;
    }
    let ans1 = i;
    // ---------- Part 2
    let n = data / 11;
    let mut h = vec![0; n];
    for i in 1..n {
        let mut count = 0;
        let mut j = i;
        while j < n {
            h[j] += i;
            count += 1;
            if count == MAX_PART2 {
                break;
            }
            j += i
        }
    }
    let mut i = 1;
    while h[i] < n {
        i += 1;
    }
    let ans2 = i;
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
        test_invalid(&["1", "2"], solve);
    }

    #[test]
    fn invalid_must_be_integer() {
        test_invalid(&["a"], solve);
    }
}
