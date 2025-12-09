//! [aoc](https://adventofcode.com/2025/day/7)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2025,
        day: 7,
        title: "Laboratories",
        solution: ("1573", "15093663987272"),
        example_solutions: vec![("21", "40")],
    }
}

#[allow(clippy::needless_range_loop)]
pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut grid = Vec::new();
    for &line in input.iter() {
        grid.push(line.chars().collect::<Vec<_>>());
    }
    let start = if let Some(x) = grid[0].iter().position(|&c| c == 'S') {
        x
    } else {
        Err("first input line must contain an `S`")?
    };
    // ---------- Part 1 + 2
    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut tachyons = vec![(start, 1)];
    for y in 1..input.len() {
        let mut next_tachyons = Vec::<(usize, u64)>::new();
        for &(x, t) in &tachyons {
            if grid[y][x] == '.' {
                if next_tachyons.is_empty() {
                    next_tachyons.push((x, t));
                    continue;
                }
                let &(prev_x, prev_t) = next_tachyons.last().unwrap();
                if prev_x != x {
                    next_tachyons.push((x, t));
                    continue;
                }
                next_tachyons.pop();
                next_tachyons.push((x, t + prev_t));
                continue;
            }
            if grid[y][x] != '^' {
                Err("input lines except first line must contain only `.` or `^`")?;
            }
            ans1 += 1;
            if next_tachyons.is_empty() {
                next_tachyons.push((x - 1, t));
                next_tachyons.push((x + 1, t));
                continue;
            }
            let &(prev_x, prev_t) = next_tachyons.last().unwrap();
            if prev_x != x - 1 {
                next_tachyons.push((x - 1, t));
                next_tachyons.push((x + 1, t));
                continue;
            }
            next_tachyons.pop();
            next_tachyons.push((x - 1, t + prev_t));
            next_tachyons.push((x + 1, t));
        }
        tachyons = next_tachyons;
    }
    for &(_, t) in &tachyons {
        ans2 += t;
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
    fn invalid_must_have_start_position() {
        test_invalid_msg(
            &[&"...", &".^."],
            solve,
            "first input line must contain an `S`",
        );
    }

    #[test]
    fn invalid_character() {
        test_invalid_msg(
            &[&".S.", &".a."],
            solve,
            "input lines except first line must contain only `.` or `^`",
        );
    }
}
