//! [aoc](https://adventofcode.com/2017/day/5)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2017,
        day: 5,
        title: "A Maze of Twisty Trampolines, All Alike",
        solution: ("396086", "28675390"),
        example_solutions: vec![("5", "10")],
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
