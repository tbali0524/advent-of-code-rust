//! [aoc](https://adventofcode.com/2015/day/12)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use serde_json::Value;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 12,
        title: "JSAbacusFramework.io",
        solution: ("111754", "65402"),
        example_solutions: vec![("18", "0"), ("0", "16")],
    }
}

type ItemType = i64;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() != 1 {
        return Err(PuzzleError("input must have a single line".into()));
    }
    let data: Value = serde_json::from_str(input[0])
        .map_err(|_| PuzzleError("input must be in valid json format".into()))?;
    // ---------- Part 1 + 2
    let ans1 = json_sum_ints(&data, 0);
    let ans2 = json_sum_reds(&data, 0);
    Ok((ans1.to_string(), ans2.to_string()))
}

fn json_sum_ints(v: &Value, sum: ItemType) -> ItemType {
    match v {
        Value::Null => sum,
        Value::Bool(_) => sum,
        Value::Number(x) => sum + x.as_i64().unwrap_or_default(),
        Value::String(_) => sum,
        Value::Array(a) => sum + a.iter().map(|x| json_sum_ints(x, 0)).sum::<ItemType>(),
        Value::Object(m) => sum + m.values().map(|x| json_sum_ints(x, 0)).sum::<ItemType>(),
    }
}

fn json_sum_reds(v: &Value, sum: ItemType) -> ItemType {
    match v {
        Value::Null => sum,
        Value::Bool(_) => sum,
        Value::Number(x) => sum + x.as_i64().unwrap_or_default(),
        Value::String(_) => sum,
        Value::Array(a) => sum + a.iter().map(|x| json_sum_reds(x, 0)).sum::<ItemType>(),
        Value::Object(m) => {
            if m.values().any(|x| x == "red") {
                0
            } else {
                sum + m.values().map(|x| json_sum_reds(x, 0)).sum::<ItemType>()
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
    fn example2() {
        test_case(metadata, solve, 2);
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
    fn invalid_must_be_valid_json() {
        test_invalid(&[&"[1,"], solve);
    }
}
