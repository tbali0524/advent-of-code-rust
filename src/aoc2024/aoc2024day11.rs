//! [aoc](https://adventofcode.com/2024/day/11)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 11,
        title: "Plutonian Pebbles",
        solution: ("197157", "234430066982597"),
        example_solutions: vec![("7", "0"), ("55312", "0")],
    }
}

const MAX_BLINKS_PART1: u8 = 25;
const MAX_BLINKS_PART2: u8 = 75;

type ItemType = u64;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() != 1 {
        Err("input must have a single line")?;
    }
    let data = input[0]
        .split_whitespace()
        .map(|x| {
            x.parse::<ItemType>()
                .map_err(|_| format!("input must contain only integers, found `{x}`"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    // ---------- Part 1 + 2
    let max_blinks_part1 = if data.len() == 5 {
        1 // example 1
    } else {
        MAX_BLINKS_PART1
    };
    let max_blinks_part2 = if data.len() <= 5 {
        1 // example 1 + 2
    } else {
        MAX_BLINKS_PART2
    };
    let mut memo = HashMap::new();
    let mut ans1 = 0;
    let mut ans2 = 0;
    for &stone in &data {
        ans1 += blink(stone, max_blinks_part1, &mut memo);
        ans2 += blink(stone, max_blinks_part2, &mut memo);
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

fn blink(
    stone: ItemType,
    count_blinks: u8,
    memo: &mut HashMap<(ItemType, u8), ItemType>,
) -> ItemType {
    if count_blinks == 0 {
        return 1;
    }
    let key = (stone, count_blinks);
    if memo.contains_key(&key) {
        return *memo.get(&key).unwrap();
    }
    let result;
    if stone == 0 {
        result = blink(1, count_blinks - 1, memo);
    } else {
        let stone_string = stone.to_string();
        if stone_string.len() % 2 == 0 {
            let divisor = (10 as ItemType).pow(stone_string.len() as u32 / 2);
            result = blink(stone / divisor, count_blinks - 1, memo)
                + blink(stone % divisor, count_blinks - 1, memo);
        } else {
            result = blink(stone * 2024, count_blinks - 1, memo);
        }
    }
    memo.insert(key, result);
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
    fn invalid_single_line() {
        test_invalid_msg(&["1 2", "3"], solve, "input must have a single line");
    }

    #[test]
    fn invalid_must_have_contain_integers() {
        test_invalid_msg(&["a 2"], solve, "input must contain only integers");
    }
}
