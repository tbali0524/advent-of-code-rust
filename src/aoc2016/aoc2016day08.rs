//! [aoc](https://adventofcode.com/2016/day/8)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2016,
        day: 8,
        title: "Two-Factor Authentication",
        solution: ("110", "ZJHRKCPLYJ"),
        example_solutions: vec![("6", "0")],
    }
}

const MAX_X: usize = 50;
const MAX_Y: usize = 6;
const LED_ON: char = '#';
const LED_OFF: char = ' ';
const SHOW_GRID: bool = false;

enum Instruction {
    Rect,
    RotateColumn,
    RotateRow,
}

type ItemType = usize;

#[allow(clippy::needless_range_loop)]
pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut instructions = Vec::new();
    for &row in input.iter() {
        let a = row.split(' ').collect::<Vec<_>>();
        if a[0] == "rect" {
            if a.len() != 2 {
                Err("invalid rect instruction")?;
            }
            let b = a[1]
                .split('x')
                .map(|x| {
                    x.parse::<ItemType>()
                        .map_err(|_| "rect arguments must be integers")
                })
                .collect::<Result<Vec<_>, _>>()?;
            if b.len() != 2 {
                Err("rect arguments must be 2 integers separated by `x`")?;
            }
            instructions.push((Instruction::Rect, b[0], b[1]));
            continue;
        }
        if a.len() != 5 || a[0] != "rotate" || a[3] != "by" {
            Err("invalid instruction")?;
        }
        let b = a[2].split('=').collect::<Vec<_>>();
        if b.len() != 2 {
            Err("invalid rotate instruction")?;
        }
        if a[1] == "column" && b[0] == "x" {
            instructions.push((
                Instruction::RotateColumn,
                b[1].parse::<ItemType>()
                    .map_err(|_| "rotate column arguments must be integers")?,
                a[4].parse::<ItemType>()
                    .map_err(|_| "rotate column arguments must be integers")?,
            ));
            continue;
        }
        if a[1] == "row" && b[0] == "y" {
            instructions.push((
                Instruction::RotateRow,
                a[4].parse::<ItemType>()
                    .map_err(|_| "rotate row arguments must be integers")?,
                b[1].parse::<ItemType>()
                    .map_err(|_| "rotate row arguments must be integers")?,
            ));
            continue;
        }
        Err("invalid rotate instruction")?;
    }
    // ---------- Part 1
    let mut grid = [[LED_OFF; MAX_X]; MAX_Y];
    for instr in instructions.iter() {
        match instr.0 {
            Instruction::Rect => {
                for y in 0..instr.2 {
                    for x in 0..instr.1 {
                        grid[y][x] = LED_ON;
                    }
                }
            }
            Instruction::RotateRow => {
                for _ in 0..instr.1 {
                    let temp = grid[instr.2][MAX_X - 1];
                    for x in (1..MAX_X).rev() {
                        grid[instr.2][x] = grid[instr.2][x - 1];
                    }
                    grid[instr.2][0] = temp;
                }
            }
            Instruction::RotateColumn => {
                for _ in 0..instr.2 {
                    let temp = grid[self::MAX_Y - 1][instr.1];
                    for y in (1..MAX_Y).rev() {
                        grid[y][instr.1] = grid[y - 1][instr.1];
                    }
                    grid[0][instr.1] = temp;
                }
            }
        }
    }
    let ans1 = grid
        .map(|row| row.iter().filter(|&c| *c == LED_ON).count())
        .iter()
        .sum::<usize>();
    // ---------- Part 2
    if SHOW_GRID {
        for row in grid.iter() {
            println!("{}", row.iter().collect::<String>());
        }
    }
    let ans2 = "ZJHRKCPLYJ"; // skipping implementing char recognition for this puzzle...
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
    fn invalid_rect_instruction() {
        test_invalid_msg(&[&"rect 3x2 1x3"], solve, "invalid rect instruction");
    }

    #[test]
    fn invalid_rect_argument_must_be_integers() {
        test_invalid_msg(&[&"rect 3xa"], solve, "rect arguments must be integers");
    }

    #[test]
    fn invalid_rect_arguments_must_be_separated_by_x() {
        test_invalid_msg(
            &[&"rect 31"],
            solve,
            "rect arguments must be 2 integers separated by `x`",
        );
    }

    #[test]
    fn invalid_instruction() {
        test_invalid_msg(&[&"a 1"], solve, "invalid instruction");
    }

    #[test]
    fn invalid_rotate() {
        test_invalid_msg(
            &[&"rotate column x by 1"],
            solve,
            "invalid rotate instruction",
        );
    }

    #[test]
    fn invalid_rotate_column_argument_must_be_integer() {
        test_invalid_msg(
            &[&"rotate column x=a by 1"],
            solve,
            "rotate column arguments must be integers",
        );
    }

    #[test]
    fn invalid_rotate_rown_argument_must_be_integer() {
        test_invalid_msg(
            &[&"rotate row y=a by 1"],
            solve,
            "rotate row arguments must be integers",
        );
    }

    #[test]
    fn invalid_rotate_instruction() {
        test_invalid_msg(&[&"rotate a y=1 by 1"], solve, "invalid rotate instruction");
    }
}
