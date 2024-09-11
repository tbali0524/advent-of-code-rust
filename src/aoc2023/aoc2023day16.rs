//! [aoc](https://adventofcode.com/2022/day/16)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashSet;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 16,
        title: "The Floor Will Be Lava",
        solution: ("7472", "7716"),
        example_solutions: vec![("46", "51")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let max_y = input.len();
    let max_x = input[0].len();
    let grid = input
        .iter()
        .map(|&x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    if grid.iter().any(|line| line.len() != max_x) {
        Err("grid must be rectangular")?;
    }
    if grid
        .iter()
        .any(|line| line.iter().any(|&c| !".|-/\\".contains(c)))
    {
        Err("invalid character in grid")?;
    }
    // ---------- Part 1
    let ans1 = sim_beam(&grid, 0, 0, 0);
    // ---------- Part 2
    let mut start_positions = Vec::new();
    for x in 0..max_x as i32 {
        start_positions.push((x, 0, 1));
        start_positions.push((x, max_y as i32 - 1, 3));
    }
    for y in 0..max_y as i32 {
        start_positions.push((0, y, 0));
        start_positions.push((max_x as i32 - 1, y, 2));
    }
    let mut ans2 = 0;
    for (x, y, dir) in start_positions.iter() {
        let energy = sim_beam(&grid, *x, *y, *dir);
        if energy > ans2 {
            ans2 = energy;
        }
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

fn sim_beam(grid: &[Vec<char>], start_x: i32, start_y: i32, start_dir: i32) -> usize {
    let max_y = grid.len() as i32;
    let max_x = grid[0].len() as i32;
    let mut energized = HashSet::new();
    let mut visited = HashSet::new();
    visited.insert((start_x, start_y, start_dir));
    let mut q = Vec::new();
    q.push((start_x, start_y, start_dir));
    let mut read_idx = 0;
    while read_idx < q.len() {
        let (x, y, dir) = q[read_idx];
        read_idx += 1;
        let c = grid[y as usize][x as usize];
        energized.insert((x, y));
        for idx in 0..=1 {
            let next_dir = next_direction(dir, c, idx);
            let (dx, dy) = match next_dir {
                0 => (1, 0),  // east
                1 => (0, 1),  // south
                2 => (-1, 0), // west
                3 => (0, -1), // north
                _ => (0, 0),
            };
            let next_x = x + dx;
            let next_y = y + dy;
            if next_x < 0 || next_x >= max_x || next_y < 0 || next_y >= max_y {
                continue;
            }
            if visited.contains(&(next_x, next_y, next_dir)) {
                continue;
            }
            q.push((next_x, next_y, next_dir));
            visited.insert((next_x, next_y, next_dir));
        }
    }
    energized.len()
}

fn next_direction(dir: i32, c: char, idx: usize) -> i32 {
    match dir {
        0 => match c {
            '.' => [0, -1][idx],
            '-' => [0, -1][idx],
            '|' => [1, 3][idx],
            '/' => [3, -1][idx],
            '\\' => [1, -1][idx],
            _ => -1,
        },
        1 => match c {
            '.' => [1, -1][idx],
            '-' => [0, 2][idx],
            '|' => [1, -1][idx],
            '/' => [2, -1][idx],
            '\\' => [0, -1][idx],
            _ => -1,
        },
        2 => match c {
            '.' => [2, -1][idx],
            '-' => [2, -1][idx],
            '|' => [1, 3][idx],
            '/' => [1, -1][idx],
            '\\' => [3, -1][idx],
            _ => -1,
        },
        3 => match c {
            '.' => [3, -1][idx],
            '-' => [0, 2][idx],
            '|' => [3, -1][idx],
            '/' => [0, -1][idx],
            '\\' => [2, -1][idx],
            _ => -1,
        },
        _ => -1,
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
    fn invalid_must_be_rectangular() {
        test_invalid_msg(&[&".-", &"-./"], solve, "must be rectangular");
    }

    #[test]
    fn invalid_must_contain_only_valid_chars() {
        test_invalid_msg(&[&".-", &"a."], solve, "invalid character in grid");
    }
}
