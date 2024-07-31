//! [aoc](https://adventofcode.com/2019/day/1)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2019,
        day: 1,
        title: "The Tyranny of the Rocket Equation",
        solution: ("3287620", "4928567"),
        example_solutions: vec![("34241", "51316")],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let data = input
        .iter()
        .map(|line| {
            line.parse::<ItemType>()
                .map_err(|_| PuzzleError("Input must contain only integers".into()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    // ---------- Part 1
    let ans1 = data.iter().map(|x| x / 3 - 2).sum::<ItemType>();
    // ---------- Part 2
    let mut ans2 = 0;
    for mass in &data {
        let mut total = 0;
        let mut fuel = *mass;
        loop {
            fuel = std::cmp::max(0, fuel / 3 - 2);
            total += fuel;
            if fuel == 0 {
                break;
            }
        }
        ans2 += total;
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
    fn invalid_only_contains_int() {
        test_invalid(&[&"a"], solve);
    }
}
