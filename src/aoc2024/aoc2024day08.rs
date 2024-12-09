//! [aoc](https://adventofcode.com/2024/day/8)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::{HashMap, HashSet};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 8,
        title: "Resonant Collinearity",
        solution: ("320", "1157"),
        example_solutions: vec![("14", "34"), ("3", "9")],
    }
}

const EMPTY: char = '.';

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let max_y = input.len() as i32;
    let max_x = input[0].len() as i32;
    let grid = input
        .iter()
        .map(|&x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    if grid.iter().any(|line| line.len() != max_x as usize) {
        Err("grid must be rectangular")?;
    }
    let mut antennas = HashMap::new();
    for y in 0..max_y {
        for x in 0..max_x {
            let c = grid[y as usize][x as usize];
            if c == EMPTY {
                continue;
            }
            let pos = (x, y);
            antennas
                .entry(c)
                .or_insert_with(Vec::new)
                .push(pos.to_owned());
        }
    }
    // ---------- Part 1
    let mut antinodes_part1 = HashSet::new();
    for positions in antennas.values() {
        for &pos1 in positions.iter() {
            for &pos2 in positions.iter() {
                if pos1 == pos2 {
                    continue;
                }
                let (x1, y1) = pos1;
                let (x2, y2) = pos2;
                let x = x2 + x2 - x1;
                let y = y2 + y2 - y1;
                if (0..max_x).contains(&x) && (0..max_y).contains(&y) {
                    antinodes_part1.insert((x, y));
                }
                let x = x1 + x1 - x2;
                let y = y1 + y1 - y2;
                if (0..max_x).contains(&x) && (0..max_y).contains(&y) {
                    antinodes_part1.insert((x, y));
                }
            }
        }
    }
    let ans1 = antinodes_part1.len();
    // ---------- Part 2
    let mut antinodes_part2 = HashSet::new();
    for positions in antennas.values() {
        for &pos1 in positions.iter() {
            for &pos2 in positions.iter() {
                if pos1 == pos2 {
                    continue;
                }
                let (x1, y1) = pos1;
                let (x2, y2) = pos2;
                let mut x = x2;
                let mut y = y2;
                while (0..max_x).contains(&x) && (0..max_y).contains(&y) {
                    antinodes_part2.insert((x, y));
                    x += x2 - x1;
                    y += y2 - y1;
                }
                let mut x = x1;
                let mut y = y1;
                while (0..max_x).contains(&x) && (0..max_y).contains(&y) {
                    antinodes_part2.insert((x, y));
                    x += x1 - x2;
                    y += y1 - y2;
                }
            }
        }
    }
    let ans2 = antinodes_part2.len();
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
    fn invalid_grid_not_rectangular() {
        test_invalid_msg(&[&"a.", &"."], solve, "grid must be rectangular");
    }
}
