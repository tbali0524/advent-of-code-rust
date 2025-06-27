//! [aoc](https://adventofcode.com/2024/day/5)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 5,
        title: "Print Queue",
        solution: ("5639", "5273"),
        example_solutions: vec![("143", "123")],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut rules = HashMap::new();
    let mut pagelists = Vec::new();
    let mut i = 0;
    while i < input.len() && !input[i].is_empty() {
        let mut row_iter = input[i].split('|');
        let left = row_iter
            .next()
            .unwrap()
            .parse::<ItemType>()
            .map_err(|_| "rules must contain only integers")?;
        let right = row_iter
            .next()
            .ok_or("rules must contain two items separated by `|`, one found")?
            .parse::<ItemType>()
            .map_err(|_| "rules must contain only integers")?;
        if row_iter.next().is_some() {
            Err("rules must contain two items separated by `|`, more found")?;
        }
        rules.insert((left, right), true);
        rules.insert((right, left), false);
        i += 1;
    }
    if i == input.len() {
        Err("missing page list, must come after after rules and an empty line")?;
    }
    i += 1;
    while i < input.len() {
        pagelists.push(
            input[i]
                .split(',')
                .map(|x| {
                    x.parse::<ItemType>()
                        .map_err(|_| format!("page list must contain only integers, found `{x}`"))
                })
                .collect::<Result<Vec<_>, _>>()?,
        );
        i += 1;
    }
    // ---------- Part 1
    let mut ans1 = 0;
    let mut incorrects = Vec::new();
    for (idx_row, pagelist) in pagelists.iter().enumerate() {
        let mut is_ok = true;
        'row: for (idx, &page) in pagelist.iter().enumerate() {
            for &prev in pagelist[0..idx].iter() {
                if !rules.get(&(prev, page)).unwrap_or(&true) {
                    is_ok = false;
                    break 'row;
                }
            }
        }
        if is_ok {
            ans1 += pagelist[pagelist.len() / 2];
        } else {
            incorrects.push(idx_row);
        }
    }
    // ---------- Part 2
    let mut ans2 = 0;
    for &idx_row in incorrects.iter() {
        let pagelist = &pagelists[idx_row];
        let mut count_prevs = Vec::new();
        for (idx, &page) in pagelist.iter().enumerate() {
            count_prevs.push((page, 0));
            for &prev in pagelist.iter() {
                if *rules.get(&(prev, page)).unwrap_or(&false) {
                    count_prevs[idx].1 += 1;
                }
            }
        }
        count_prevs.sort_by_key(|&x| x.1);
        ans2 += count_prevs[count_prevs.len() / 2].0;
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
    fn invalid_rule_must_contain_integer() {
        test_invalid_msg(
            &["1|a", "", "1,2"],
            solve,
            "rules must contain only integers",
        );
    }

    #[test]
    fn invalid_rule_must_contain_2_found_1() {
        test_invalid_msg(
            &["1", "", "1,2"],
            solve,
            "rules must contain two items separated by `|`, one found",
        );
    }

    #[test]
    fn invalid_rule_must_contain_2_found_more() {
        test_invalid_msg(
            &["1|2|3", "", "1,2"],
            solve,
            "rules must contain two items separated by `|`, more found",
        );
    }

    #[test]
    fn invalid_pagelist_missing() {
        test_invalid_msg(
            &["1|2"],
            solve,
            "missing page list, must come after after rules and an empty line",
        );
    }

    #[test]
    fn invalid_pagelist_must_contain_integer() {
        test_invalid_msg(
            &["1|2", "", "1,a"],
            solve,
            "page list must contain only integers",
        );
    }
}
