//! [aoc](https://adventofcode.com/2017/day/23)

use crate::aoc::PuzzleMetaData;
use crate::aoc::PuzzleResult;
use std::collections::HashMap;

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2017,
    day: 23,
    title: "Coprocessor Conflagration",
    solution: (8281, 911),
    example_solutions: [(0, 0), (0, 0)],
    string_solution: None,
    example_string_solutions: None,
    example_string_inputs: None,
};

type ItemType = i64;

struct CoProcessor {
    instructions: Vec<String>,
    total_muls: ItemType,
    registers: HashMap<char, ItemType>,
    pc: ItemType,
}

impl CoProcessor {
    fn new(input: &[String]) -> Self {
        CoProcessor {
            instructions: input.to_owned(),
            total_muls: 0,
            registers: HashMap::new(),
            pc: -1,
        }
    }

    fn execute(&mut self) -> Result<(), &'static str> {
        loop {
            self.pc += 1;
            if self.pc < 0 || self.pc >= self.instructions.len() as ItemType {
                return Ok(());
            }
            let line = &self.instructions[self.pc as usize];
            if line.len() < 7
                || line.as_bytes()[3] as char != ' '
                || line.as_bytes()[5] as char != ' '
            {
                return Err("Invalid input");
            }
            let instruction = &line[0..3];
            let x_reg = line.as_bytes()[4] as char;
            let x_value = if x_reg.is_ascii_lowercase() {
                *self.registers.get(&x_reg).unwrap_or(&0)
            } else if x_reg.is_ascii_digit() {
                x_reg.to_digit(10).unwrap() as ItemType
            } else {
                return Err("Invalid first argument in input");
            };
            let y_reg = line.as_bytes()[6] as char;
            let y_value = if y_reg.is_ascii_lowercase() {
                *self.registers.get(&y_reg).unwrap_or(&0)
            } else {
                line[6..]
                    .parse::<ItemType>()
                    .map_err(|_| "Invalid second argument in input")?
            };
            match instruction {
                "set" => {
                    self.registers.insert(x_reg, y_value);
                }
                "sub" => {
                    self.registers.insert(x_reg, x_value - y_value);
                }
                "mul" => {
                    self.registers.insert(x_reg, x_value * y_value);
                    self.total_muls += 1;
                }
                "mod" => {
                    self.registers.insert(x_reg, x_value % y_value);
                }
                "jnz" => {
                    if x_value != 0 {
                        self.pc += y_value - 1;
                    }
                }
                _ => {
                    return Err("Invalid instruction");
                }
            }
        }
    }
}

fn is_prime(n: ItemType) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    if n <= 1 || n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

pub fn solve(input: &[String]) -> PuzzleResult {
    // ---------- Part 1
    let mut proc = CoProcessor::new(input);
    proc.execute()?;
    let ans1 = proc.total_muls;
    // ---------- Part 2
    if input.len() != 32 {
        return Ok((ans1.to_string(), "0".to_string()));
    }
    let mut ans2 = 0;
    let start_b = input[0][6..]
        .parse::<ItemType>()
        .map_err(|_| "Invalid input")?;
    let mul_b = input[4][6..]
        .parse::<ItemType>()
        .map_err(|_| "Invalid input")?;
    let sub_b = input[5][6..]
        .parse::<ItemType>()
        .map_err(|_| "Invalid input")?;
    let sub_c = input[7][6..]
        .parse::<ItemType>()
        .map_err(|_| "Invalid input")?;
    let step = -input[30][6..]
        .parse::<ItemType>()
        .map_err(|_| "Invalid input")?;
    let from = start_b * mul_b - sub_b;
    let to = from - sub_c;
    for n in (from..=to).step_by(step as usize) {
        if !is_prime(n) {
            ans2 += 1;
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
    fn puzzle() {
        test_case(&PUZZLE_METADATA, 0, solve);
    }

    #[test]
    fn invalid_short_line() {
        test_invalid(&PUZZLE_METADATA, &[String::from("a")], solve);
    }

    #[test]
    fn invalid_instruction_too_long() {
        test_invalid(&PUZZLE_METADATA, &[String::from("setaa 1")], solve);
    }

    #[test]
    fn invalid_istruction() {
        test_invalid(&PUZZLE_METADATA, &[String::from("abc a 1")], solve);
    }

    #[test]
    fn invalid_first_argument() {
        test_invalid(&PUZZLE_METADATA, &[String::from("set - 1")], solve);
    }

    #[test]
    fn invalid_second_argument() {
        test_invalid(&PUZZLE_METADATA, &[String::from("set a *")], solve);
    }
}
