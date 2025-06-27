//! [aoc](https://adventofcode.com/2023/day/15)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 15,
        title: "Lens Library",
        solution: ("517015", "286104"),
        example_solutions: vec![("1320", "145")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() != 1 {
        Err("input must have a single line")?;
    }
    let data = input[0].split(',').collect::<Vec<_>>();
    // ---------- Part 1
    let ans1 = data.iter().map(|&x| hash(x)).sum::<usize>();
    // ---------- Part 2
    let mut ans2 = 0;
    let mut boxes = vec![HashMap::<String, (usize, usize)>::new(); 256];
    let mut idx = 0;
    for command in data {
        idx += 1;
        if let Some(label) = command.strip_suffix('-') {
            let id_box = hash(label);
            boxes[id_box].remove(label);
            continue;
        }
        let mut a_iter = command.split("=");
        let label = a_iter.next().unwrap();
        let value = a_iter
            .next()
            .ok_or("assign commands must contain `=`")?
            .parse::<usize>()
            .map_err(|_| "assign value must be integer")?;
        if a_iter.next().is_some() {
            Err("assign commands must contain single `=`")?;
        }
        let id_box = hash(label);
        if boxes[id_box].contains_key(label) {
            let old_idx = boxes[id_box].get(label).unwrap().1;
            boxes[id_box].insert(label.to_owned(), (value, old_idx));
        } else {
            boxes[id_box].insert(label.to_owned(), (value, idx));
        }
    }
    for (id_box, box_map) in boxes.iter().enumerate() {
        let mut box_list = box_map.values().collect::<Vec<_>>();
        box_list.sort_by_key(|&x| x.1);
        for (id_item, &box_tuple) in box_list.iter().enumerate() {
            ans2 += (id_box + 1) * (id_item + 1) * box_tuple.0;
        }
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, x| ((acc + x as usize) * 17) & 0xFF)
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
    fn invalid_single_line() {
        test_invalid_msg(&["rn=1,cm-", "a"], solve, "input must have a single line");
    }

    #[test]
    fn invalid_assign_must_contain_equal_sign() {
        test_invalid_msg(&["rn1,cm-"], solve, "assign commands must contain `=`");
    }

    #[test]
    fn invalid_assign_must_contain_single_equal_sign() {
        test_invalid_msg(
            &["rn=1=2,cm-"],
            solve,
            "assign commands must contain single `=`",
        );
    }

    #[test]
    fn invalid_assign_value_must_be_integer() {
        test_invalid_msg(&["rn=a,cm-"], solve, "assign value must be integer");
    }
}
