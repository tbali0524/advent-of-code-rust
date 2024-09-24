//! [aoc](https://adventofcode.com/2023/day/18)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::cmp;
use std::collections::{HashMap, HashSet};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 18,
        title: "Lavaduct Lagoon",
        solution: ("40131", "104454050898331"),
        example_solutions: vec![("62", "952408144115")],
    }
}

type ItemType = i64;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut directions = String::new();
    let mut directions_part2 = String::new();
    let mut steps = Vec::new();
    let mut steps_part2 = Vec::new();
    for &line in input {
        let mut a_iter = line.split(' ');
        let direction = a_iter.next().unwrap().chars().next().unwrap();
        if !['U', 'R', 'D', 'L'].contains(&direction) {
            Err("direction must be one of `URDL`")?;
        }
        let step = a_iter
            .next()
            .ok_or("missing step")?
            .parse::<ItemType>()
            .map_err(|_| "step must be integer")?;
        let c = a_iter.next().ok_or("missing hex number")?;
        if a_iter.next().is_some() {
            Err("each line must contain 3 items, separated by space")?;
        }
        if c.len() != 9
            || c.as_bytes()[0] != b'('
            || c.as_bytes()[1] != b'#'
            || c.as_bytes()[8] != b')'
        {
            Err("hex number must be in format `(#abcdef)`")?;
        }
        directions.push(direction);
        steps.push(step);
        let dir_idx = c
            .chars()
            .nth(7)
            .unwrap()
            .to_digit(10)
            .ok_or("direction index must be a digit")?;
        if dir_idx >= 4 {
            Err("direction index must be a between 0 and 3")?;
        }
        let direction_part2 = ['U', 'R', 'D', 'L'][dir_idx as usize];
        let step_part2 =
            ItemType::from_str_radix(&c[2..7], 16).map_err(|_| "invalid hex number")?;
        directions_part2.push(direction_part2);
        steps_part2.push(step_part2);
    }
    // ---------- Part 1
    let ans1 = solve_part1(&directions, &steps);
    // ---------- Part 2
    let ans2 = solve_part2(&directions_part2, &steps_part2);
    Ok((ans1.to_string(), ans2.to_string()))
}

fn dir_delta(c: char) -> (ItemType, ItemType) {
    match c {
        'R' => (1, 0),
        'D' => (0, 1),
        'L' => (-1, 0),
        'U' => (0, -1),
        _ => (0, 0),
    }
}

fn solve_part1(directions: &str, steps: &[ItemType]) -> ItemType {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    // traverse loop, fill out edge_block and compute x,y boundaries
    let mut x = 0;
    let mut y = 0;
    let mut edge_blocks = HashSet::new();
    edge_blocks.insert((x, y));
    for (i, direction) in directions.chars().enumerate() {
        let (dx, dy) = dir_delta(direction);
        for _ in 0..steps[i] {
            x += dx;
            y += dy;
            edge_blocks.insert((x, y));
        }
        min_x = cmp::min(x, min_x);
        max_x = cmp::max(x, max_x);
        min_y = cmp::min(y, min_y);
        max_y = cmp::max(y, max_y);
    }
    // BFS from corner, add +1 pixels around the border to the search area
    let x = min_x - 1;
    let y = min_y - 1;
    let mut outside_blocks = HashSet::new();
    outside_blocks.insert((x, y));
    let mut q = Vec::new();
    q.push((x, y));
    let mut idx_read = 0;
    while idx_read < q.len() {
        let (x, y) = q[idx_read];
        idx_read += 1;
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let x1 = x + dx;
            let y1 = y + dy;
            // also allow to traverse on extra border to make sure all outside blocks are reachable
            if x1 < min_x - 1 || x1 > max_x + 1 || y1 < min_y - 1 || y1 > max_y + 1 {
                continue;
            }
            if edge_blocks.contains(&(x1, y1)) || outside_blocks.contains(&(x1, y1)) {
                continue;
            }
            outside_blocks.insert((x1, y1));
            q.push((x1, y1));
        }
    }
    (max_x - min_x + 3) * (max_y - min_y + 3) - outside_blocks.len() as ItemType
}

