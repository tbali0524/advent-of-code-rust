//! [aoc](https://adventofcode.com/2023/day/3)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::{HashMap, HashSet};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 3,
        title: "Gear Ratios",
        solution: ("530849", "84900879"),
        example_solutions: vec![("4361", "467835")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Part 1 + 2
    let mut ans1 = 0;
    let max_y = input.len() as i32;
    let max_x = input[0].len() as i32;
    let mut is_in_number = false;
    let mut as_adjacent = false;
    let mut adjacent_gears = HashSet::new();
    let mut gear_numbers = HashMap::<(i32, i32), Vec<i32>>::new();
    let mut number = 0;
    for (y, &line) in input.iter().enumerate() {
        for (x, c) in line.as_bytes().iter().enumerate() {
            let is_digit = c.is_ascii_digit();
            if !is_digit && !is_in_number {
                continue;
            }
            if is_digit {
                is_in_number = true;
                number = number * 10 + (*c as char).to_digit(10).unwrap() as i32;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = (x as i32) + dx;
                        let ny = (y as i32) + dy;
                        if nx < 0 || nx >= max_x || ny < 0 || ny >= max_y || (dx == 0 && dy == 0) {
                            continue;
                        }
                        let nc = input[ny as usize].as_bytes()[nx as usize];
                        if nc != b'.' && !nc.is_ascii_digit() {
                            as_adjacent = true;
                            if nc == b'*' {
                                adjacent_gears.insert((nx, ny));
                            }
                        }
                    }
                }
            }
            if !is_digit || (x as i32) == max_x - 1 {
                if as_adjacent {
                    ans1 += number;
                }
                for hash in &adjacent_gears {
                    gear_numbers.entry(*hash).or_default().push(number);
                }
                number = 0;
                as_adjacent = false;
                is_in_number = false;
                adjacent_gears = HashSet::new();
            }
        }
    }
    let mut ans2 = 0;
    for numbers in gear_numbers.values() {
        if numbers.len() == 2 {
            ans2 += numbers[0] * numbers[1];
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
}
