//! [aoc](https://adventofcode.com/2024/day/16)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 16,
        title: "Reindeer Maze",
        solution: ("107468", "533"),
        example_solutions: vec![("7036", "45"), ("11048", "64")],
    }
}

const EMPTY: char = '.';
const WALL: char = '#';
const START: char = 'S';
const TARGET: char = 'E';
const STRAIGHT_COST: i32 = 1;
const TURN_COST: i32 = 1000;
const DELTA_XY: [(i32, i32); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)]; // must start with East, and in circular order

#[expect(clippy::unnecessary_cast)]
pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input7
    let max_y = input.len() as i32;
    let max_x = input[0].len() as i32;
    let mut start_x = 0;
    let mut start_y = 0;
    let mut has_start = false;
    let mut target_x = 0;
    let mut target_y = 0;
    let mut has_target = false;
    let grid = input
        .iter()
        .map(|&x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    if grid.iter().any(|line| line.len() != max_x as usize) {
        Err("grid must be rectangular")?;
    }
    for y in 0..max_y {
        for x in 0..max_x {
            match grid[y as usize][x as usize] {
                START => {
                    start_x = x;
                    start_y = y;
                    has_start = true;
                }
                TARGET => {
                    target_x = x;
                    target_y = y;
                    has_target = true;
                }
                WALL => (),
                EMPTY => (),
                _ => Err("invalid character in grid")?,
            }
        }
    }
    if !has_start || !has_target {
        Err("missing start or target position in grid")?;
    }
    // ---------- Part 1 + 2
    let mut ans1 = 0;
    let mut pq = PriorityQueue::new();
    let mut best_costs = HashMap::new();
    let mut on_best_path = HashSet::new();
    let start_dir = 0; // east
    let hash = (start_x, start_y, start_dir);
    let path = vec![(start_x, start_y)];
    let item = (start_x, start_y, start_dir, path);
    pq.push(item, Reverse(0));
    best_costs.insert(hash, 0);
    while !pq.is_empty() {
        let (item, priority) = pq.pop().unwrap();
        let total_cost = priority.0;
        if ans1 != 0 && total_cost > ans1 {
            break;
        }
        let (x, y, dir, path) = item;
        let hash = (x, y, dir);
        if best_costs.contains_key(&hash) && *best_costs.get(&hash).unwrap() != total_cost {
            continue;
        }
        if x == target_x && y == target_y {
            ans1 = total_cost;
            for pos in path.iter() {
                on_best_path.insert(*pos);
            }
            continue;
        }
        for next_dir in 0..4 {
            if ((next_dir - dir) as i32).abs() == 2 {
                // turning back
                continue;
            }
            let (dx, dy) = DELTA_XY[next_dir as usize];
            let next_x = x + dx;
            let next_y = y + dy;
            if next_x < 0 || next_x >= max_x || next_y < 0 || next_y >= max_y {
                continue;
            }
            if grid[next_y as usize][next_x as usize] == WALL {
                continue;
            }
            let mut next_cost = total_cost + STRAIGHT_COST;
            if next_dir != dir {
                next_cost += TURN_COST;
            }
            let next_hash = (next_x, next_y, next_dir);
            if best_costs.contains_key(&next_hash)
                && *best_costs.get(&next_hash).unwrap() < next_cost
            {
                continue;
            }
            let mut next_path = path.clone();
            next_path.push((next_x, next_y));
            let next_item = (next_x, next_y, next_dir, next_path);
            best_costs.insert(next_hash, next_cost);
            pq.push(next_item, Reverse(next_cost));
        }
    }
    let ans2 = on_best_path.len();
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
    fn example2() {
        test_case(metadata, solve, 2);
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
