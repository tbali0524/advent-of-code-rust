// https://adventofcode.com/2019/day/1

use crate::aoc::PuzzleMetaData;
use crate::aoc::PuzzleResult;

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2019,
    day: 1,
    title: "The Tyranny of the Rocket Equation",
    solutions: (3287620, 4928567),
    example_solutions: [(34241, 51316), (0, 0)],
    example_string_inputs: ["", ""],
};

type ItemType = i32;

pub fn solve(input: &[String]) -> PuzzleResult {
    // ---------- Parse and Check input
    let data = input
        .iter()
        .map(|line| {
            line.parse::<ItemType>()
                .map_err(|_| "Input must contain only integers")
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
// --- boilerplate below ---

pub fn run() -> bool {
    crate::aoc::runner::run_puzzle(&PUZZLE_METADATA, solve)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::runner::tests::*;

    #[test]
    fn example1() {
        test_case(&PUZZLE_METADATA, 1, solve);
    }

    #[test]
    fn puzzle() {
        test_case(&PUZZLE_METADATA, 0, solve);
    }

    #[test]
    fn invalid_only_contains_int() {
        test_invalid(&PUZZLE_METADATA, &[String::from("a")], solve);
    }
}
