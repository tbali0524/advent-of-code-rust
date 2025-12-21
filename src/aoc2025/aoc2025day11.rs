//! [aoc](https://adventofcode.com/2025/day/11)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2025,
        day: 11,
        title: "Reactor",
        solution: ("428", "331468292364745"),
        example_solutions: vec![("5", "0"), ("0", "2")],
    }
}

type ItemType = u64;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut adj_list = HashMap::new();
    let mut has_out = false;
    for &line in input.iter() {
        let item = line.split_ascii_whitespace().next().unwrap();
        if !item.ends_with(':') {
            Err("first word in input lines must end with `:`")?;
        }
        let from = item[..(item.len() - 1)].to_string();
        let mut to = Vec::new();
        for item in line.split_ascii_whitespace().skip(1) {
            if item == "out" {
                has_out = true;
            }
            to.push(item.to_string());
        }
        adj_list.insert(from, to);
    }
    if !has_out {
        Err("input must contain rule with endpoint `out`")?;
    }
    // ---------- Part 1
    let ans1 = if adj_list.contains_key("you") {
        let mut memo = HashMap::new();
        count_paths_part1("you", &adj_list, &mut memo)
    } else {
        0
    };
    // ---------- Part 2
    let mut memo = HashMap::new();
    let mut memo_d = HashMap::new();
    let mut memo_f = HashMap::new();
    let mut memo_df = HashMap::new();
    let ans2 = if adj_list.contains_key("svr") {
        count_paths_part2(
            "svr",
            true,
            true,
            &adj_list,
            &mut memo,
            &mut memo_d,
            &mut memo_f,
            &mut memo_df,
        )
    } else {
        0
    };
    Ok((ans1.to_string(), ans2.to_string()))
}

fn count_paths_part1(
    from: &str,
    adj_list: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, ItemType>,
) -> ItemType {
    if from == "out" {
        return 1;
    }
    if let Some(result) = memo.get(from) {
        return *result;
    }
    let mut result = 0;
    for next in adj_list.get(from).unwrap().iter() {
        result += count_paths_part1(next, adj_list, memo)
    }
    memo.insert(from.to_string(), result);
    result
}

#[expect(clippy::too_many_arguments)]
fn count_paths_part2(
    from: &str,
    needs_dac: bool,
    needs_fft: bool,
    adj_list: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, ItemType>,
    memo_d: &mut HashMap<String, ItemType>,
    memo_f: &mut HashMap<String, ItemType>,
    memo_df: &mut HashMap<String, ItemType>,
) -> ItemType {
    if from == "out" {
        return if needs_dac || needs_fft { 0 } else { 1 };
    }
    let current_dac = from == "dac";
    let current_fft = from == "fft";
    let mut result = 0;
    let m = match (needs_dac, needs_fft) {
        (false, false) => &memo,
        (true, false) => &memo_d,
        (false, true) => &memo_f,
        (true, true) => &memo_df,
    };
    if let Some(result) = m.get(from) {
        return *result;
    }
    for next in adj_list.get(from).unwrap().iter() {
        result += count_paths_part2(
            next,
            needs_dac && !current_dac,
            needs_fft && !current_fft,
            adj_list,
            memo,
            memo_d,
            memo_f,
            memo_df,
        );
    }
    match (needs_dac, needs_fft) {
        (false, false) => memo.insert(from.to_string(), result),
        (true, false) => memo_d.insert(from.to_string(), result),
        (false, true) => memo_f.insert(from.to_string(), result),
        (true, true) => memo_df.insert(from.to_string(), result),
    };
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
    fn example2() {
        test_case(metadata, solve, 2);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_from_must_end_with_colon() {
        test_invalid_msg(
            &[&"from aaa you"],
            solve,
            "first word in input lines must end with `:`",
        );
    }

    #[test]
    fn invalid_missing_out() {
        test_invalid_msg(
            &[&"you: aaa"],
            solve,
            "input must contain rule with endpoint `out`",
        );
    }
}
