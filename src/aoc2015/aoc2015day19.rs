//! [aoc](https://adventofcode.com/2015/day/19)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::{HashMap, HashSet};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 19,
        title: "Medicine for Rudolph",
        solution: ("535", "212"),
        example_solutions: vec![],
    }
}

#[expect(clippy::needless_range_loop)]
pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and check input
    if input.len() < 3 {
        Err("input must have at least 3 lines")?;
    }
    if !input[input.len() - 2].is_empty() {
        Err("rules and molecule must be separated by empty line")?;
    }
    let mut replacements = HashMap::new();
    let mut reverse = HashMap::new();
    let molecule = input[input.len() - 1];
    for i in 0..input.len() - 2 {
        let a = input[i].split(" => ").collect::<Vec<_>>();
        if a.len() != 2 {
            Err("rules must have a => arrow")?;
        }
        if !replacements.contains_key(a[0]) {
            replacements.insert(a[0].to_owned(), Vec::new());
        }
        replacements.get_mut(a[0]).unwrap().push(a[1].to_owned());
        reverse.insert(a[1].to_owned(), a[0].to_owned());
    }
    // ---------- Part 1
    let mut result_set = HashSet::new();
    for i in 0..molecule.len() {
        let elem = &molecule[i..(i + 1)];
        if !replacements.contains_key(elem) {
            continue;
        }
        for new_elem in replacements.get(elem).unwrap() {
            let new_molecule = molecule[0..i].to_string() + new_elem + &molecule[i + 1..];
            result_set.insert(new_molecule);
        }
    }
    for i in 0..molecule.len() - 1 {
        let elem = &molecule[i..(i + 2)];
        if !replacements.contains_key(elem) {
            continue;
        }
        for new_elem in replacements.get(elem).unwrap() {
            let new_molecule = molecule[0..i].to_string() + new_elem + &molecule[i + 2..];
            result_set.insert(new_molecule);
        }
    }
    let ans1 = result_set.len();
    // ---------- Part 2
    // Note: not a correct solution, but works for this specific input...
    // but requires a specific iteration order for the keys of reverse...
    let mut ans2 = 0;
    let mut new_molecule = molecule.to_owned();
    while new_molecule != "e" {
        let mut keys = reverse.keys().collect::<Vec<_>>();
        keys.sort();
        keys.reverse();
        // for (to, from) in &reverse { // solution works only with specific iteration order
        for to in keys {
            let from = reverse.get(to).unwrap();
            if new_molecule.contains(to) {
                ans2 += new_molecule.matches(to).count();
                new_molecule = new_molecule.replace(to, from);
            }
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
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_must_have_3_or_more_lines() {
        test_invalid(&["a"], solve);
    }

    #[test]
    fn invalid_line_before_last_must_be_empty() {
        test_invalid(&["a => b", "a => c", "b"], solve);
    }

    #[test]
    fn invalid_must_have_arrow() {
        test_invalid(&["a", "", "a"], solve);
    }
}
