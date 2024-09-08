//! [aoc](https://adventofcode.com/2023/day/9)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 9,
        title: "Mirage Maintenance",
        solution: ("1861775706", "1082"),
        example_solutions: vec![("114", "2")],
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
    // ---------- Part 1
    let ans1 = data.iter().map(|x| extrapolate_next(x)).sum::<ItemType>();
    // ---------- Part 2
    let ans2 = data.iter().map(|x| extrapolate_prev(x)).sum::<ItemType>();
    Ok((ans1.to_string(), ans2.to_string()))
}

fn extrapolate_next(a: &[ItemType]) -> ItemType {
    if a.iter().filter(|&x| *x != 0).count() == 0 {
        return 0;
    }
    let diff = a.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    a.last().unwrap() + extrapolate_next(&diff)
}

fn extrapolate_prev(a: &[ItemType]) -> ItemType {
    if a.iter().filter(|&x| *x != 0).count() == 0 {
        return 0;
    }
    let diff = a.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    a.first().unwrap() - extrapolate_prev(&diff)
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
    fn invalid_must_contain_integers() {
        test_invalid_msg(&[&"1 a"], solve, "input must contain only integers");
    }
}
