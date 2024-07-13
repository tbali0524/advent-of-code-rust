// https://adventofcode.com/2015/day/2

use crate::aoc::PuzzleMetaData;
use crate::aoc::PuzzleResult;

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2015,
    day: 2,
    title: "I Was Told There Would Be No Math",
    solution: (1606483, 3842356),
    example_solutions: [(58, 34), (43, 14)],
    string_solution: None,
    example_string_solutions: None,
    example_string_inputs: Some(["2x3x4", "1x1x10"]),
};

type ItemType = i32;

pub fn solve(input: &[String]) -> PuzzleResult {
    // ---------- Parse and Check input
    let data = input
        .iter()
        .map(|line| {
            line.split('x')
                .map(|x| {
                    x.parse::<ItemType>()
                        .map_err(|_| "Input must contain only integers")
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<Vec<_>>, _>>()?;
    for row in &data {
        if row.len() != 3 {
            return Err("Input must contain 3 integers per line");
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
    fn example2() {
        test_case(&PUZZLE_METADATA, 2, solve);
    }

    #[test]
    fn puzzle() {
        test_case(&PUZZLE_METADATA, 0, solve);
    }

    #[test]
    fn invalid_only_2d_array_of_ints() {
        test_invalid(
            &PUZZLE_METADATA,
            &[String::from("1x2x3"), String::from("4xax6")],
            solve,
        );
    }

    #[test]
    fn invalid_only_triplets_of_ints() {
        test_invalid(&PUZZLE_METADATA, &[String::from("1x2x3x4")], solve);
    }
}