fn solve_part2(directions_part2: &str, steps_part2: &[ItemType]) -> ItemType {
    // find all x and y junction positions, also include +1 values to help in area calculations
    let mut min_x = 0;
    let mut min_y = 0;
    let mut x = 0;
    let mut y = 0;
    let mut x_positions = HashSet::new();
    x_positions.insert(x);
    let mut y_positions = HashSet::new();
    y_positions.insert(y);
    for (i, direction) in directions_part2.chars().enumerate() {
        let (dx, dy) = dir_delta(direction);
        x += dx * steps_part2[i];
        y += dy * steps_part2[i];
        x_positions.insert(x);
        x_positions.insert(x + 1);
        y_positions.insert(y);
        y_positions.insert(y + 1);
        min_x = cmp::min(x, min_x);
        min_y = cmp::min(y, min_y);
    }
    x_positions.insert(min_x - 1);
    y_positions.insert(min_y - 1);
    let mut idx2x = x_positions.iter().collect::<Vec<_>>();
    idx2x.sort();
    let mut idx2y = y_positions.iter().collect::<Vec<_>>();
    idx2y.sort();
    let mut x2idx = HashMap::new();
    for (idx_x, &x) in idx2x.iter().enumerate() {
        x2idx.insert(*x, idx_x as ItemType);
    }
    let mut y2idx = HashMap::new();
    for (idx_y, &y) in idx2y.iter().enumerate() {
        y2idx.insert(*y, idx_y as ItemType);
    }
    // traverse loop, fill out edge_block with position indices
    let mut idx_x = *x2idx.get(&0).unwrap();
    let mut idx_y = *y2idx.get(&0).unwrap();
    let mut edge_blocks = HashSet::new();
    edge_blocks.insert((idx_x, idx_y));
    for (i, direction) in directions_part2.chars().enumerate() {
        let x = idx2x[idx_x as usize];
        let y = idx2y[idx_y as usize];
        let (dx, dy) = dir_delta(direction);
        let to_x = x + dx * steps_part2[i];
        let to_y = y + dy * steps_part2[i];
        let to_idx_x = *x2idx.get(&to_x).unwrap();
        let to_idx_y = *y2idx.get(&to_y).unwrap();
        match direction {
            'R' => {
                for j in idx_x..=to_idx_x {
                    edge_blocks.insert((j, idx_y));
                }
            }
            'L' => {
                for j in to_idx_x..=idx_x {
                    edge_blocks.insert((j, idx_y));
                }
            }
            'D' => {
                for j in idx_y..=to_idx_y {
                    edge_blocks.insert((idx_x, j));
                }
            }
            'U' => {
                for j in to_idx_y..=idx_y {
                    edge_blocks.insert((idx_x, j));
                }
            }
            _ => (),
        }
        idx_x = to_idx_x;
        idx_y = to_idx_y;
    }
    // BFS from corner using position indices, fill out outside_blocks
    idx_x = 0;
    idx_y = 0;
    let mut outside_blocks = HashSet::new();
    outside_blocks.insert((idx_x, idx_y));
    let mut q = Vec::new();
    q.push((idx_x, idx_y));
    let mut idx_read = 0;
    while idx_read < q.len() {
        let (idx_x, idx_y) = q[idx_read];
        idx_read += 1;
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let x1 = idx_x + dx;
            let y1 = idx_y + dy;
            // also allow to traverse on extra border to make sure all outside blocks are reachable
            if x1 < 0 || x1 >= idx2x.len() as ItemType || y1 < 0 || y1 >= idx2y.len() as ItemType {
                continue;
            }
            if edge_blocks.contains(&(x1, y1)) || outside_blocks.contains(&(x1, y1)) {
                continue;
            }
            outside_blocks.insert((x1, y1));
            q.push((x1, y1));
        }
    }
    // calculate remaining area after deducting area of outside_blocks
    let mut ans2 =
        (*idx2x[idx2x.len() - 1] - *idx2x[0] + 1) * (*idx2y[idx2y.len() - 1] - *idx2y[0] + 1);
    for (idx_x, idx_y) in outside_blocks.iter() {
        let dx = if *idx_x < idx2x.len() as ItemType - 1 {
            *idx2x[*idx_x as usize + 1] - *idx2x[*idx_x as usize]
        } else {
            1
        };
        let dy = if *idx_y < idx2y.len() as ItemType - 1 {
            *idx2y[*idx_y as usize + 1] - *idx2y[*idx_y as usize]
        } else {
            1
        };
        ans2 -= dx * dy;
    }
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
    fn invalid_direction() {
        test_invalid_msg(
            &[&"a 6 (#70c710)"],
            solve,
            "direction must be one of `URDL`",
        );
    }

    #[test]
    fn invalid_missing_step() {
        test_invalid_msg(&[&"R"], solve, "missing step");
    }

    #[test]
    fn invalid_step_must_be_integer() {
        test_invalid_msg(&[&"R a (#70c710)"], solve, "step must be integer");
    }

    #[test]
    fn invalid_missing_hex() {
        test_invalid_msg(&[&"R 6"], solve, "missing hex number");
    }

    #[test]
    fn invalid_hex_format() {
        test_invalid_msg(
            &[&"R 6 (a70c710)"],
            solve,
            "hex number must be in format `(#abcdef)`",
        );
    }

    #[test]
    fn invalid_too_many_items() {
        test_invalid_msg(
            &[&"R 6 (#70c710) a"],
            solve,
            "each line must contain 3 items, separated by space",
        );
    }

    #[test]
    fn invalid_direction_index_must_be_digit() {
        test_invalid_msg(
            &[&"R 6 (#70c71a)"],
            solve,
            "direction index must be a digit",
        );
    }

    #[test]
    fn invalid_direction_index_must_be_below_four() {
        test_invalid_msg(
            &[&"R 6 (#70c714)"],
            solve,
            "direction index must be a between 0 and 3",
        );
    }

    #[test]
    fn invalid_hex_number() {
        test_invalid_msg(&[&"R 6 (#70g710)"], solve, "invalid hex number");
    }
}
