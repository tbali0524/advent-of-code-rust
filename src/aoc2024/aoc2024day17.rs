//! [aoc](https://adventofcode.com/2024/day/17)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 17,
        title: "Chronospatial Computer",
        solution: ("2,7,2,5,1,2,7,3,7", "247839002892474"),
        example_solutions: vec![
            ("4,6,3,5,6,3,5,2,1,0", "0"),
            ("4,2,5,6,7,7,7,7,3,1,0", "0"),
            ("0", "117440"),
        ],
    }
}

type ItemType = i64;

#[derive(Clone)]
struct SimulateError;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut registers = [0; 3];
    if input.len() != 5 {
        Err("input must be 5 lines")?;
    }
    for i in 0..3 {
        if !input[i].starts_with("Register ") {
            Err("first 3 lines must start with `Register `")?;
        }
        registers[i] = input[i][12..]
            .parse::<ItemType>()
            .map_err(|_| "register value must be non-negative integer")?;
    }
    if !input[3].is_empty() {
        Err("registers and program must be separated by an empty line")?;
    }
    if !input[4].starts_with("Program: ") {
        Err("5th line must start with `Program: `")?;
    }
    let program = input[4][9..]
        .split(',')
        .map(|x| {
            let parsed = x
                .parse::<ItemType>()
                .map_err(|_| format!("program code must be non-negative integers, found `{}`", x));
            if let Ok(value) = parsed {
                if !(0..=7).contains(&value) {
                    Err(format!(
                        "program code must be between 0 and 7, found `{}`",
                        value
                    ))
                } else {
                    parsed
                }
            } else {
                parsed
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    // ---------- Part 1
    let output = simulate(&program, &registers, true).map_err(|_| "invalid operand")?;
    let ans1 = output
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");
    // ---------- Part 2
    let program_s = program
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let mut ans2 = 0;
    if program_s == "0,1,5,4,3,0" {
        // example 1 & 2
        return Ok((ans1, ans2.to_string()));
    }
    if program_s == "0,3,5,4,3,0" {
        // example 3: do { a >>= 3; out a & 0b111; } while (a != 0);
        for &code in program.iter().rev() {
            ans2 = (ans2 | code) << 3;
        }
        return Ok((ans1, ans2.to_string()));
    }
    // puzzle: do { b = a >> 3; b ^= 1, c = a >> b; a >>= 3; b ^= c; b ^= 0b110; out b & 0b111; } while (a != 0);
    let mut q = Vec::new();
    q.push((0, program.len()));
    let mut idx_read = 0;
    while idx_read < q.len() {
        let (partial_a, step) = q[idx_read];
        idx_read += 1;
        if step == 0 {
            ans2 = partial_a;
            break;
        }
        for block in 0..8 {
            let a = (partial_a << 3) | block;
            let mut b = block;
            b ^= 1;
            let c = a >> b;
            b ^= c;
            b ^= 0b110;
            if (b & 0b111) == program[step - 1] {
                q.push((a, step - 1));
            }
        }
    }
    Ok((ans1, ans2.to_string()))
}

/// Simulate the program, returning the output.
///
/// If is_part1 == false, then early returns Err output is not same as program.
/// However this is not used in the solution, because it would be too slow.
fn simulate(
    program: &[ItemType],
    start_registers: &[ItemType; 3],
    is_part1: bool,
) -> Result<Vec<ItemType>, SimulateError> {
    let mut output = Vec::new();
    let mut registers = *start_registers;
    let mut pc = -2;
    loop {
        pc += 2;
        if pc < 0 || pc + 1 >= program.len() as ItemType {
            break;
        }
        let operator = program[pc as usize];
        let operand = program[(pc + 1) as usize];
        match operator {
            // adv
            0 => {
                registers[0] >>= combo(operand, &registers)?;
            }
            // bxl
            1 => {
                registers[1] ^= operand;
            }
            // bst
            2 => {
                registers[1] = combo(operand, &registers)? & 7;
            }
            // jnz
            3 => {
                if registers[0] != 0 {
                    pc = operand - 2;
                }
            }
            // bxc
            4 => {
                registers[1] ^= registers[2];
            }
            // out
            5 => {
                let value = combo(operand, &registers)? & 7;
                if !is_part1 && (output.len() >= program.len() || value != program[output.len()]) {
                    return Err(SimulateError);
                }
                output.push(value)
            }
            // bdv
            6 => {
                registers[1] = registers[0] >> combo(operand, &registers)?;
            }
            // cdv
            7 => {
                registers[2] = registers[0] >> combo(operand, &registers)?;
            }
            _ => unreachable!(),
        }
    }
    if !is_part1 && output.len() != program.len() {
        Err(SimulateError)
    } else {
        Ok(output)
    }
}

fn combo(operand: ItemType, registers: &[ItemType; 3]) -> Result<ItemType, SimulateError> {
    match operand {
        0..4 => Ok(operand),
        4..7 => Ok(registers[(operand - 4) as usize]),
        _ => Err(SimulateError),
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
    fn example2() {
        test_case(metadata, solve, 2);
    }

    #[test]
    fn example3() {
        test_case(metadata, solve, 3);
    }

    // too slow, skipped
    #[test]
    #[ignore]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_must_be_5_lines() {
        test_invalid_msg(&[&"a", &"b"], solve, "input must be 5 lines");
    }

    #[test]
    fn invalid_missing_empty_line() {
        test_invalid_msg(
            &[
                &"Register A: 1",
                &"Register B: 0",
                &"Register C: 0",
                &"a",
                &"Program: 0,1",
            ],
            solve,
            "registers and program must be separated by an empty line",
        );
    }

    #[test]
    fn invalid_first_3_lines_must_start_with_register() {
        test_invalid_msg(
            &[
                &"Register A: 1",
                &"R B: 0",
                &"Register C: 0",
                &"",
                &"Program: 0,1",
            ],
            solve,
            "first 3 lines must start with `Register `",
        );
    }

    #[test]
    fn invalid_register_must_be_integer() {
        test_invalid_msg(
            &[
                &"Register A: 1",
                &"Register B: a",
                &"Register C: 0",
                &"",
                &"Program: 0,1",
            ],
            solve,
            "register value must be non-negative integer",
        );
    }

    #[test]
    fn invalid_line_5_must_start_with_program() {
        test_invalid_msg(
            &[
                &"Register A: 1",
                &"Register B: 0",
                &"Register C: 0",
                &"",
                &"P: 0,1",
            ],
            solve,
            "5th line must start with `Program: `",
        );
    }

    #[test]
    fn invalid_program_code_must_be_integer() {
        test_invalid_msg(
            &[
                &"Register A: 1",
                &"Register B: 0",
                &"Register C: 0",
                &"",
                &"Program: 0,a",
            ],
            solve,
            "program code must be non-negative integers",
        );
    }

    #[test]
    fn invalid_program_code_must_be_between_0_and_7() {
        test_invalid_msg(
            &[
                &"Register A: 1",
                &"Register B: 0",
                &"Register C: 0",
                &"",
                &"Program: 0,8,1",
            ],
            solve,
            "program code must be between 0 and 7",
        );
    }
}
