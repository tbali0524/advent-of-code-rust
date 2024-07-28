//! [aoc](https://adventofcode.com/2023/day/11)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 11,
        title: "Cosmic Expansion",
        solution: ("9329143", "710674907809"),
        example_solutions: vec![("374", "8410")],
    }
}

#[allow(clippy::needless_range_loop)]
pub fn solve(input: PuzzleInput) -> PuzzleResult {
    const EXPANSE_PART1: usize = 2;
    const EXPANSE_EXAMPLE_PART2: usize = 100;
    const EXPANSE_PART2: usize = 1_000_000;
    // ---------- Parse input
    let max_y = input.len();
    let max_x = input[0].len();
    let mut galaxies = Vec::new();
    let mut is_empy_cols = vec![true; max_x];
    let mut is_empty_rows = vec![true; max_y];
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((x, y));
                is_empty_rows[y] = false;
                is_empy_cols[x] = false;
            }
        }
    }
    // ---------- Part 1 + 2
    let mut ans1 = 0;
    let mut ans2 = 0;
    let is_example = max_y <= 10;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let x1 = std::cmp::min(galaxies[i].0, galaxies[j].0);
            let x2 = std::cmp::max(galaxies[i].0, galaxies[j].0);
            let y1 = std::cmp::min(galaxies[i].1, galaxies[j].1);
            let y2 = std::cmp::max(galaxies[i].1, galaxies[j].1);
            ans1 += x2 - x1 + y2 - y1;
            ans2 += x2 - x1 + y2 - y1;
            for x in x1..=x2 {
                if is_empy_cols[x] {
                    ans1 += EXPANSE_PART1 - 1;
                    ans2 += if is_example {
                        EXPANSE_EXAMPLE_PART2
                    } else {
                        EXPANSE_PART2
                    } - 1;
                }
            }
            for y in y1..=y2 {
                if is_empty_rows[y] {
                    ans1 += EXPANSE_PART1 - 1;
                    ans2 += if is_example {
                        EXPANSE_EXAMPLE_PART2
                    } else {
                        EXPANSE_PART2
                    } - 1;
                }
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
    fn example1() {
        test_case(metadata, solve, 1);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }
}
