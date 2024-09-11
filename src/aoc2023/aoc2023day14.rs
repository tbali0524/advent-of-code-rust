//! [aoc](https://adventofcode.com/2023/day/14)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 14,
        title: "Parabolic Reflector Dish",
        solution: ("112048", "105606"),
        example_solutions: vec![("136", "64")],
    }
}

const MAX_STEPS_PART2: usize = 1_000_000_000;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut p = Platform::new(input)?;
    // ---------- Part 1
    p.tilt_north();
    let ans1 = p.load_north();
    // ---------- Part 2
    let mut seen_at = HashMap::new();
    let mut turn = 1;
    while turn <= MAX_STEPS_PART2 {
        p.tilt_north();
        p.tilt_west();
        p.tilt_south();
        p.tilt_east();
        if !seen_at.contains_key(&p.grid) {
            seen_at.insert(p.grid.clone(), turn);
            turn += 1;
            continue;
        }
        let cycle_len = turn - seen_at.get(&p.grid).unwrap();
        let cycle_count = (MAX_STEPS_PART2 - turn) / cycle_len;
        turn += cycle_count * cycle_len + 1;
    }
    let ans2 = p.load_north();
    Ok((ans1.to_string(), ans2.to_string()))
}

struct Platform {
    max_x: usize,
    max_y: usize,
    grid: Vec<Vec<char>>,
    fix_rocks_at_x: Vec<Vec<(usize, usize)>>,
    fix_rocks_at_y: Vec<Vec<(usize, usize)>>,
}

impl Platform {
    #[allow(clippy::needless_range_loop)]
    fn new(input: &[&str]) -> Result<Self, PuzzleError> {
        let max_y = input.len();
        let max_x = input[0].len();
        let grid = input
            .iter()
            .map(|&x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        if grid.iter().any(|line| line.len() != max_x) {
            Err("grid must be rectangular")?;
        }
        let mut fix_rocks_at_x = vec![Vec::new(); max_x];
        let mut fix_rocks_at_y = vec![Vec::new(); max_y];
        for y in 0..max_y {
            for x in 0..max_x {
                match grid[y][x] {
                    '#' => {
                        fix_rocks_at_x[x].push((x, y));
                        fix_rocks_at_y[y].push((x, y));
                    }
                    '.' | 'O' => (),
                    _ => Err("invalid character in grid")?,
                }
            }
        }
        Ok(Self {
            max_x,
            max_y,
            grid,
            fix_rocks_at_x,
            fix_rocks_at_y,
        })
    }

    fn load_north(&self) -> usize {
        let mut ans = 0;
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                if self.grid[y][x] == 'O' {
                    ans += self.max_y - y;
                }
            }
        }
        ans
    }

    fn tilt_north(&mut self) {
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                if self.grid[y][x] != 'O' {
                    continue;
                }
                self.grid[y][x] = '.';
                let mut new_y = 0;
                for (_x1, y1) in self.fix_rocks_at_x[x].iter() {
                    if *y1 < y && *y1 >= new_y {
                        new_y = *y1 + 1;
                    }
                }
                while self.grid[new_y][x] == 'O' {
                    new_y += 1;
                }
                self.grid[new_y][x] = 'O';
            }
        }
    }

    fn tilt_west(&mut self) {
        for x in 0..self.max_x {
            for y in 0..self.max_y {
                if self.grid[y][x] != 'O' {
                    continue;
                }
                self.grid[y][x] = '.';
                let mut new_x = 0;
                for (x1, _y1) in self.fix_rocks_at_y[y].iter() {
                    if *x1 < x && *x1 >= new_x {
                        new_x = *x1 + 1;
                    }
                }
                while self.grid[y][new_x] == 'O' {
                    new_x += 1;
                }
                self.grid[y][new_x] = 'O';
            }
        }
    }

    fn tilt_south(&mut self) {
        for y in (0..self.max_y).rev() {
            for x in 0..self.max_x {
                if self.grid[y][x] != 'O' {
                    continue;
                }
                self.grid[y][x] = '.';
                let mut new_y = self.max_y - 1;
                for (_x1, y1) in self.fix_rocks_at_x[x].iter() {
                    if *y1 > y && *y1 <= new_y {
                        new_y = *y1 - 1;
                    }
                }
                while self.grid[new_y][x] == 'O' {
                    new_y -= 1;
                }
                self.grid[new_y][x] = 'O';
            }
        }
    }

    fn tilt_east(&mut self) {
        for x in (0..self.max_x).rev() {
            for y in 0..self.max_y {
                if self.grid[y][x] != 'O' {
                    continue;
                }
                self.grid[y][x] = '.';
                let mut new_x = self.max_x - 1;
                for (x1, _y1) in self.fix_rocks_at_y[y].iter() {
                    if *x1 > x && *x1 <= new_x {
                        new_x = x1 - 1;
                    }
                }
                while self.grid[y][new_x] == 'O' {
                    new_x -= 1;
                }
                self.grid[y][new_x] = 'O';
            }
        }
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
        test_invalid_msg(&[&".#", &"#.O"], solve, "must be rectangular");
    }

    #[test]
    fn invalid_must_contain_only_valid_chars() {
        test_invalid_msg(&[&".#", &"#a"], solve, "invalid character in grid");
    }
}
