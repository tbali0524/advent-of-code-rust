//! [aoc](https://adventofcode.com/2015/day/7)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 7,
        title: "Some Assembly Required",
        solution: ("956", "40149"),
        example_solutions: vec![("72", "0")],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Check input
    // ---------- Part 1
    let mut circuit = Circuit::new(input)?;
    let ans1 = circuit.evaluate("a")?;
    // ---------- Part 2
    let mut circuit = Circuit::new(input)?;
    circuit
        .gates
        .insert("b".into(), Gate::try_from("956 -> b")?);
    let ans2 = circuit.evaluate("a")?;
    Ok((ans1.to_string(), ans2.to_string()))
}

#[derive(Default)]
struct Gate {
    id: String,
    operator: String,
    input1: Option<String>,
    input2: Option<String>,
    operand1: Option<ItemType>,
    operand2: Option<ItemType>,
    value: Option<ItemType>,
}

impl TryFrom<&str> for Gate {
    type Error = PuzzleError;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut g = Gate::default();
        let a = line.split(" -> ").map(String::from).collect::<Vec<_>>();
        if a.len() != 2 {
            return Err(PuzzleError("lines must contain ->".into()));
        }
        g.id = a[1].to_owned();
        let b = a[0].split(' ').map(String::from).collect::<Vec<_>>();
        match b.len() {
            1 => {
                g.operator = String::from("ASSIGN");
                match b[0].parse::<ItemType>() {
                    Ok(x) => g.operand1 = Some(x),
                    Err(_) => g.input1 = Some(b[0].to_owned()),
                }
            }
            2 => {
                if b[0] != "NOT" {
                    return Err(PuzzleError("invalid operator".into()));
                }
                g.operator = b[0].to_owned();
                match b[1].parse::<ItemType>() {
                    Ok(x) => g.operand1 = Some(x),
                    Err(_) => g.input1 = Some(b[1].to_owned()),
                }
            }
            3 => {
                g.operator = b[1].to_owned();
                match b[0].parse::<ItemType>() {
                    Ok(x) => g.operand1 = Some(x),
                    Err(_) => g.input1 = Some(b[0].to_owned()),
                }
                match b[2].parse::<ItemType>() {
                    Ok(x) => g.operand2 = Some(x),
                    Err(_) => g.input2 = Some(b[2].to_owned()),
                }
            }
            _ => {
                return Err(PuzzleError("too many operands".into()));
            }
        }
        Ok(g)
    }
}

#[derive(Default)]
struct Circuit {
    gates: HashMap<String, Gate>,
}

impl Circuit {
    fn new(input: PuzzleInput) -> Result<Self, PuzzleError> {
        let mut circuit = Self::default();
        for line in input {
            let gate = Gate::try_from(*line)?;
            circuit.gates.insert(gate.id.to_owned(), gate);
        }
        Ok(circuit)
    }

    fn evaluate(&mut self, id: &str) -> Result<ItemType, PuzzleError> {
        let gate = self
            .gates
            .get(id)
            .ok_or(PuzzleError("invalid wire id".into()))?;
        if let Some(x) = gate.value {
            return Ok(x);
        }
        let input1 = &gate.input1.to_owned();
        let a = match &input1 {
            Some(x) => self.evaluate(x)?,
            None => gate.operand1.unwrap(),
        };
        let gate = self.gates.get(id).unwrap();
        let input2 = &gate.input2.to_owned();
        let operator = gate.operator.to_owned();
        let value = match operator.as_str() {
            "ASSIGN" => a,
            "NOT" => !a,
            _ => {
                let b = match &input2 {
                    Some(x) => self.evaluate(x)?,
                    None => gate.operand2.unwrap(),
                };
                match operator.as_str() {
                    "AND" => a & b,
                    "OR" => a | b,
                    "LSHIFT" => a << b,
                    "RSHIFT" => a >> b,
                    _ => return Err(PuzzleError("invalid wire id".into())),
                }
            }
        };
        let gate = self.gates.get_mut(id).unwrap();
        gate.value = Some(value);
        Ok(value)
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
    fn invalid_must_have_arrow() {
        test_invalid(&[&"a"], solve);
    }

    #[test]
    fn invalid_too_many_operands() {
        test_invalid(&[&"x LSHIFT 2 3 -> y"], solve);
    }

    #[test]
    fn invalid_unary_operator() {
        test_invalid(&[&"AND x -> y"], solve);
    }

    #[test]
    fn invalid_binary_operator() {
        test_invalid(&[&"x XOR y -> z"], solve);
    }

    #[test]
    fn invalid_wire_id() {
        test_invalid(&[&"x AND 1 -> y"], solve);
    }
}
