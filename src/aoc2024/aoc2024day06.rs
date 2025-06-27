//! [aoc](https://adventofcode.com/2024/day/6)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashSet;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 6,
        title: "Guard Gallivant",
        solution: ("4515", "1309"),
        example_solutions: vec![("41", "6")],
    }
}

const EMPTY: char = '.';
const WALL: char = '#';
const START: char = '^';
const DELTA_XY: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)]; // must start with UP

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let max_y = input.len() as i32;
    let max_x = input[0].len() as i32;
    let mut grid = input
        .iter()
        .map(|&x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    if grid.iter().any(|line| line.len() != max_x as usize) {
        Err("grid must be rectangular")?;
    }
    let mut start_x = 0;
    let mut start_y = 0;
    let mut has_start = false;
    for y in 0..max_y {
        for x in 0..max_x {
            match grid[y as usize][x as usize] {
                START => {
                    start_x = x;
                    start_y = y;
                    has_start = true;
                }
                WALL => (),
                EMPTY => (),
                _ => Err("invalid character in grid")?,
            }
        }
    }
    if !has_start {
        Err("missing start position in grid")?;
    }
    // ---------- Part 1
    let sim_result = sim_guard(start_x, start_y, &grid).ok_or("input already contains loop")?;
    let ans1 = sim_result.len();
    // ---------- Part 2
    let mut ans2 = 0;
    for pos in sim_result.iter() {
        let (block_x, block_y) = *pos;
        if block_x == start_x && block_y == start_y {
            continue;
        }
        grid[block_y as usize][block_x as usize] = WALL;
        if sim_guard(start_x, start_y, &grid).is_none() {
            ans2 += 1;
        }
        grid[block_y as usize][block_x as usize] = EMPTY;
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

fn sim_guard(start_x: i32, start_y: i32, grid: &[Vec<char>]) -> Option<HashSet<(i32, i32)>> {
    let max_y = grid.len() as i32;
    let max_x = grid[0].len() as i32;
    let mut x = start_x;
    let mut y = start_y;
    let mut dir = 0; // UP
    let mut visited_pos = HashSet::new();
    let mut visited_pos_dir = HashSet::new();
    'outer: loop {
        visited_pos.insert((x, y));
        visited_pos_dir.insert((x, y, dir));
        let mut x1 = x;
        let mut y1 = y;
        let mut turns = 0;
        for _ in 0..4 {
            let (dx, dy) = DELTA_XY[dir];
            x1 = x + dx;
            y1 = y + dy;
            if y1 < 0 || y1 >= max_y || x1 < 0 || x1 >= max_x {
                break 'outer;
            }
            if grid[y1 as usize][x1 as usize] != WALL {
                break;
            }
            dir = (dir + 1) % 4;
            turns += 1;
        }
        x = x1;
        y = y1;
        if turns == 4 || visited_pos_dir.contains(&(x, y, dir)) {
            return None;
        }
    }
    Some(visited_pos)
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

    // too slow, skipped
    #[test]
    #[ignore]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_missing_pos() {
        test_invalid_msg(&["#.", ".#"], solve, "missing start position in grid");
    }

    #[test]
    fn invalid_character() {
        test_invalid_msg(&["a"], solve, "invalid character in grid");
    }

    #[test]
    fn invalid_grid_not_rectangular() {
        test_invalid_msg(&["#.", "^"], solve, "grid must be rectangular");
    }

    #[test]
    fn invalid_grid_contains_loop() {
        test_invalid_msg(&["###", "#^#", "###"], solve, "input already contains loop");
    }
}
