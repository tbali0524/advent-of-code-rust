//! [aoc](https://adventofcode.com/2015/day/16)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 16,
        title: "Aunt Sue",
        solution: ("373", "260"),
        example_solutions: vec![],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    let aunt_spec = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);
    let expected_compare_result = HashMap::from([
        ("children", Ordering::Equal),
        ("cats", Ordering::Greater),
        ("samoyeds", Ordering::Equal),
        ("pomeranians", Ordering::Less),
        ("akitas", Ordering::Equal),
        ("vizslas", Ordering::Equal),
        ("goldfish", Ordering::Less),
        ("trees", Ordering::Greater),
        ("cars", Ordering::Equal),
        ("perfumes", Ordering::Equal),
    ]);
    let aunts = parse_input(input)?;
    // ---------- Part 1
    let mut ans1 = 0;
    for (&id, aunt) in &aunts {
        let mut is_ok = true;
        for (prop_name, &prop_value) in aunt {
            if *aunt_spec.get(prop_name.as_str()).unwrap_or(&-1) != prop_value {
                is_ok = false;
                break;
            }
        }
        if is_ok {
            ans1 = id;
            break;
        }
    }
    // ---------- Part 2
    let mut ans2 = 0;
    for (&id, aunt) in &aunts {
        let mut is_ok = true;
        for (prop_name, &prop_value) in aunt {
            if !aunt_spec.contains_key(prop_name.as_str()) {
                continue;
            }
            let comp = prop_value.cmp(aunt_spec.get(prop_name.as_str()).unwrap());
            if comp != *expected_compare_result.get(prop_name.as_str()).unwrap() {
                is_ok = false;
                break;
            }
        }
        if is_ok {
            ans2 = id;
            break;
        }
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

fn parse_input(
    input: PuzzleInput,
) -> Result<HashMap<ItemType, HashMap<String, ItemType>>, PuzzleError> {
    let mut aunts = HashMap::new();
    for line in input {
        let a = line.split(' ').collect::<Vec<_>>();
        if a.len() != 8 {
            Err("invalid number of words in input line")?;
        }
        let id = a[1][0..(a[1].len() - 1)]
            .parse::<ItemType>()
            .map_err(|_| "aunt it must be an integer")?;
        let mut aunt = HashMap::new();
        for i in 0..3 {
            let prop_name = &a[2 + 2 * i][0..(a[2 + 2 * i].len() - 1)];
            let prop_value = if i < 2 {
                a[3 + 2 * i][0..(a[3 + 2 * i].len() - 1)]
                    .parse::<ItemType>()
                    .map_err(|_| "property value must be an integer")?
            } else {
                a[7].parse::<ItemType>()
                    .map_err(|_| format!("property value must be an integer, found `{}", a[7]))?
            };
            aunt.insert(prop_name.to_owned(), prop_value);
        }
        aunts.insert(id, aunt);
    }
    Ok(aunts)
}

// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::runner::tests::*;

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_input_line() {
        test_invalid(&["a"], solve);
    }

    #[test]
    fn invalid_aunt_id_must_be_integer() {
        test_invalid(&["Sue X: cars: 9, akitas: 3, goldfish: 0"], solve);
    }

    #[test]
    fn invalid_property_value_must_be_integer() {
        test_invalid(&["Sue 1: cars: 9, akitas: X, goldfish: 0"], solve);
    }
}
