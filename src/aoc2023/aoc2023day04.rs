//! [aoc](https://adventofcode.com/2023/day/4)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::cmp;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 4,
        title: "Scratchcards",
        solution: ("21088", "6874754"),
        example_solutions: vec![("13", "30")],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut win_numbers = Vec::new();
    let mut have_numbers = Vec::new();
    for &line in input {
        let mut a_iter = line.split(": ");
        let a1 = a_iter.next().unwrap();
        let a2 = a_iter
            .next()
            .ok_or("Card numbers must be followed by a `:`")?;
        if !a1.starts_with("Card ") {
            Err("line must start with `Card `")?
        }
        let mut a2_iter = a2.split(" | ");
        let a2_1 = a2_iter.next().unwrap();
        let a2_2 = a2_iter
            .next()
            .ok_or("win and have numbers must be separated by |`:`")?;
        win_numbers.push(
            a2_1.split_whitespace()
                .map(|x| {
                    x.trim()
                        .parse::<ItemType>()
                        .map_err(|_| format!("win numbers must be integers, found `{}`", x).into())
                })
                .collect::<Result<Vec<_>, PuzzleError>>()?,
        );
        have_numbers.push(
            a2_2.split_whitespace()
                .map(|x| {
                    x.trim()
                        .parse::<ItemType>()
                        .map_err(|_| format!("have numbers must be integers, found `{}`", x).into())
                })
                .collect::<Result<Vec<_>, PuzzleError>>()?,
        );
    }
    // ---------- Part 1 + 2
    let mut ans1 = 0;
    let mut count_cards = vec![1; input.len()];
    for (idx_card, have_list) in have_numbers.iter().enumerate() {
        let mut count_match = 0;
        for number in have_list {
            if win_numbers[idx_card].contains(number) {
                count_match += 1;
            }
        }
        if count_match > 0 {
            ans1 += 1 << (count_match - 1);
            for i in (idx_card + 1)..cmp::min(input.len(), idx_card + 1 + count_match) {
                count_cards[i] += count_cards[idx_card];
            }
        }
    }
    let ans2 = count_cards.iter().sum::<ItemType>();
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
    fn invalid_must_separate_with_colon() {
        test_invalid_msg(
            &[&"Card 1 a 41 48 83 86 17 | 83 86  6 31 17  9 48 53"],
            solve,
            "Card numbers must be followed by a `:`",
        );
    }

    #[test]
    fn invalid_must_separate_with_vertical_line() {
        test_invalid_msg(
            &[&"Card 1: 41 48 83 86 17 a 83 86  6 31 17  9 48 53"],
            solve,
            "win and have numbers must be separated by |`:`",
        );
    }

    #[test]
    fn invalid_must_start_with_card() {
        test_invalid_msg(
            &[&"Karte 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"],
            solve,
            "line must start with `Card `",
        );
    }

    #[test]
    fn invalid_win_number_must_be_integer() {
        test_invalid_msg(
            &[&"Card 1: 41 a 83 86 17 | 83 86  6 31 17  9 48 53"],
            solve,
            "win numbers must be integers",
        );
    }

    #[test]
    fn invalid_have_number_must_be_integer() {
        test_invalid_msg(
            &[&"Card 1: 41 48 83 86 17 | 83 86  b 31 17  9 48 53"],
            solve,
            "have numbers must be integers",
        );
    }
}
