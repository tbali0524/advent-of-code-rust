//! [aoc](https://adventofcode.com/2025/day/5)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2025,
        day: 5,
        title: "Cafeteria",
        solution: ("607", "342433357244012"),
        example_solutions: vec![("3", "14")],
    }
}

type ItemType = u64;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    let mut i = 0;
    while !input[i].is_empty() {
        let mut range_iter = input[i].split('-');
        let from = range_iter
            .next()
            .ok_or("invalid input")?
            .parse::<ItemType>()
            .map_err(|_| "range `from` must be an integer")?;
        let to = range_iter
            .next()
            .ok_or("missing `to` in range")?
            .parse::<ItemType>()
            .map_err(|_| "range `to` must be an integer")?;
        if range_iter.next().is_some() {
            Err("range must have only two parts")?;
        }
        ranges.push((from, to));
        i += 1;
    }
    i += 1;
    while i < input.len() {
        let id = input[i]
            .parse::<ItemType>()
            .map_err(|_| "`id` must be an integer")?;
        ids.push(id);
        i += 1;
    }
    // ---------- Part 1
    let mut ans1 = 0;
    for &id in &ids {
        for &(from, to) in &ranges {
            if (from..=to).contains(&id) {
                ans1 += 1;
                break;
            }
        }
    }
    // ---------- Part 3
    let mut ans2 = 0;
    let mut froms = Vec::new();
    for &(from, to) in &ranges {
        froms.push(from);
        froms.push(to + 1);
    }
    froms.sort();
    froms.dedup();
    let count = froms.len().saturating_sub(1);
    let mut fresh = vec![false; count];
    for &(from, to) in &ranges {
        for i in 0..count {
            if from <= froms[i] && froms[i + 1] - 1 <= to {
                fresh[i] = true;
            }
        }
    }
    for i in 0..count {
        if fresh[i] {
            ans2 += froms[i + 1] - froms[i];
        }
    }
    Ok((ans1.to_string(), ans2.to_string()))
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
    fn invalid_from_must_be_integer() {
        test_invalid_msg(
            &[&"a-2", &"", &"1"],
            solve,
            "range `from` must be an integer",
        );
    }

    #[test]
    fn invalid_to_must_be_integer() {
        test_invalid_msg(&[&"1-a", &"", &"1"], solve, "range `to` must be an integer");
    }

    #[test]
    fn invalid_range_must_have_2_parts() {
        test_invalid_msg(
            &[&"1-2-3", &"", &"1"],
            solve,
            "range must have only two parts",
        );
    }

    #[test]
    fn invalid_id_must_be_integer() {
        test_invalid_msg(&[&"1-2", &"", &"a"], solve, "`id` must be an integer");
    }
}
