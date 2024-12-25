//! [aoc](https://adventofcode.com/2024/day/25)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 25,
        title: "Code Chronicle",
        solution: ("3317", "0"),
        example_solutions: vec![("3", "0")],
    }
}

const FILL: char = '#';
const EMPTY: char = '.';

#[allow(clippy::needless_range_loop)]
pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    let mut i = 0;
    while i + 6 < input.len() {
        let mut shape = Vec::new();
        for y in i..=(i + 6) {
            let row = input[y].chars().collect::<Vec<_>>();
            if row.len() != 5 {
                Err("input must be 5 chars wide")?;
            }
            if row.iter().any(|&x| x != FILL && x != EMPTY) {
                Err("input must contain only `#` and `.`")?;
            }
            shape.push(row);
        }
        let mut code = [0; 5];
        let is_lock = shape[0][0] == FILL;
        if is_lock {
            for x in 0..5 {
                if shape[0][x] != FILL || shape[6][x] != EMPTY {
                    Err("lock shapes must have filled top row and empty bottom row")?;
                }
                for y in 0..=5 {
                    if shape[y + 1][x] == EMPTY {
                        code[x] = y;
                        break;
                    }
                }
            }
            locks.push(code);
        } else {
            for x in 0..5 {
                if shape[0][x] != EMPTY || shape[6][x] != FILL {
                    Err("key shapes must have empty top row and filled bottom row")?;
                }
                for y in 0..=5 {
                    if shape[5 - y][x] == EMPTY {
                        code[x] = y;
                        break;
                    }
                }
            }
            keys.push(code);
        }
        if i + 7 < input.len() && !input[i + 7].is_empty() {
            Err("shapes must be separated by an empty line")?;
        }
        i += 8;
    }
    // ---------- Part 1
    let mut ans1 = 0;
    for &lock in locks.iter() {
        for &key in keys.iter() {
            if lock.iter().zip(key.iter()).all(|a| a.0 + a.1 <= 5) {
                ans1 += 1;
            }
        }
    }
    let ans2 = 0;
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
    fn invalid_must_be_5_chars() {
        test_invalid_msg(
            &[
                &"#####", &".##", &".####", &".####", &".#.#.", &".#...", &".....",
            ],
            solve,
            "input must be 5 chars wide",
        );
    }

    #[test]
    fn invalid_char() {
        test_invalid_msg(
            &[
                &"#####", &".#a##", &".####", &".####", &".#.#.", &".#...", &".....",
            ],
            solve,
            "input must contain only `#` and `.`",
        );
    }

    #[test]
    fn invalid_lock_top_bottom_row() {
        test_invalid_msg(
            &[
                &"##.##", &".####", &".####", &".####", &".#.#.", &".#...", &".....",
            ],
            solve,
            "lock shapes must have filled top row and empty bottom row",
        );
    }

    #[test]
    fn invalid_key_top_bottom_row() {
        test_invalid_msg(
            &[
                &"..#..", &"#....", &"#....", &"#...#", &"#.#.#", &"#.###", &"#####",
            ],
            solve,
            "key shapes must have empty top row and filled bottom row",
        );
    }

    #[test]
    fn invalid_missing_empty_separator() {
        test_invalid_msg(
            &[
                &"#####", &".####", &".####", &".####", &".#.#.", &".#...", &".....", &"a",
                &".....",
            ],
            solve,
            "shapes must be separated by an empty line",
        );
    }
}
