//! [aoc](https://adventofcode.com/2016/day/3)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2016,
        day: 3,
        title: "Squares With Three Sides",
        solution: ("982", "1826"),
        example_solutions: Vec::new(),
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let data = input
        .iter()
        .map(|&line| {
            line.split_whitespace()
                .map(|x| {
                    x.parse::<ItemType>()
                        .map_err(|_| format!("input must contain only integers, found `{}`", x))
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<Vec<_>>, _>>()?;
    if data.len() % 3 != 0 {
        Err("number of rows must be multiple of 3")?;
    }
    if data.iter().any(|x| x.len() != 3) {
        Err("each row must contain 3 integers")?;
    }
    // ---------- Part 1
    let mut ans1 = 0;
    for row in data.iter() {
        let mut sides = row.clone();
        sides.sort();
        if sides[0] + sides[1] > sides[2] {
            ans1 += 1;
        }
    }
    // ---------- Part 2
    let mut ans2 = 0;
    for x in 0..3 {
        let mut y = 0;
        while y < data.len() {
            let mut sides = [data[y][x], data[y + 1][x], data[y + 2][x]];
            sides.sort();
            if sides[0] + sides[1] > sides[2] {
                ans2 += 1;
            }
            y += 3;
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
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_must_contain_integer() {
        test_invalid_msg(
            &[&"1 a 3"],
            solve,
            "input must contain only integers, found ",
        );
    }

    #[test]
    fn invalid_must_have_multiple_of_3_rows() {
        test_invalid_msg(&[&"1 2 3"], solve, "number of rows must be multiple of 3");
    }

    #[test]
    fn invalid_must_containg_3_integers_per_line() {
        test_invalid_msg(
            &[&"1 2 3", &"4 5", &"7 8 9"],
            solve,
            "each row must contain 3 integers",
        );
    }
}
