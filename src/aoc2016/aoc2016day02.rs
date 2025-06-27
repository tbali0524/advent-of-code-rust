//! [aoc](https://adventofcode.com/2016/day/2)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2016,
        day: 2,
        title: "Bathroom Security",
        solution: ("24862", "46C91"),
        example_solutions: vec![("1985", "5DB3")],
    }
}

const KEYS_PART2: [&str; 5] = ["00100", "02340", "56789", "0ABC0", "00D00"];

#[expect(clippy::manual_range_contains)]
pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Part 1
    let mut ans1 = 0;
    let mut x = 1;
    let mut y = 0;
    for &line in input {
        for c in line.chars() {
            let (dx, dy) = match c {
                'U' => (0, -1),
                'R' => (1, 0),
                'D' => (0, 1),
                'L' => (-1, 0),
                _ => Err("direction must be one of 'URDL'")?,
            };
            let (x1, y1) = (x + dx, y + dy);
            if x1 < 0 || x1 >= 3 || y1 < 0 || y1 >= 3 {
                continue;
            }
            (x, y) = (x1, y1);
        }
        ans1 = 10 * ans1 + y * 3 + x + 1;
    }
    // ---------- Part 2
    let mut ans2 = String::new();
    let mut x = 0;
    let mut y = 2;
    for &line in input {
        for c in line.chars() {
            let (dx, dy) = match c {
                'U' => (0, -1),
                'R' => (1, 0),
                'D' => (0, 1),
                'L' => (-1, 0),
                _ => (0, 0),
            };
            let (x1, y1) = (x + dx, y + dy);
            if x1 < 0 || x1 >= 5 || y1 < 0 || y1 >= 5 {
                continue;
            }
            if KEYS_PART2[y1 as usize].chars().nth(x1 as usize).unwrap() == '0' {
                continue;
            }
            (x, y) = (x1, y1);
        }
        ans2.push(KEYS_PART2[y as usize].chars().nth(x as usize).unwrap());
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
    fn invalid_direction() {
        test_invalid_msg(&["A"], solve, "direction must be one of 'URDL'");
    }
}
