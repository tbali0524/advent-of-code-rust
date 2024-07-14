//! [aoc](https://adventofcode.com/2017/day/5)

use crate::aoc::PuzzleMetaData;
use crate::aoc::PuzzleResult;

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2017,
    day: 5,
    title: "A Maze of Twisty Trampolines, All Alike",
    solution: (396086, 28675390),
    example_solutions: [(5, 10), (0, 0)],
    string_solution: None,
    example_string_solutions: None,
    example_string_inputs: None,
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
    let mut ans1 = 0;
    let mut data1 = data.clone();
    let mut pc = 0;
    loop {
        if pc < 0 || pc >= data1.len() as ItemType {
            break;
        }
        ans1 += 1;
        let delta = data1[pc as usize];
        data1[pc as usize] += 1;
        pc += delta;
    }
    // ---------- Part 2
    let mut ans2 = 0;
    let mut data2 = data.clone();
    let mut pc = 0;
    loop {
        if pc < 0 || pc >= data1.len() as ItemType {
            break;
        }
        ans2 += 1;
        let delta = data2[pc as usize];
        data2[pc as usize] += if data2[pc as usize] >= 3 { -1 } else { 1 };
        pc += delta;
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
