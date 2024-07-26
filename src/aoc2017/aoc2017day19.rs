//! [aoc](https://adventofcode.com/2017/day/19)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2017,
    day: 19,
    title: "A Series of Tubes",
    solution: (0, 0),
    example_solutions: [(0, 0), (0, 0)],
    string_solution: Some(("LXWCKGRAOY", "17302")),
    example_string_solutions: Some([("ABCDEF", "38"), ("", "")]),
    example_string_inputs: None,
};

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Check input
    let max_y = input.len() as i32;
    let start_x = input[0]
        .find('|')
        .ok_or("First line of input must contain a starting point |")?;
    // ---------- Part 1 + 2
    let mut ans1 = String::new();
    let mut ans2 = -1;
    let mut x = start_x as i32;
    let mut y = -1;
    let mut dx = 0;
    let mut dy = 1;
    loop {
        x += dx;
        y += dy;
        if x < 0 || y < 0 || y >= max_y || x >= input[y as usize].len() as i32 {
            break;
        }
        ans2 += 1;
        let c = input[y as usize].as_bytes()[x as usize] as char;
        match c {
            ' ' => {
                break;
            }
            '|' | '-' => {
                continue;
            }
            '+' => {
                for (dx1, dy1) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    if dx + dx1 == 0 && dy + dy1 == 0 {
                        continue;
                    }
                    let x1 = x + dx1;
                    let y1 = y + dy1;
                    if x1 < 0 || y1 < 0 || y1 >= max_y || x1 >= input[y1 as usize].len() as i32 {
                        continue;
                    }
                    if input[y1 as usize].as_bytes()[x1 as usize] as char == ' ' {
                        continue;
                    }
                    dx = dx1;
                    dy = dy1;
                    break;
                }
                continue;
            }
            _ => ans1.push(c),
        }
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
    fn invalid_no_start_position() {
        test_invalid(
            &PUZZLE_METADATA,
            &[String::from(" - "), String::from(" | ")],
            solve,
        );
    }
}
