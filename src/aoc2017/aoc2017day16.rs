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

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() != 1 {
        Err("input must have a single line")?;
    }
    let instructions = input[0].split(',').map(String::from).collect::<Vec<_>>();
    let mut commands = Vec::with_capacity(instructions.len());
    for instruction in &instructions {
        let command = Command::try_from(instruction.as_str())?;
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
                    .ok_or("impossible")?;
                let pos2 = progs
                    .iter()
                    .position(|&x| x == c.op2s.unwrap() as u8)
                    .ok_or("impossible")?;
                progs.swap(pos1, pos2);
            }
            _ => {
                Err(format!("command must be s, x, or p, found `{}`", c.command))?;
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
                        .ok_or("impossible")?;
                    let pos2 = progs
                        .iter()
                        .position(|&x| x == c.op2s.unwrap() as u8)
                        .ok_or("impossible")?;
                    progs.swap(pos1, pos2);
                }
                _ => {
                    Err(format!("command must be s, x, or p, found `{}`", c.command))?;
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

#[derive(Default)]
struct Command {
    command: char,
    op1: Option<ItemType>,
    op2: Option<ItemType>,
    op1s: Option<char>,
    op2s: Option<char>,
}

impl TryFrom<&str> for Command {
    type Error = PuzzleError;

    #[expect(clippy::field_reassign_with_default)]
    fn try_from(instruction: &str) -> Result<Command, PuzzleError> {
        let mut command = Command::default();
        command.command = instruction.chars().next().unwrap();
        let mut s = instruction[1..].split('/');
        let a = s.next().unwrap();
        match command.command {
            's' => {
                command.op1 = Some(
                    a.parse::<ItemType>()
                        .map_err(|_| "s argument must be integer")?,
                );
            }
            'x' => {
                command.op1 = Some(
                    a.parse::<ItemType>()
                        .map_err(|_| "x first argument must be integer")?,
                );
                let b = s.next().ok_or("x command needs 2 arguments")?;
                command.op2 = Some(
                    b.parse::<ItemType>()
                        .map_err(|_| "x second argument must be integer")?,
                );
            }
            'p' => {
                command.op1s = Some(a.chars().next().unwrap());
                let b = s
                    .next()
                    .ok_or("Invalid input: x command needs 2 arguments")?;
                command.op2s = Some(b.chars().next().unwrap());
            }
            _ => {
                Err(format!(
                    "command must be s, x, or p, found `{}`",
                    command.command
                ))?;
            }
        }
        Ok(command)
    }
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
        test_invalid(&[&"a", &"b"], solve);
    }

    #[test]
    fn invalid_command() {
        test_invalid(&[&"a1"], solve);
    }

    #[test]
    fn invalid_command_s_argument_must_be_int() {
        test_invalid(&[&"sa"], solve);
    }

    #[test]
    fn invalid_command_x_must_have_2_arguments() {
        test_invalid(&[&"p1"], solve);
    }

    #[test]
    fn invalid_command_x_argument_must_be_int() {
        test_invalid(&[&"pa/1"], solve);
    }
}
