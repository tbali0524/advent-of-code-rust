//! [aoc](https://adventofcode.com/2024/day/22)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 22,
        title: "Monkey Market",
        solution: ("18261820068", "2044"),
        example_solutions: vec![("37327623", "0"), ("0", "23")],
    }
}

type ItemType = u64;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let data = input
        .iter()
        .map(|&x| {
            x.parse::<ItemType>()
                .map_err(|_| "input must contain only non-negative integers")
        })
        .collect::<Result<Vec<_>, _>>()?;
    // ---------- Part 1 + 2
    let mut ans1 = 0;
    let mut total_sales = HashMap::new();
    for &start in &data {
        let mut secret = start;
        let mut price = (start % 10) as u8;
        let mut hash = 0;
        let mut best_prices = HashMap::new();
        for i in 1..=2000 {
            let mut next_secret = (secret ^ (secret << 6)) & 0x00ff_ffff; // 24 bits
            next_secret = (next_secret ^ (next_secret >> 5)) & 0x00ff_ffff;
            next_secret = (next_secret ^ (next_secret << 11)) & 0x00ff_ffff;
            let next_price = (next_secret % 10) as u8;
            hash = ((hash << 5) | (10 + next_price - price) as usize) & 0x000f_ffff; // 4*5 = 20 bits
            if i >= 4 && !best_prices.contains_key(&hash) {
                best_prices.insert(hash, next_price);
            }
            secret = next_secret;
            price = next_price;
        }
        ans1 += secret;
        for (hash, price) in best_prices.iter() {
            total_sales
                .entry(*hash)
                .and_modify(|e| *e += *price as u32)
                .or_insert(*price as u32);
        }
    }
    let ans2 = total_sales.values().max().unwrap();
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
    fn example2() {
        test_case(metadata, solve, 2);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid() {
        test_invalid_msg(
            &["a"],
            solve,
            "input must contain only non-negative integers",
        );
    }
}
