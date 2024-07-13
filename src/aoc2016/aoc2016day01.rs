// https://adventofcode.com/2016/day/1

use crate::aoc::PuzzleMetaData;
use crate::aoc::PuzzleResult;
use std::collections::HashSet;

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2016,
    day: 1,
    title: "No Time for a Taxicab",
    solution: (262, 131),
    example_solutions: [(8, 4), (0, 0)],
    string_solution: None,
    example_string_solutions: None,
    example_string_inputs: Some(["R8, R4, R4, R8", ""]),
};

pub fn solve(input: &[String]) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() != 1 {
        return Err("Input must have a single line");
    }
    let instructions = input[0]
        .split(", ")
        .map(|x| {
            if x.len() < 2 {
                Err("Instruction must be at least 2 digits")
            } else if !['R', 'L'].contains(&x.chars().next().unwrap()) {
                Err("Instruction must start with R or L")
            } else {
                let moves_result = x[1..].parse::<i32>();
                if let Ok(moves) = moves_result {
                    Ok((if x.starts_with('R') { 1i32 } else { -1 }, moves))
                } else {
                    Err("Instruction must contain number of moves as integer")
                }
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    // ---------- Part 1
    const DELTAS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut x = 0;
    let mut y = 0;
    let mut direction = 0; // N
    for (turn, moves) in &instructions {
        direction = (direction + turn + 4) % 4;
        let (dx, dy) = DELTAS[direction as usize];
        x += dx * moves;
        y += dy * moves;
    }
    let ans1 = x.abs() + y.abs();
    // ---------- Part 2
    let mut ans2 = 0;
    let mut x = 0;
    let mut y = 0;
    let mut direction = 0; // N
    let mut memo = HashSet::new();
    memo.insert((x, y));
    'outer: for (turn, moves) in &instructions {
        direction = (direction + turn + 4) % 4;
        let (dx, dy) = DELTAS[direction as usize];
        for _ in 0..*moves as usize {
            x += dx;
            y += dy;
            if memo.contains(&(x, y)) {
                ans2 = x.abs() + y.abs();
                break 'outer;
            }
            memo.insert((x, y));
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
    fn invalid_single_line() {
        test_invalid(
            &PUZZLE_METADATA,
            &[String::from("R8"), String::from("R4")],
            solve,
        );
    }
}
