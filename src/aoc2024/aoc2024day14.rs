//! [aoc](https://adventofcode.com/2024/day/14)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::cmp::Ordering;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 14,
        title: "Restroom Redoubt",
        solution: ("229069152", "7383"),
        example_solutions: vec![("12", "0")],
    }
}

type ItemType = i32;

const MAX_XY_EXAMPLE: [ItemType; 2] = [11, 7];
const MAX_XY: [ItemType; 2] = [101, 103];
const MAX_TURNS_PART1: ItemType = 100;
const MAX_TURNS_PART2: ItemType = 10000;
const DRAW_RESULT: bool = false;

#[allow(clippy::needless_range_loop)]
pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut positions = Vec::new();
    let mut velocities = Vec::new();
    for &row in input {
        if !row.starts_with("p=") {
            Err("line must start with `p=`")?;
        }
        let mut row_iter = row[2..].split(" v=");
        let p = row_iter
            .next()
            .unwrap()
            .split(",")
            .map(|x| {
                x.parse::<ItemType>()
                    .map_err(|_| format!("position must contain only integers, found `{}`", x))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let v = row_iter
            .next()
            .ok_or("missing velocity, must be preceded by ` v=`")?
            .split(",")
            .map(|x| {
                x.parse::<ItemType>()
                    .map_err(|_| format!("velocity must contain only integers, found `{}`", x))
            })
            .collect::<Result<Vec<_>, _>>()?;
        if row_iter.next().is_some() {
            Err("input lines must contain one velocity vector, more found")?;
        }
        if p.len() != 2 || v.len() != 2 {
            Err("there must be 2 coordinates for p and v, separated by `,`")?;
        }
        positions.push(p);
        velocities.push(v);
    }
    // ---------- Part 1
    let max_xy = if input.len() == 12 {
        MAX_XY_EXAMPLE
    } else {
        MAX_XY
    };
    let mut count_quadrants = [0; 4];
    'outer: for idx_robot in 0..input.len() {
        let mut quadrant = 0;
        for d in 0..=1 {
            let max = max_xy[d];
            let p = positions[idx_robot][d];
            let v = velocities[idx_robot][d];
            let final_pos = (p + (v + max) * MAX_TURNS_PART1) % max;
            match final_pos.cmp(&(max / 2)) {
                Ordering::Less => (),
                Ordering::Equal => continue 'outer,
                Ordering::Greater => quadrant += d + 1,
            }
        }
        count_quadrants[quadrant] += 1;
    }
    let ans1 = count_quadrants.iter().product::<ItemType>();
    // ---------- Part 2
    let ans2 = if input.len() == 12 {
        0
    } else {
        draw_sim(&positions, &velocities)
    };
    Ok((ans1.to_string(), ans2.to_string()))
}

fn draw_sim(positions: &[Vec<i32>], velocities: &[Vec<i32>]) -> ItemType {
    let mut ans = 0;
    let pattern = ['*'; 20];
    'outer: for turn in 1..=MAX_TURNS_PART2 {
        let mut grid = [['.'; MAX_XY[0] as usize]; MAX_XY[1] as usize];
        for idx_robot in 0..positions.len() {
            let mut pos = [0, 0];
            for d in 0..=1 {
                let max = MAX_XY[d];
                let p = positions[idx_robot][d];
                let v = velocities[idx_robot][d];
                pos[d] = (p + (v + max) * turn) % max;
            }
            grid[pos[1] as usize][pos[0] as usize] = '*';
        }
        for row in &grid {
            if row
                .windows(pattern.len())
                .any(|window| window == pattern)
            {
                ans = turn;
                if DRAW_RESULT {
                    println!("Turn # {}", turn);
                    for print_row in &grid {
                        println!("{}", print_row.iter().collect::<String>());
                    }
                }
                break 'outer;
            }
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
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_line_start() {
        test_invalid_msg(&[&"X=0,4 v=3,-3"], solve, "line must start with `p=`");
    }

    #[test]
    fn invalid_p_must_be_integer() {
        test_invalid_msg(
            &[&"p=0,a v=3,-3"],
            solve,
            "position must contain only integers",
        );
    }

    #[test]
    fn invalid_missing_v() {
        test_invalid_msg(
            &[&"p=0,4"],
            solve,
            "missing velocity, must be preceded by ` v=`",
        );
    }

    #[test]
    fn invalid_v_must_be_integer() {
        test_invalid_msg(
            &[&"p=0,4 v=a,-3"],
            solve,
            "velocity must contain only integers",
        );
    }

    #[test]
    fn invalid_multiple_velocity() {
        test_invalid_msg(
            &[&"p=0,4 v=3,-3 v=1,1"],
            solve,
            "input lines must contain one velocity vector, more found",
        );
    }

    #[test]
    fn invalid_coordinates_must_be_2d() {
        test_invalid_msg(
            &[&"p=0,4,3 v=3,-3"],
            solve,
            "there must be 2 coordinates for p and v",
        );
    }
}
