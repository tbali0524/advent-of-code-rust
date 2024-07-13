// https://adventofcode.com/2017/day/13

use crate::aoc::PuzzleMetaData;
use crate::aoc::PuzzleResult;
use std::collections::HashMap;

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2017,
    day: 13,
    title: "Packet Scanners",
    solution: (1728, 3946838),
    example_solutions: [(24, 10), (0, 0)],
    string_solution: None,
    example_string_solutions: None,
    example_string_inputs: None,
};

type ItemType = i32;

pub fn solve(input: &[String]) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut scanners = HashMap::new();
    for line in input {
        let a = line
            .split(": ")
            .map(|x| {
                x.parse::<ItemType>()
                    .map_err(|_| "Input must contain only integers")
            })
            .collect::<Result<Vec<_>, _>>()?;
        if a.len() != 2 {
            return Err("Input lines must have 2 items separated by a : and a whitespace");
        }
        scanners.insert(a[0], a[1]);
    }
    // ---------- Part 1
    let mut ans1 = 0;
    for (depth, range) in &scanners {
        if depth % (2 * (range - 1)) == 0 {
            ans1 += depth * range;
        }
    }
    // ---------- Part 2
    let mut ans2 = 0;
    loop {
        let mut is_ok = true;
        for (depth, range) in &scanners {
            if (depth + ans2) % (2 * (range - 1)) == 0 {
                is_ok = false;
                break;
            }
        }
        if is_ok {
            break;
        }
        ans2 += 1;
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
    fn invalid_must_have_two_data_per_line() {
        test_invalid(&PUZZLE_METADATA, &[String::from("1: 2: 3")], solve);
    }

    #[test]
    fn invalid_only_contains_int() {
        test_invalid(&PUZZLE_METADATA, &[String::from("1: a")], solve);
    }
}
