//! [aoc](https://adventofcode.com/2023/day/17)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 17,
        title: "Clumsy Crucible",
        solution: ("684", "822"),
        example_solutions: vec![("102", "94"), ("0", "71")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let max_x = input[0].len();
    let grid = input
        .iter()
        .map(|&x| {
            x.chars()
                .map(|c| c.to_digit(10).ok_or("invalid character in grid"))
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    if grid.iter().any(|line| line.len() != max_x) {
        Err("grid must be rectangular")?;
    }
    // ---------- Part 1 + 2
    let ans1 = solve_part(&grid, 1);
    let ans2 = solve_part(&grid, 2);
    Ok((ans1.to_string(), ans2.to_string()))
}

// seems like false positive clippy warning: without the `(next_dir - dir) as i32).abs()` cast
// we would get "can't call method `abs` on ambiguous numeric type `{integer}`"
#[expect(clippy::unnecessary_cast)]
fn solve_part(grid: &[Vec<u32>], part: usize) -> u32 {
    let max_y = grid.len();
    let max_x = grid[0].len();
    let max_steps = if part == 1 { 3 } else { 10 };
    let mut ans = 0;
    let mut pq = PriorityQueue::new();
    let mut best_costs = HashMap::new();
    // x, y, dir, straight-steps
    let item = (0, 0, 0, 0);
    pq.push(item, Reverse(0));
    best_costs.insert(item, 0);
    if part == 2 {
        let item = (0, 0, 1, 0);
        pq.push(item, Reverse(0));
        best_costs.insert(item, 0);
    }
    while !pq.is_empty() {
        let (item, priority) = pq.pop().unwrap();
        let total_cost = priority.0;
        let (x, y, dir, steps) = item;
        if best_costs.contains_key(&item) && *best_costs.get(&item).unwrap() != total_cost {
            continue;
        }
        if x == max_x as i32 - 1 && y == max_y as i32 - 1 && (part == 1 || steps >= 4) {
            ans = total_cost;
            break;
        }
        for next_dir in 0..4 {
            if ((next_dir - dir) as i32).abs() == 2 {
                // turning back
                continue;
            }
            let next_steps = if next_dir == dir {
                if steps >= max_steps {
                    continue;
                }
                steps + 1
            } else {
                if part == 2 && steps < 4 {
                    continue;
                }
                1
            };
            let (dx, dy) = [(1, 0), (0, 1), (-1, 0), (0, -1)][next_dir as usize];
            let next_x = x + dx;
            let next_y = y + dy;
            if next_x < 0 || next_x >= max_x as i32 || next_y < 0 || next_y >= max_y as i32 {
                continue;
            }
            let next_cost = total_cost + grid[next_y as usize][next_x as usize];
            let next_item = (next_x, next_y, next_dir, next_steps);
            if best_costs.contains_key(&next_item)
                && *best_costs.get(&next_item).unwrap() <= next_cost
            {
                continue;
            }
            best_costs.insert(next_item, next_cost);
            pq.push(next_item, Reverse(next_cost));
        }
    }
    ans
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
    fn invalid_must_be_rectangular() {
        test_invalid_msg(&[&"12", &"123"], solve, "must be rectangular");
    }

    #[test]
    fn invalid_must_contain_only_valid_chars() {
        test_invalid_msg(&[&"12", &"1a3"], solve, "invalid character in grid");
    }
}
