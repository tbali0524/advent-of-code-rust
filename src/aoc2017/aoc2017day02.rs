//! [aoc](https://adventofcode.com/2017/day/2)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2017,
        day: 2,
        title: "Corruption Checksum",
        solution: ("48357", "351"),
        example_solutions: vec![("18", "0"), ("0", "9")],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    // TODO fix: parse errors propagated to Result only in last line?
    let data = input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|x| {
                    x.parse::<ItemType>()
                        .map_err(|_| PuzzleError("Input must contain only integers".into()))
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<Vec<_>>, _>>()?;
    // ---------- Part 1
    let ans1: ItemType = data
        .iter()
        .map(|row| row.iter().max().unwrap_or(&0) - row.iter().min().unwrap_or(&0))
        .sum();
    // ---------- Part 2
    let mut ans2 = 0;
    for row0 in data {
        let mut row = row0.to_owned();
        row.sort();
        row.reverse();
        'to_break: for i in 0..row.len() - 1 {
            for j in (i + 1)..row.len() {
                if row[j] != 0 && row[i] % row[j] == 0 {
                    ans2 += row[i] / row[j];
                    break 'to_break;
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
    fn example2() {
        test_case(metadata, solve, 2);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_only_2d_array_of_ints() {
        test_invalid(&vec![String::from("1 2 3"), String::from("4 a 6")], solve);
    }
}
