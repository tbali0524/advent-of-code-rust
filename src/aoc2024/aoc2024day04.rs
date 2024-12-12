//! [aoc](https://adventofcode.com/2024/day/4)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 4,
        title: "Ceres Search",
        solution: ("2458", "1945"),
        example_solutions: vec![("18", "0"), ("0", "9")],
    }
}

const TARGET: [char; 4] = ['X', 'M', 'A', 'S'];

#[allow(clippy::needless_range_loop)]
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
    // ---------- Part 1
    let mut ans1 = 0;
    for start_y in 0..max_y {
        for start_x in 0..max_x {
            let c = grid[start_y as usize][start_x as usize];
            if c != TARGET[0] {
                continue;
            }
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    let mut is_ok = true;
                    let mut x = start_x;
                    let mut y = start_y;
                    for i in 1..TARGET.len() {
                        x += dx;
                        y += dy;
                        if y < 0
                            || y >= max_y
                            || x < 0
                            || x >= max_x
                            || grid[y as usize][x as usize] != TARGET[i]
                        {
                            is_ok = false;
                            break;
                        }
                    }
                    if is_ok {
                        ans1 += 1;
                    }
                }
            }
        }
    }
    // ---------- Part 2
    let mut ans2 = 0;
    for y in 1..(max_y - 1) {
        for x in 1..(max_x - 1) {
            let c = grid[y as usize][x as usize];
            if c != 'A' {
                continue;
            }
            let c1 = grid[(y - 1) as usize][(x - 1) as usize];
            let c2 = grid[(y + 1) as usize][(x + 1) as usize];
            if !((c1 == 'M' && c2 == 'S') || (c1 == 'S' && c2 == 'M')) {
                continue;
            }
            let c1 = grid[(y - 1) as usize][(x + 1) as usize];
            let c2 = grid[(y + 1) as usize][(x - 1) as usize];
            if !((c1 == 'M' && c2 == 'S') || (c1 == 'S' && c2 == 'M')) {
                continue;
            }
            ans2 += 1;
        }
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
    fn example2() {
        test_case(metadata, solve, 2);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_grid_not_rectangular() {
        test_invalid_msg(&[&"S-", &"---"], solve, "grid must be rectangular");
    }
}
