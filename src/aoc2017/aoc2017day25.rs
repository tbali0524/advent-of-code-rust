//! [aoc](https://adventofcode.com/2017/day/25)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashMap;

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2017,
    day: 25,
    title: "The Halting Problem",
    solution: (2832, 0),
    example_solutions: [(3, 0), (0, 0)],
    string_solution: None,
    example_string_solutions: None,
    example_string_inputs: None,
};

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Check input
    if input.len() < 2 {
        return Err("Invalid input");
    }
    let count_states = (input.len() - 2) / 10;
    if count_states < 2
        || input.len() != count_states * 10 + 2
        || !input[0].starts_with("Begin in state ")
        || !input[1].starts_with("Perform a diagnostic checksum after ")
        || input[0].len() != 17
        || input[1].len() < 42
    {
        return Err("Invalid input");
    }
    let start_state = input[0].as_bytes()[15] as char;
    let max_steps = input[1][36..]
        .split(" step")
        .next()
        .unwrap()
        .parse::<ItemType>()
        .map_err(|_| "Input must contain only integers")?;
    let mut states = HashMap::new();
    for i in 0..count_states {
        if !input[10 * i + 2].is_empty()
            || !input[10 * i + 3].starts_with("In state ")
            || input[10 * i + 3].len() != 11
            || input[10 * i + 4] != "  If the current value is 0:"
            || !input[10 * i + 5].starts_with("    - Write the value ")
            || input[10 * i + 5].len() != 24
            || !input[10 * i + 6].starts_with("    - Move one slot to the ")
            || input[10 * i + 6].len() < 32
            || !input[10 * i + 7].starts_with("    - Continue with state ")
            || input[10 * i + 7].len() != 28
            || input[10 * i + 8] != "  If the current value is 1:"
            || !input[10 * i + 9].starts_with("    - Write the value ")
            || input[10 * i + 9].len() != 24
            || !input[10 * i + 10].starts_with("    - Move one slot to the ")
            || input[10 * i + 10].len() < 32
            || !input[10 * i + 11].starts_with("    - Continue with state ")
            || input[10 * i + 11].len() != 28
        {
            return Err("Invalid input");
        }
        let state = input[10 * i + 3].as_bytes()[9] as char;
        states.insert(
            state,
            [
                input[10 * i + 5].as_bytes()[22] as char,
                input[10 * i + 6].as_bytes()[27] as char,
                input[10 * i + 7].as_bytes()[26] as char,
                input[10 * i + 9].as_bytes()[22] as char,
                input[10 * i + 10].as_bytes()[27] as char,
                input[10 * i + 11].as_bytes()[26] as char,
            ],
        );
    }
    // ---------- Part 1
    let mut tape = HashMap::new();
    let mut cursor = 0;
    let mut state = start_state;
    let slot_lookup = HashMap::from([('0', false), ('1', true)]);
    let cursor_lookup = HashMap::from([('l', -1), ('r', 1)]);
    for _ in 0..max_steps {
        let slot = *tape.get(&cursor).unwrap_or(&0);
        let todo = states.get(&state).unwrap();
        let new_slot = *slot_lookup.get(&todo[3 * slot]).ok_or("Invalid input")?;
        if new_slot {
            tape.insert(cursor, 1);
        } else {
            tape.remove(&cursor);
        }
        cursor += cursor_lookup
            .get(&todo[3 * slot + 1])
            .ok_or("Invalid input")?;
        state = todo[3 * slot + 2];
    }
    let ans1 = tape.len();
    Ok((ans1.to_string(), "0".to_string()))
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
    fn invalid_input() {
        test_invalid(&PUZZLE_METADATA, &[String::from("a")], solve);
    }
}
