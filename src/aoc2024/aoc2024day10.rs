//! [aoc](https://adventofcode.com/2024/day/10)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::{HashMap, HashSet};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 10,
        title: "Hoof It",
        solution: ("674", "1372"),
        example_solutions: vec![
            ("36", "0"),
            ("1", "0"),
            ("2", "0"),
            ("4", "0"),
            ("3", "0"),
            ("0", "3"),
            ("0", "13"),
            ("0", "227"),
            ("0", "81"),
        ],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let max_x = input[0].len() as i32;
    let grid = input
        .iter()
        .map(|&x| {
            x.chars()
                .map(|c| {
                    if c == '.' {
                        Ok(None)
                    } else {
                        let height = c.to_digit(10);
                        if height.is_some() {
                            Ok(height)
                        } else {
                            Err("input must contain only digits or `.`")
                        }
                    }
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<Vec<_>>, _>>()?;
    if grid.iter().any(|line| line.len() != max_x as usize) {
        Err("grid must be rectangular")?;
    }
    // ---------- Part 1 + 2
    let ans1 = solve_part(&grid, true);
    let ans2 = solve_part(&grid, false);
    Ok((ans1.to_string(), ans2.to_string()))
}

fn solve_part(grid: &[Vec<Option<u32>>], is_part1: bool) -> u64 {
    let max_y = grid.len() as i32;
    let max_x = grid[0].len() as i32;
    let mut scores = HashMap::new();
    for start_y in 0..max_y {
        for start_x in 0..max_x {
            if grid[start_y as usize][start_x as usize] != Some(0) {
                continue;
            }
            scores.insert((start_x, start_y), 0);
            let mut visited = HashSet::new();
            let mut q = Vec::new();
            q.push((start_x, start_y, 0));
            visited.insert((start_x, start_y));
            let mut idx_read = 0;
            while idx_read < q.len() {
                let (x, y, height) = q[idx_read];
                idx_read += 1;
                if height == 9 {
                    *scores.get_mut(&(start_x, start_y)).unwrap() += 1;
                    continue;
                }
                for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    let x1 = x + dx;
                    let y1 = y + dy;
                    if x1 < 0 || x1 >= max_x || y1 < 0 || y1 >= max_y {
                        continue;
                    }
                    if is_part1 && visited.contains(&(x1, y1)) {
                        continue;
                    }
                    let c = grid[y1 as usize][x1 as usize];
                    if let Some(height1) = c {
                        if height1 == height + 1 {
                            visited.insert((x1, y1));
                            q.push((x1, y1, height1));
                        }
                    }
                }
            }
        }
    }
    scores.values().sum::<u64>()
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
    fn example3() {
        test_case(metadata, solve, 3);
    }

    #[test]
    fn example4() {
        test_case(metadata, solve, 4);
    }

    #[test]
    fn example5() {
        test_case(metadata, solve, 5);
    }

    #[test]
    fn example6() {
        test_case(metadata, solve, 6);
    }

    #[test]
    fn example7() {
        test_case(metadata, solve, 7);
    }

    #[test]
    fn example8() {
        test_case(metadata, solve, 8);
    }

    #[test]
    fn example9() {
        test_case(metadata, solve, 9);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_character() {
        test_invalid_msg(&[&"a"], solve, "input must contain only digits or `.`");
    }

    #[test]
    fn invalid_grid_not_rectangular() {
        test_invalid_msg(&[&"01", &"2"], solve, "grid must be rectangular");
    }
}
