//! [aoc](https://adventofcode.com/2015/day/23)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 23,
        title: "Opening the Turing Lock",
        solution: ("170", "247"),
        example_solutions: vec![("2", "2")],
    }
}

type ItemType = i64;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Part 1 + 2
    let mut registers = HashMap::from([('a', 0), ('b', 0)]);
    let ans1 = execute(input, &mut registers)?;
    let mut registers = HashMap::from([('a', 1), ('b', 0)]);
    let ans2 = execute(input, &mut registers)?;
    Ok((ans1.to_string(), ans2.to_string()))
}

fn execute(
    input: PuzzleInput,
    registers: &mut HashMap<char, ItemType>,
) -> Result<ItemType, PuzzleError> {
    let mut pc = -1;
    loop {
        pc += 1;
        if pc < 0 || pc >= input.len() as ItemType {
            return Ok(*registers.get(&'b').unwrap_or(&0));
        }
        let line = input[pc as usize];
        let instruction = &line[0..3];
        match instruction {
            "hlf" => {
                let r = line.chars().nth(4).unwrap();
                registers.entry(r).and_modify(|x| *x /= 2).or_insert(0);
            }
            "tpl" => {
                let r = line.chars().nth(4).unwrap();
                registers.entry(r).and_modify(|x| *x *= 3).or_insert(0);
            }
            "inc" => {
                let r = line.chars().nth(4).unwrap();
                registers.entry(r).and_modify(|x| *x += 1).or_insert(1);
            }
            "jmp" => {
                let offset = line[4..]
                    .parse::<ItemType>()
                    .map_err(|_| PuzzleError("offset must be an integer".into()))?;
                pc += offset - 1;
            }
            "jie" => {
                let r = line.chars().nth(4).unwrap();
                let offset = line[7..]
                    .parse::<ItemType>()
                    .map_err(|_| PuzzleError("offset must be an integer".into()))?;
                if registers.get(&r).unwrap_or(&0) % 2 == 0 {
                    pc += offset - 1;
                }
            }
            "jio" => {
                let r = line.chars().nth(4).unwrap();
                let offset = line[7..]
                    .parse::<ItemType>()
                    .map_err(|_| PuzzleError("offset3 must be an integer".into()))?;
                if registers.get(&r) == Some(&1) {
                    pc += offset - 1;
                }
            }
            _ => {
                return Err(PuzzleError("invalid instruction".into()));
            }
        }
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
    fn invalid_instruction() {
        test_invalid(&[&"abc"], solve);
    }

    #[test]
    fn invalid_offset_must_be_integer() {
        test_invalid(&[&"jmp X"], solve);
    }
}
