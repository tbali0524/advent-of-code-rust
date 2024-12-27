//! [aoc](https://adventofcode.com/2017/day/22)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2017,
        day: 22,
        title: "Sporifica Virus",
        solution: ("5538", "2511090"),
        example_solutions: vec![("5587", "2511944")],
    }
}

const CLEAN: i32 = 0;
const _WEAKENED: i32 = 1;
const INFECTED: i32 = 2;
const _FLAGGED: i32 = 3;
const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)]; // dir == 0 must be up, list in clockwise order

#[expect(clippy::map_entry)]
pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse input
    let mut start_nodes = HashMap::new();
    let delta = input.len() / 2;
    for (idx, line) in input.iter().enumerate() {
        for i in 0..line.len() {
            if line.chars().nth(i).unwrap() == '#' {
                let hash = (i as i32 - delta as i32, idx as i32 - delta as i32);
                start_nodes.insert(hash, INFECTED);
            }
        }
    }
    // ---------- Part 1
    const MAX_BURSTS_PART1: usize = 10_000;
    let mut ans1 = 0;
    let mut nodes = start_nodes.clone();
    let mut x = 0;
    let mut y = 0;
    let mut dir = 0i32;
    for _ in 0..MAX_BURSTS_PART1 {
        let hash = (x, y);
        let ddir;
        if nodes.contains_key(&hash) {
            ddir = 1;
            nodes.remove(&hash);
        } else {
            ddir = -1;
            nodes.insert(hash, INFECTED);
            ans1 += 1;
        }
        dir = (dir + ddir + 4) % 4;
        let (dx, dy) = DIRS[dir as usize];
        x += dx;
        y += dy;
    }
    // ---------- Part 2
    const MAX_BURSTS_PART2: usize = 10_000_000;
    let mut ans2 = 0;
    nodes = start_nodes;
    x = 0;
    y = 0;
    dir = 0;
    for _ in 0..MAX_BURSTS_PART2 {
        let hash = (x, y);
        let mut state = *nodes.get(&hash).unwrap_or(&CLEAN);
        let ddir = state - 1;
        state = (state + 1) % 4;
        if state == CLEAN {
            nodes.remove(&hash);
        } else {
            nodes.insert(hash, state);
            if state == INFECTED {
                ans2 += 1;
            }
        }
        dir = (dir + ddir + 4) % 4;
        let (dx, dy) = DIRS[dir as usize];
        x += dx;
        y += dy;
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
