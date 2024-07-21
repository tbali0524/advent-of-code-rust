//! [aoc](https://adventofcode.com/2017/day/16)

use crate::aoc::PuzzleMetaData;
use crate::aoc::PuzzleResult;
use std::collections::HashMap;

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2017,
    day: 16,
    title: "Permutation Promenade",
    solution: (0, 0),
    example_solutions: [(0, 0), (0, 0)],
    string_solution: Some(("eojfmbpkldghncia", "iecopnahgdflmkjb")),
    example_string_solutions: Some([("baedc", "0"), ("0", "0")]),
    example_string_inputs: None,
};

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
pub fn solve(input: &[String]) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() != 1 {
        return Err("Input must have a single line");
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
                command.op1 = Some(
                    a.parse::<ItemType>()
                        .map_err(|_| "Invalid input: s argument must be integer")?,
                );
            }
            'x' => {
                command.op1 = Some(
                    a.parse::<ItemType>()
                        .map_err(|_| "Invalid input: x first argument must be integer")?,
                );
                let b = s
                    .next()
                    .ok_or("Invalid input: x command needs 2 arguments")?;
                command.op2 = Some(
                    b.parse::<ItemType>()
                        .map_err(|_| "Invalid input: x second argument must be integer")?,
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
                return Err("Command must be s, x, or p");
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
                    .ok_or("Impossible")?;
                let pos2 = progs
                    .iter()
                    .position(|&x| x == c.op2s.unwrap() as u8)
                    .ok_or("Impossible")?;
                progs.swap(pos1, pos2);
            }
            _ => {
                return Err("Command must be s, x, or p");
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
                        .ok_or("Impossible")?;
                    let pos2 = progs
                        .iter()
                        .position(|&x| x == c.op2s.unwrap() as u8)
                        .ok_or("Impossible")?;
                    progs.swap(pos1, pos2);
                }
                _ => {
                    return Err("Command must be s, x, or p");
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
            &[String::from("a"), String::from("b")],
            solve,
        );
    }

    #[test]
    fn invalid_command() {
        test_invalid(&PUZZLE_METADATA, &[String::from("a1")], solve);
    }

    #[test]
    fn invalid_command_s_argument_must_be_int() {
        test_invalid(&PUZZLE_METADATA, &[String::from("sa")], solve);
    }

    #[test]
    fn invalid_command_x_must_have_2_arguments() {
        test_invalid(&PUZZLE_METADATA, &[String::from("p1")], solve);
    }

    #[test]
    fn invalid_command_x_argument_must_be_int() {
        test_invalid(&PUZZLE_METADATA, &[String::from("pa/1")], solve);
    }
}
