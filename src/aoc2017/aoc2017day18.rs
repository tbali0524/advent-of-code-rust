//! [aoc](https://adventofcode.com/2017/day/18)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashMap;
use std::collections::VecDeque;

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2017,
    day: 18,
    title: "Duet",
    solution: (9423, 7620),
    example_solutions: [(4, 0), (0, 3)],
    string_solution: None,
    example_string_solutions: None,
    example_string_inputs: None,
};

type ItemType = i64;

#[derive(Default)]
struct Thread {
    is_part1: bool,
    _id: ItemType,
    instructions: Vec<String>,
    snd_queue: VecDeque<ItemType>,
    rcv_queue: VecDeque<ItemType>,
    wait_to_receive: bool,
    completed: bool,
    sound: ItemType,
    total_sent: usize,
    registers: HashMap<char, ItemType>,
    pc: ItemType,
}

impl Thread {
    fn new(input: PuzzleInput, id: ItemType, is_part1: bool) -> Self {
        let mut p = Thread {
            _id: id,
            is_part1,
            instructions: input.to_owned(),
            sound: -1,
            pc: -1,
            ..Default::default()
        };
        p.registers.insert('p', id);
        p
    }

    fn execute(&mut self) -> Result<(), &'static str> {
        'outer: loop {
            if !self.wait_to_receive {
                self.pc += 1;
            } else if self.rcv_queue.is_empty() {
                return Ok(());
            }
            if self.pc < 0 || self.pc >= self.instructions.len() as ItemType {
                self.wait_to_receive = false;
                self.completed = true;
                return Ok(());
            }
            let line = &self.instructions[self.pc as usize];
            if line.len() < 5 || line.as_bytes()[3] as char != ' ' {
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
            match instruction {
                "snd" => {
                    if self.is_part1 {
                        self.sound = x_value;
                        continue 'outer;
                    }
                    self.total_sent += 1;
                    self.snd_queue.push_back(x_value);
                    continue 'outer;
                }
                "rcv" => {
                    if self.is_part1 {
                        if x_value != 0 {
                            self.completed = true;
                            return Ok(());
                        }
                        continue 'outer;
                    }
                    if self.rcv_queue.is_empty() {
                        self.wait_to_receive = true;
                        return Ok(());
                    }
                    self.wait_to_receive = false;
                    let y_value = self.rcv_queue.pop_front().unwrap();
                    self.registers.insert(x_reg, y_value);
                    continue 'outer;
                }
                _ => (),
            }
            if line.len() < 7 || line.as_bytes()[5] as char != ' ' {
                return Err("Invalid input");
            }
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
                "add" => {
                    self.registers.insert(x_reg, x_value + y_value);
                }
                "mul" => {
                    self.registers.insert(x_reg, x_value * y_value);
                }
                "mod" => {
                    self.registers.insert(x_reg, x_value % y_value);
                }
                "jgz" => {
                    if x_value > 0 {
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

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Part 1
    let mut thread = Thread::new(input, 0, true);
    thread.execute()?;
    let ans1 = thread.sound;
    // ---------- Part 2
    let mut thread0 = Thread::new(input, 0, false);
    let mut thread1 = Thread::new(input, 1, false);
    loop {
        let mut was_comm = false;
        thread0.execute()?;
        while !thread0.snd_queue.is_empty() {
            thread1
                .rcv_queue
                .push_back(thread0.snd_queue.pop_front().unwrap());
            was_comm = true;
        }
        thread1.execute()?;
        while !thread1.snd_queue.is_empty() {
            thread0
                .rcv_queue
                .push_back(thread1.snd_queue.pop_front().unwrap());
            was_comm = true;
        }
        if thread0.completed && thread1.completed {
            break;
        }
        if !was_comm {
            break;
        }
    }
    let ans2 = thread1.total_sent;
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
    fn example2() {
        test_case(&PUZZLE_METADATA, 2, solve);
    }

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
        test_invalid(&PUZZLE_METADATA, &[String::from("sndda 1")], solve);
    }

    #[test]
    fn invalid_istruction() {
        test_invalid(&PUZZLE_METADATA, &[String::from("abc a 1")], solve);
    }

    #[test]
    fn invalid_first_argument() {
        test_invalid(&PUZZLE_METADATA, &[String::from("add - 1")], solve);
    }

    #[test]
    fn invalid_second_argument() {
        test_invalid(&PUZZLE_METADATA, &[String::from("add a *")], solve);
    }
}
