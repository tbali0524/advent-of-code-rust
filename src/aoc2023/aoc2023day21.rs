//! [aoc](https://adventofcode.com/2023/day/21)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashSet;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 21,
        title: "Step Counter",
        solution: ("3820", "632421652138917"),
        // Part 2 solution is not valid for the example (different assumptions)
        example_solutions: vec![("16", "0")],
    }
}

const MAX_STEPS_EXAMPLE_PART1: usize = 6;
const MAX_STEPS_PART1: usize = 64;
const MAX_STEPS_PART2: usize = 26501365;
const EMPTY: char = '.';
const WALL: char = '#';
const START: char = 'S';
const DELTA_XY: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let max_y = input.len();
    let max_x = input[0].len();
    let mut grid = input
        .iter()
        .map(|&x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    if grid.iter().any(|line| line.len() != max_x) {
        Err("grid must be rectangular")?;
    }
    let mut start_x = 0;
    let mut start_y = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == START {
                start_x = x;
                start_y = y;
            } else if c != EMPTY && c != WALL {
                Err("invalid char in grid")?;
            }
        }
    }
    if start_x == 0 && start_y == 0 {
        Err("missing start")?;
    }
    grid[start_y][start_x] = EMPTY;
    // ---------- Part 1
    let max_steps = if max_y < 12 {
        MAX_STEPS_EXAMPLE_PART1
    } else {
        MAX_STEPS_PART1
    };
    let ans1 = solve_part1(&grid, start_x, start_y, max_steps);
    // ---------- Part 2
    if max_y < 12 {
        return Ok((ans1.to_string(), 0.to_string()));
    }
    let max_steps = self::MAX_STEPS_PART2;
    // Solution works only with following assumptions (valid for input, but not for example):
    // not checked here, but also the following columns must be also completely empty: 0, start_x, max_x - 1
    if max_x != max_y
        || max_x % 2 != 1
        || start_x != start_y
        || start_x != max_x / 2
        || max_steps <= 2 * max_x
        || grid[0].iter().any(|&x| x != EMPTY)
        || grid[start_y].iter().any(|&x| x != EMPTY)
        || grid[max_y - 1].iter().any(|&x| x != EMPTY)
    {
        Err("invalid input for part 2")?;
    }
    let ans2 = solve_part2(&grid, start_x, start_y, max_steps);
    Ok((ans1.to_string(), ans2.to_string()))
}

fn solve_part1(grid: &[Vec<char>], from_x: usize, from_y: usize, max_steps: usize) -> usize {
    let max_y = grid.len();
    let max_x = grid[0].len();
    let mut result = HashSet::new();
    let pos = (from_x, from_y, 0);
    let mut visited = HashSet::new();
    visited.insert(pos);
    let mut q = Vec::new();
    q.push(pos);
    let mut idx_read = 0;
    while idx_read < q.len() {
        let pos = q[idx_read];
        idx_read += 1;
        let (x, y, step) = pos;
        if step % 2 == max_steps % 2 {
            result.insert((x, y));
            if step == max_steps {
                continue;
            }
        }
        for (dx, dy) in &DELTA_XY {
            let next_x = x as i32 + dx;
            let next_y = y as i32 + dy;
            if next_x < 0 || next_x >= max_x as i32 || next_y < 0 || next_y >= max_y as i32 {
                continue;
            }
            if grid[next_y as usize][next_x as usize] != EMPTY {
                continue;
            }
            let next_pos = (next_x as usize, next_y as usize, step + 1);
            if visited.contains(&next_pos) {
                continue;
            }
            q.push(next_pos);
            visited.insert(next_pos);
        }
    }
    result.len()
}

fn solve_part2(grid: &[Vec<char>], start_x: usize, start_y: usize, max_steps: usize) -> usize {
    let max_y = grid.len();
    let max_x = grid[0].len();
    let mut ans2 = 0;
    let half = max_x / 2;
    let n = (max_steps - max_x - half) / max_x;
    let remaining_steps = max_steps - (half + n * max_x + 1);
    for (dx, dy) in &DELTA_XY {
        let from_x = start_x as i32 + dx * start_x as i32;
        let from_y = start_y as i32 + dy * start_y as i32;
        ans2 += solve_part1(grid, from_x as usize, from_y as usize, remaining_steps);
    }
    let remaining_steps1 = remaining_steps - (half + 1);
    let remaining_steps2 = remaining_steps1 + max_x;
    for (dx, dy) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
        ans2 += (n + 1) * solve_part1(grid, dx * (max_x - 1), dy * (max_y - 1), remaining_steps1);
        ans2 += n * solve_part1(grid, dx * (max_x - 1), dy * (max_y - 1), remaining_steps2);
    }
    let result_center = solve_part1(grid, start_x, start_y, max_x);
    let result_checkerboard = solve_part1(grid, start_x, start_y, max_x - 1);
    let count_center = if n % 2 == 1 { n * n } else { (n + 1) * (n + 1) };
    let count_checkerboard = (2 * n * n) + (2 * n) + 1 - count_center;
    ans2 += count_center * result_center + count_checkerboard * result_checkerboard;
    ans2
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
    fn invalid_grid_must_be_rectangular() {
        test_invalid_msg(&["##", "S"], solve, "grid must be rectangular");
    }

    #[test]
    fn invalid_grid_must_contain_valid_chars() {
        test_invalid_msg(&[".S", "#a"], solve, "invalid char in grid");
    }

    #[test]
    fn invalid_grid_missing_start() {
        test_invalid_msg(&["#.#", "#.#", "#.#"], solve, "missing start");
    }
}
