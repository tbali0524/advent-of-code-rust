//! [aoc](https://adventofcode.com/2015/day/2)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 2,
        title: "I Was Told There Would Be No Math",
        solution: ("1606483", "3842356"),
        example_solutions: vec![("58", "34"), ("43", "14")],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let data = input
        .iter()
        .map(|line| {
            line.split('x')
                .map(|x| {
                    x.parse::<ItemType>()
                        .map_err(|_| PuzzleError("input must contain only integers".into()))
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<Vec<_>>, _>>()?;
    for row in &data {
        if row.len() != 3 {
            return Err(PuzzleError("input must contain 3 integers per line".into()));
        }
    }
    // ---------- Part 1
    let mut ans1 = 0;
    for row in &data {
        let sides = [row[0] * row[1], row[0] * row[2], row[1] * row[2]];
        ans1 += 2 * sides.iter().sum::<ItemType>() + *sides.iter().min().unwrap();
    }
    // ---------- Part 2
    let mut ans2 = 0;
    for item in &data {
        let mut row = item.clone();
        row.sort();
        ans2 += 2 * (row[0] + row[1]) + row.iter().product::<ItemType>();
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

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
        test_invalid(&[&"1x2x3", &"4xax6"], solve);
    }

    #[test]
    fn invalid_only_triplets_of_ints() {
        test_invalid(&[&"1x2x3x4"], solve);
    }
}
