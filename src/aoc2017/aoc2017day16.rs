//! [aoc](https://adventofcode.com/2017/day/16)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2017,
        day: 16,
        title: "Permutation Promenade",
        solution: ("eojfmbpkldghncia", "iecopnahgdflmkjb"),
        example_solutions: vec![("baedc", "0")],
    }
}

type ItemType = usize;

#[derive(Default)]
struct Command {
    command: char,
    op1: Option<ItemType>,
    op2: Option<ItemType>,
    op1s: Option<char>,
    op2s: Option<char>,
}

#[allow(clippy::field_reassign_with_default)]
pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() != 1 {
        return Err(PuzzleError("Input must have a single line".into()));
    }
    let instructions = input[0]
        .split(',')
        .map(|x| x.to_owned())
        .collect::<Vec<String>>();
    let mut commands = Vec::new();
    for instruction in &instructions {
        let mut command = Command::default();
        command.command = instruction.chars().next().unwrap();
        let mut s = instruction[1..].split('/');
        let a = s.next().unwrap();
        match command.command {
            's' => {
                command.op1 = Some(a.parse::<ItemType>().map_err(|_| {
                    PuzzleError("Invalid input: s argument must be integer".into())
                })?);
            }
            'x' => {
                command.op1 = Some(a.parse::<ItemType>().map_err(|_| {
                    PuzzleError("Invalid input: x first argument must be integer".into())
                })?);
                let b = s.next().ok_or(PuzzleError(
                    "Invalid input: x command needs 2 arguments".into(),
                ))?;
                command.op2 = Some(b.parse::<ItemType>().map_err(|_| {
                    PuzzleError("Invalid input: x second argument must be integer".into())
                })?);
            }
            'p' => {
                command.op1s = Some(a.chars().next().unwrap());
                let b = s.next().ok_or(PuzzleError(
                    "Invalid input: x command needs 2 arguments".into(),
                ))?;
                command.op2s = Some(b.chars().next().unwrap());
            }
            _ => {
                return Err(PuzzleError("Command must be s, x, or p".into()));
            }
        }
        commands.push(command);
    }
    let start = if instructions.len() == 3 {
        "abcde"
    } else {
        "abcdefghijklmnop"
    };
    // ---------- Part 1
    let mut progs = start.as_bytes().to_vec();
    for c in &commands {
        match c.command {
            's' => {
                progs.rotate_right(c.op1.unwrap());
            }
            'x' => {
                progs.swap(c.op1.unwrap(), c.op2.unwrap());
            }
            'p' => {
                let pos1 = progs
                    .iter()
                    .position(|&x| x == c.op1s.unwrap() as u8)
                    .ok_or(PuzzleError("Impossible".into()))?;
                let pos2 = progs
                    .iter()
                    .position(|&x| x == c.op2s.unwrap() as u8)
                    .ok_or(PuzzleError("Impossible".into()))?;
                progs.swap(pos1, pos2);
            }
            _ => {
                return Err(PuzzleError("Command must be s, x, or p".into()));
            }
        }
    }
    let ans1 = String::from_utf8(progs).unwrap();
    // ---------- Part 2
    progs = start.as_bytes().to_vec();
    let mut seen_at = HashMap::from([(progs.to_owned(), 0)]);
    let mut turn = 1;
    const MAX_STEPS_PART2: usize = 1_000_000_000;
    while turn <= MAX_STEPS_PART2 {
        for c in &commands {
            match c.command {
                's' => {
                    progs.rotate_right(c.op1.unwrap());
                }
                'x' => {
                    progs.swap(c.op1.unwrap(), c.op2.unwrap());
                }
                'p' => {
                    let pos1 = progs
                        .iter()
                        .position(|&x| x == c.op1s.unwrap() as u8)
                        .ok_or(PuzzleError("Impossible".into()))?;
                    let pos2 = progs
                        .iter()
                        .position(|&x| x == c.op2s.unwrap() as u8)
                        .ok_or(PuzzleError("Impossible".into()))?;
                    progs.swap(pos1, pos2);
                }
                _ => {
                    return Err(PuzzleError("Command must be s, x, or p".into()));
                }
            }
        }
        if !seen_at.contains_key(&progs) {
            seen_at.insert(progs.to_owned(), turn);
            turn += 1;
            continue;
        }
        let cycle_len = turn - seen_at.get(&progs).unwrap();
        let cycle_count = (MAX_STEPS_PART2 - turn) / cycle_len;
        turn += cycle_count * cycle_len + 1;
    }
    let ans2 = String::from_utf8(progs).unwrap();
    Ok((ans1, ans2))
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
    fn invalid_single_line() {
        test_invalid(&vec![String::from("a"), String::from("b")], solve);
    }

    #[test]
    fn invalid_command() {
        test_invalid(&vec![String::from("a1")], solve);
    }

    #[test]
    fn invalid_command_s_argument_must_be_int() {
        test_invalid(&vec![String::from("sa")], solve);
    }

    #[test]
    fn invalid_command_x_must_have_2_arguments() {
        test_invalid(&vec![String::from("p1")], solve);
    }

    #[test]
    fn invalid_command_x_argument_must_be_int() {
        test_invalid(&vec![String::from("pa/1")], solve);
    }
}
