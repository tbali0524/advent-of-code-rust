//! [aoc](https://adventofcode.com/2023/day/12)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 12,
        title: "Hot Springs",
        solution: ("8193", "45322533163795"),
        example_solutions: vec![("21", "525152")],
    }
}

type ItemType = i64;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut records = Vec::new();
    let mut sizes = Vec::new();
    for line in input {
        let mut a_iter = line.split(' ');
        let record = a_iter.next().unwrap();
        let size_list = a_iter
            .next()
            .ok_or("missing size list")?
            .split(',')
            .map(|x| {
                x.parse::<ItemType>()
                    .map_err(|_| format!("sizes must be integers, found `{x}`").into())
            })
            .collect::<Result<Vec<_>, PuzzleError>>()?;
        if a_iter.next().is_some() {
            Err("lines must contain only record and size list")?;
        }
        records.push(record);
        sizes.push(size_list);
    }
    // ---------- Part 1
    let mut ans1 = 0;
    let mut memo = HashMap::new();
    for i in 0..records.len() {
        ans1 += count_solutions(records[i], &sizes[i], &mut memo);
    }
    // ---------- Part 2
    let mut ans2 = 0;
    let mut memo = HashMap::new();
    for i in 0..records.len() {
        let record = records[i].to_owned()
            + "?"
            + records[i]
            + "?"
            + records[i]
            + "?"
            + records[i]
            + "?"
            + records[i];
        let size = sizes[i].repeat(5);
        ans2 += count_solutions(&record, &size, &mut memo);
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

fn count_solutions(
    pattern: &str,
    sizes: &[ItemType],
    memo: &mut HashMap<String, ItemType>,
) -> ItemType {
    if pattern.is_empty() {
        return if sizes.is_empty() { 1 } else { 0 };
    }
    let hash = pattern.to_owned()
        + " "
        + &sizes
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");
    if memo.contains_key(&hash) {
        return *memo.get(&hash).unwrap();
    }
    let mut result = 0;
    let first_char = pattern.chars().next().unwrap();
    if first_char == '.' {
        if pattern.len() > 1 {
            result = count_solutions(&pattern[1..], sizes, memo);
        } else {
            result = count_solutions("", sizes, memo);
        }
    } else if first_char == '?' {
        if pattern.len() > 1 {
            result = count_solutions(&(".".to_string() + &pattern[1..]), sizes, memo)
                + count_solutions(&("#".to_string() + &pattern[1..]), sizes, memo);
        } else {
            result = count_solutions(".", sizes, memo) + count_solutions("#", sizes, memo);
        }
    } else if sizes.is_empty() || pattern.len() < sizes[0] as usize {
        result = 0;
    } else {
        let mut is_ok = true;
        for i in 0..sizes[0] as usize {
            if pattern.as_bytes()[i] == b'.' {
                is_ok = false;
                break;
            }
        }
        if (sizes[0] as usize) < pattern.len() && pattern.as_bytes()[sizes[0] as usize] == b'#' {
            is_ok = false;
        }
        if is_ok {
            if sizes[0] as usize + 1 < pattern.len() {
                result = count_solutions(&pattern[sizes[0] as usize + 1..], &sizes[1..], memo);
            } else {
                result = count_solutions("", &sizes[1..], memo);
            }
        }
    }
    memo.insert(hash, result);
    result
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
    fn invalid_missing_size_list() {
        test_invalid_msg(&["???.###"], solve, "missing size list");
    }

    #[test]
    fn invalid_sizes_must_be_integers() {
        test_invalid_msg(&["???.### 1,a,3"], solve, "sizes must be integers");
    }

    #[test]
    fn invalid_only_record_and_sizes() {
        test_invalid_msg(
            &["???.### 1,1,3 x"],
            solve,
            "lines must contain only record and size list",
        );
    }
}
