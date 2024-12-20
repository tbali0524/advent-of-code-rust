//! [aoc](https://adventofcode.com/2024/day/20)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::cmp::max;
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 20,
        title: "Race Condition",
        solution: ("1450", "1015247"),
        example_solutions: vec![("10", "285")],
    }
}

const DELTA_XY: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
const MAX_CHEAT_LEN_PART1: i32 = 2;
const MAX_CHEAT_LEN_PART2: i32 = 20;
const SAVING_THRESHOLD_PART1_EXAMPLE: i32 = 10;
const SAVING_THRESHOLD_PART2_EXAMPLE: i32 = 50;
const SAVING_THRESHOLD: i32 = 100;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let max_y = input.len() as i32;
    let max_x = input[0].len() as i32;
    let mut start_x = 0;
    let mut start_y = 0;
    let mut has_start = false;
    let mut target_x = 0;
    let mut target_y = 0;
    let mut has_target = false;
    let mut grid = Vec::new();
    for (y, &row) in input.iter().enumerate() {
        grid.push(Vec::new());
        for (x, c) in row.chars().enumerate() {
            let item = match c {
                '.' => Some(-1),
                '#' => None,
                'S' => {
                    start_x = x as i32;
                    start_y = y as i32;
                    has_start = true;
                    Some(-1)
                }
                'E' => {
                    target_x = x as i32;
                    target_y = y as i32;
                    has_target = true;
                    Some(-1)
                }
                _ => Err("invalid character in grid")?,
            };
            grid[y].push(item);
        }
        if grid[y].len() as i32 != max_x {
            Err("grid must be rectangular")?;
        }
    }
    if !has_start || !has_target {
        Err("missing start or target position in grid")?;
    }
    // ---------- Part 1 + 2
    // fill up grid with distances from target
    let mut x = target_x;
    let mut y = target_y;
    let mut dist = 0;
    loop {
        grid[y as usize][x as usize] = Some(dist);
        if x == start_x && y == start_y {
            break;
        }
        for (dx, dy) in DELTA_XY {
            let x1 = x + dx;
            let y1 = y + dy;
            if x1 < 0 || x1 >= max_x || y1 < 0 || y1 >= max_y {
                continue;
            }
            if grid[y1 as usize][x1 as usize] == Some(-1) {
                x = x1;
                y = y1;
                break;
            }
        }
        dist += 1;
    }
    let ans1;
    let ans2;
    if max_y == 15 {
        ans1 = solve_part(&grid, MAX_CHEAT_LEN_PART1, SAVING_THRESHOLD_PART1_EXAMPLE);
        ans2 = solve_part(&grid, MAX_CHEAT_LEN_PART2, SAVING_THRESHOLD_PART2_EXAMPLE);
    } else {
        ans1 = solve_part(&grid, MAX_CHEAT_LEN_PART1, SAVING_THRESHOLD);
        ans2 = solve_part(&grid, MAX_CHEAT_LEN_PART2, SAVING_THRESHOLD);
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

fn solve_part(grid: &[Vec<Option<i32>>], max_cheat_len: i32, threshold: i32) -> usize {
    let max_y = grid.len() as i32;
    let max_x = grid[0].len() as i32;
    let mut cheats = HashMap::new();
    for y in 0..max_y {
        for x in 0..max_x {
            if grid[y as usize][x as usize].is_none() {
                continue;
            }
            for (dx, dy) in DELTA_XY {
                let x1 = x + dx;
                let y1 = y + dy;
                if x1 < 0 || x1 >= max_x || y1 < 0 || y1 >= max_y {
                    continue;
                }
                for y2 in (y1 - (max_cheat_len - 1))..=(y1 + max_cheat_len - 1) {
                    if y2 < 0 || y2 >= max_y {
                        continue;
                    }
                    let dx = max_cheat_len - 1 - (y2 - y1).abs();
                    for x2 in (x1 - dx)..=(x1 + dx) {
                        if x2 < 0 || x2 >= max_x {
                            continue;
                        }
                        if grid[y2 as usize][x2 as usize].is_none() {
                            continue;
                        }

                        let cheat_len = (y2 - y1).abs() + (x2 - x1).abs() + 1;
                        let saving = grid[y as usize][x as usize].unwrap()
                            - grid[y2 as usize][x2 as usize].unwrap()
                            - cheat_len;
                        if saving <= 0 {
                            continue;
                        }
                        cheats
                            .entry((x, y, x2, y2))
                            .and_modify(|e| *e = max(*e, saving))
                            .or_insert(saving);
                    }
                }
            }
        }
    }
    cheats
        .values()
        .filter(|&saving| *saving >= threshold)
        .count()
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
    fn invalid_missing_pos() {
        test_invalid_msg(
            &[&"#S", &".#"],
            solve,
            "missing start or target position in grid",
        );
    }

    #[test]
    fn invalid_character() {
        test_invalid_msg(&[&"aSE"], solve, "invalid character in grid");
    }

    #[test]
    fn invalid_grid_not_rectangular() {
        test_invalid_msg(&[&".S", &"E"], solve, "grid must be rectangular");
    }
}
