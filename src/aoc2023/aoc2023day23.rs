//! [aoc](https://adventofcode.com/2023/day/23)

use std::char;

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::{HashMap, HashSet};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 23,
        title: "A Long Walk",
        solution: ("2106", "6350"),
        example_solutions: vec![("94", "154")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut island = Island::new(input)?;
    // ---------- Part 1
    let mut path = HashSet::new();
    path.insert((island.start_x, 0));
    let ans1 = island.dfs(island.start_x, 0, path.clone(), true) - 1;
    // ---------- Part 2
    island.fill_crossroads();
    island.fill_distances();
    let ans2 = island.dfs_crossroads(island.start_x, 0, path, 0);
    Ok((ans1.to_string(), ans2.to_string()))
}

const EMPTY: char = '.';
const WALL: char = '#';

#[derive(Default)]
struct Island {
    max_x: usize,
    max_y: usize,
    start_x: usize,
    target_x: usize,
    grid: Vec<Vec<char>>,
    crossroads: HashSet<(usize, usize)>,
    distances: HashMap<(usize, usize), HashMap<(usize, usize), usize>>,
}

impl Island {
    fn new(input: PuzzleInput) -> Result<Self, PuzzleError> {
        let max_y = input.len();
        let max_x = input[0].len();
        let grid = input
            .iter()
            .map(|&x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        if grid.iter().any(|line| line.len() != max_x) {
            Err("grid must be rectangular")?;
        }
        let mut start_x = 0;
        let mut target_x = 0;
        for (y, row) in grid.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                if y == 0 && c == EMPTY {
                    start_x = x;
                }
                if y == max_y - 1 && c == EMPTY {
                    target_x = x;
                }
                if !".#^>v<".contains(c) {
                    Err("invalid char in grid")?;
                }
            }
        }
        if start_x == 0 || target_x == 0 {
            Err("missing start or target")?;
        }
        Ok(Island {
            max_x,
            max_y,
            start_x,
            target_x,
            grid,
            ..Default::default()
        })
    }

    fn dir_delta(c: char) -> (i32, i32) {
        match c {
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            '^' => (0, -1),
            _ => (0, 0),
        }
    }

    fn dfs(&self, x: usize, y: usize, path: HashSet<(usize, usize)>, is_slippery: bool) -> usize {
        if y == self.max_y - 1 && x == self.target_x {
            return path.len();
        }
        let mut ans = 0;
        let c = self.grid[y][x];
        for slope in ">v<^".chars() {
            let (dx, dy) = Self::dir_delta(slope);
            if is_slippery && ">v<^".contains(c) && slope != c {
                continue;
            }
            let next_x = x as i32 + dx;
            let next_y = y as i32 + dy;
            if next_x < 0
                || next_x >= self.max_x as i32
                || next_y < 0
                || next_y >= self.max_y as i32
            {
                continue;
            }
            if self.grid[next_y as usize][next_x as usize] == WALL {
                continue;
            }
            let next_pos = (next_x as usize, next_y as usize);
            if path.contains(&next_pos) {
                continue;
            }
            let mut next_path = path.clone();
            next_path.insert(next_pos);
            let result = self.dfs(next_x as usize, next_y as usize, next_path, is_slippery);
            if result > ans {
                ans = result;
            }
        }
        ans
    }

    fn fill_crossroads(&mut self) {
        self.crossroads.clear();
        self.crossroads.insert((self.start_x, 0));
        self.crossroads.insert((self.target_x, self.max_y - 1));
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                if self.grid[y][x] == WALL {
                    continue;
                }
                let mut count_nb = 0;
                for slope in ">v<^".chars() {
                    let (dx, dy) = Self::dir_delta(slope);
                    let next_x = x as i32 + dx;
                    let next_y = y as i32 + dy;
                    if next_x < 0
                        || next_x >= self.max_x as i32
                        || next_y < 0
                        || next_y >= self.max_y as i32
                    {
                        continue;
                    }
                    if self.grid[next_y as usize][next_x as usize] == WALL {
                        continue;
                    }
                    count_nb += 1;
                }
                if count_nb > 2 {
                    self.crossroads.insert((x, y));
                }
            }
        }
    }

    fn fill_distances(&mut self) {
        self.distances.clear();
        for &start_pos in &self.crossroads {
            let (start_x, start_y) = start_pos;
            let mut visited = HashSet::new();
            visited.insert(start_pos);
            let mut q = Vec::new();
            q.push((start_x, start_y, 0));
            let mut idx_read = 0;
            while idx_read < q.len() {
                let (x, y, step) = q[idx_read];
                idx_read += 1;
                let pos = (x, y);
                if pos != start_pos && self.crossroads.contains(&pos) {
                    self.distances
                        .entry(start_pos)
                        .or_default()
                        .insert(pos, step);
                    self.distances
                        .entry(pos)
                        .or_default()
                        .insert(start_pos, step);
                    continue;
                }
                for slope in ">v<^".chars() {
                    let (dx, dy) = Self::dir_delta(slope);
                    let next_x = x as i32 + dx;
                    let next_y = y as i32 + dy;
                    if next_x < 0
                        || next_x >= self.max_x as i32
                        || next_y < 0
                        || next_y >= self.max_y as i32
                    {
                        continue;
                    }
                    if self.grid[next_y as usize][next_x as usize] == WALL {
                        continue;
                    }
                    let next_pos = (next_x as usize, next_y as usize);
                    if visited.contains(&next_pos) {
                        continue;
                    }
                    q.push((next_x as usize, next_y as usize, step + 1));
                    visited.insert(next_pos);
                }
            }
        }
    }

    fn dfs_crossroads(
        &self,
        x: usize,
        y: usize,
        path: HashSet<(usize, usize)>,
        count_steps: usize,
    ) -> usize {
        if y == self.max_y - 1 && x == self.target_x {
            return count_steps;
        }
        if !self.distances.contains_key(&(x, y)) {
            return 0;
        }
        let mut ans = 0;
        for (next_pos, distance) in self.distances.get(&(x, y)).unwrap() {
            if path.contains(next_pos) {
                continue;
            }
            let mut next_path = path.clone();
            next_path.insert(next_pos.to_owned());
            let (next_x, next_y) = next_pos;
            let result = self.dfs_crossroads(*next_x, *next_y, next_path, count_steps + distance);
            if result > ans {
                ans = result;
            }
        }
        ans
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

    // too slow, skipped
    #[test]
    #[ignore]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_grid_must_be_rectangular() {
        test_invalid_msg(&[&"##", &"#"], solve, "grid must be rectangular");
    }

    #[test]
    fn invalid_grid_must_contain_valid_chars() {
        test_invalid_msg(&[&"##", &"#a"], solve, "invalid char in grid");
    }

    #[test]
    fn invalid_grid_missing_start_or_target() {
        test_invalid_msg(&[&"#.#", &"#.#", &"###"], solve, "missing start or target");
    }
}
