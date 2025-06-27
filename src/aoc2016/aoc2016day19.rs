//! [aoc](https://adventofcode.com/2016/day/19)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2016,
        day: 19,
        title: "An Elephant Named Joseph",
        solution: ("1841611", "1423634"),
        example_solutions: vec![("3", "2")],
    }
}

type ItemType = u64;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() != 1 {
        Err("input must have a single line")?;
    }
    let n = input[0]
        .parse::<ItemType>()
        .map_err(|_| "input must be a single non-negative integer")?;
    // ---------- Part 1
    // <https://en.wikipedia.org/wiki/Josephus_problem#k=2>
    let mut ans1 = !1u64;
    for i in (1..=63).rev() {
        if (n & (1u64 << i)) != 0 {
            ans1 = !(1 << (i + 1)) & ((n << 1) | 1);
            break;
        }
    }
    // ---------- Part 2
    let mut ans2 = 0;
    for i in 1..n {
        ans2 = ans2 % i + 1;
        if ans2 > i.div_ceil(2) {
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
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_single_line() {
        test_invalid_msg(&["a", "b"], solve, "input must have a single line");
    }

    #[test]
    fn invalid_must_be_non_negative_integer() {
        test_invalid_msg(&["a"], solve, "input must be a single non-negative integer");
    }
}
