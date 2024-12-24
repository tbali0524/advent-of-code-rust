//! [aoc](https://adventofcode.com/2024/day/24)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 24,
        title: "Crossed Wires",
        solution: ("51107420031718", "0"),
        example_solutions: vec![("4", "0"), ("2024", "0")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut gates = HashMap::new();
    let mut i = 0;
    while i < input.len() && !input[i].is_empty() {
        if input[i].len() != 6 || input[i].as_bytes()[3] != b':' || input[i].as_bytes()[4] != b' ' {
            Err("input definitions must be 3 letters, followed by `: ` and 0 or 1")?;
        }
        let name = input[i][..3].to_string();
        let mut gate = Gate::new(&name);
        gate.output = match input[i].as_bytes()[5] {
            b'0' => Some(0),
            b'1' => Some(1),
            _ => Err("input values must be 0 or 1")?,
        };
        gates.insert(name, gate);
        i += 1;
    }
    if i == input.len() {
        Err("input and gate definitions must be separated by an empty line")?;
    }
    i += 1;
    while i < input.len() {
        let words = input[i].split(' ').collect::<Vec<_>>();
        if words.len() != 5 {
            Err("gate definitions must be 5 words")?;
        }
        if words[3] != "->" {
            Err("gate definition 4th word must be `->`")?;
        }
        let name = words[4].to_string();
        let mut gate = Gate::new(&name);
        gate.inputs[0] = words[0].to_string();
        gate.inputs[1] = words[2].to_string();
        gate.operator = match words[1] {
            "AND" => Operator::OpAnd,
            "OR" => Operator::OpOr,
            "XOR" => Operator::OpXor,
            _ => Err("gate operator must be AND, OR, XOR")?,
        };
        if gates.contains_key(words[4]) {
            Err("duplicate gate definition")?;
        }
        gates.insert(name, gate);
        i += 1;
    }
    // ---------- Part 1
    let mut ans1 = 0u64;
    for i in 0..64 {
        let name = format!("z{:0>2}", i);
        if !gates.contains_key(&name) {
            break;
        }
        let bit = evaluate(&mut gates, &name)?;
        ans1 |= (bit as u64) << i;
    }
    // ---------- Part 2
    let ans2 = 0;
    Ok((ans1.to_string(), ans2.to_string()))
}

#[derive(Clone)]
enum Operator {
    OpAnd,
    OpOr,
    OpXor,
    Input,
}

#[allow(dead_code)]
struct Gate {
    name: String,
    operator: Operator,
    inputs: [String; 2],
    output: Option<u8>,
}

impl Gate {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            operator: Operator::Input,
            inputs: [String::new(), String::new()],
            output: None,
        }
    }
}

fn evaluate(gates: &mut HashMap<String, Gate>, name: &str) -> Result<u8, PuzzleError> {
    let gate = gates.get(name).ok_or("invalid gate name")?;
    if gate.output.is_some() {
        return Ok(gate.output.unwrap());
    }
    let operand1_name = gate.inputs[0].clone();
    let operand2_name = gate.inputs[1].clone();
    let operator = gate.operator.clone();
    let operand1 = evaluate(gates, &operand1_name)?;
    let operand2 = evaluate(gates, &operand2_name)?;
    let output = match operator {
        Operator::OpAnd => operand1 & operand2,
        Operator::OpOr => operand1 | operand2,
        Operator::OpXor => operand1 ^ operand2,
        _ => unreachable!(),
    };
    let gate = gates.get_mut(name).ok_or("invalid gate name")?;
    gate.output = Some(output);
    Ok(output)
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
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_input_must_be_6_chars_with() {
        test_invalid_msg(
            &[&"x01: 0a"],
            solve,
            "input definitions must be 3 letters, followed by `: ` and 0 or 1",
        );
    }

    #[test]
    fn invalid_input_must_have_colon() {
        test_invalid_msg(
            &[&"x01| 0"],
            solve,
            "input definitions must be 3 letters, followed by `: ` and 0 or 1",
        );
    }

    #[test]
    fn invalid_input_must_be_0_or_1() {
        test_invalid_msg(&[&"x01: 2"], solve, "input values must be 0 or 1");
    }

    #[test]
    fn invalid_missing_empty_separator() {
        test_invalid_msg(
            &[&"x01: 0"],
            solve,
            "input and gate definitions must be separated by an empty line",
        );
    }

    #[test]
    fn invalid_gate_must_be_5_words() {
        test_invalid_msg(
            &[&"x01: 0", &"", &"x00 AND y00 -> z00 a"],
            solve,
            "gate definitions must be 5 words",
        );
    }

    #[test]
    fn invalid_gate_must_have_arrow() {
        test_invalid_msg(
            &[&"x01: 0", &"", &"x00 AND y00 a z00"],
            solve,
            "gate definition 4th word must be `->`",
        );
    }

    #[test]
    fn invalid_gate_operator() {
        test_invalid_msg(
            &[&"x01: 0", &"", &"x00 a y00 -> z00"],
            solve,
            "gate operator must be AND, OR, XOR",
        );
    }

    #[test]
    fn invalid_duplicate_gate() {
        test_invalid_msg(
            &[&"x01: 0", &"", &"x00 AND y00 -> x01"],
            solve,
            "duplicate gate definition",
        );
    }

    #[test]
    fn invalid_gate_name() {
        test_invalid_msg(
            &[&"x01: 0", &"", &"x00 AND y00 -> z00"],
            solve,
            "invalid gate name",
        );
    }
}
