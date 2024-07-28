//! [aoc](https://adventofcode.com/2015/day/17)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 17,
        title: "No Such Thing as Too Much",
        solution: ("1304", "18"),
        example_solutions: vec![("4", "3")],
    }
}

type ItemType = i32;

const TOTAL: ItemType = 150;
const EXAMPLE_TOTAL: ItemType = 25;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut data = input
        .iter()
        .map(|line| {
            line.parse::<ItemType>()
                .map_err(|_| PuzzleError("Input must contain only integers".into()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    // detect puzzle example as input
    let total = if data.len() == 5 {
        EXAMPLE_TOTAL
    } else {
        TOTAL
    };
    // ---------- Part 1 + 2
    let mut ans1 = 0;
    data.sort();
    data.reverse();
    let mut counts = vec![0; data.len()];
    for i in 0..(1 << data.len()) {
        let mut n = i;
        let mut pos = 0;
        let mut sum = 0;
        let mut bits = 0;
        while n > 0 && sum < total {
            if n & 1 != 0 {
                sum += data[pos];
                bits += 1;
            }
            pos += 1;
            n >>= 1;
        }
        if n == 0 && sum == total {
            ans1 += 1;
            counts[bits] += 1;
        }
    }
    let mut i = 0;
    while i < counts.len() - 1 && counts[i] == 0 {
        i += 1;
    }
    let ans2 = counts[i];
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
    fn invalid_must_be_integers() {
        test_invalid(&vec![String::from("a")], solve);
    }
}
