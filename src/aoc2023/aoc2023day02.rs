//! [aoc](https://adventofcode.com/2023/day/2)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::cmp;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 2,
        title: "Cube Conundrum",
        solution: ("2617", "59795"),
        example_solutions: vec![("8", "2286")],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Part 1 + 2
    let mut ans1 = 0;
    let mut ans2 = 0;
    let bag = Hand {
        red: 12,
        green: 13,
        blue: 14,
    };
    for line in input {
        if !line.starts_with("Game ") {
            return Err(PuzzleError("input lines must start with Game".into()));
        }
        let mut split_all = line.split(": ");
        let id = split_all.next().unwrap()[5..]
            .parse::<ItemType>()
            .map_err(|_| PuzzleError("Game number must be an integer".into()))?;
        let split_hands = split_all
            .next()
            .ok_or(PuzzleError(
                "Game number and hands must be separated by :".into(),
            ))?
            .split("; ");
        let mut is_ok = true;
        let mut min_bag = Hand::default();
        for hand_str in split_hands {
            let mut hand = Hand::default();
            let items = hand_str.split(", ");
            for item in items {
                let mut split_color = item.split(' ');
                let count = split_color
                    .next()
                    .unwrap()
                    .parse::<ItemType>()
                    .map_err(|_| PuzzleError("color count must be an integer".into()))?;
                let color = split_color.next().ok_or(PuzzleError(
                    "color count and name must be space separated ".into(),
                ))?;
                match color {
                    "red" => {
                        hand.red = count;
                        min_bag.red = cmp::max(min_bag.red, hand.red);
                    }
                    "green" => {
                        hand.green = count;
                        min_bag.green = cmp::max(min_bag.green, hand.green);
                    }
                    "blue" => {
                        hand.blue = count;
                        min_bag.blue = cmp::max(min_bag.blue, hand.blue);
                    }
                    _ => return Err(PuzzleError("invalid color".into())),
                }
            }
            if !hand.is_possible(&bag) {
                is_ok = false;
            }
        }
        if is_ok {
            ans1 += id;
        }
        ans2 += min_bag.power();
    }

    Ok((ans1.to_string(), ans2.to_string()))
}

#[derive(Default)]
struct Hand {
    red: ItemType,
    green: ItemType,
    blue: ItemType,
}

impl Hand {
    fn is_possible(&self, bag: &Hand) -> bool {
        self.red <= bag.red && self.green <= bag.green && self.blue <= bag.blue
    }

    fn power(&self) -> ItemType {
        self.red * self.green * self.blue
    }
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
    fn invalid_must_start_with_game() {
        test_invalid(&[&"a"], solve);
    }

    #[test]
    fn invalid_game_must_be_int() {
        test_invalid(
            &[&"Game X: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"],
            solve,
        );
    }

    #[test]
    fn invalid_must_have_colon() {
        test_invalid(
            &[&"Game 1 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"],
            solve,
        );
    }

    #[test]
    fn invalid_color_must_be_int() {
        test_invalid(&[&"Game 1: 3 blue, X red; 1 red, 2 green, 6 blue"], solve);
    }

    #[test]
    fn invalid_color_name() {
        test_invalid(&[&"Game 1: 3 blue, 1 red; 1 red, 2 YELLOW, 6 blue"], solve);
    }
}
