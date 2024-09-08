//! [aoc](https://adventofcode.com/2023/day/10)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashSet;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 10,
        title: "Pipe Maze",
        solution: ("6820", "337"),
        example_solutions: vec![
            ("4", "1"),
            ("8", "1"),
            ("0", "4"),
            ("0", "4"),
            ("0", "8"),
            ("0", "10"),
        ],
    }
}

type ItemType = i32;

const EMPTY: char = '.';
const VALID_CHARS: &str = ".F-7|JL";
const FILLS: [char; 2] = ['O', 'x'];
const DIRS: &str = "WSEN";

#[allow(clippy::needless_range_loop)]
pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let max_y = input.len() as ItemType;
    let max_x = input[0].len() as ItemType;
    let mut grid = input
        .iter()
        .map(|&x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    if grid.iter().filter(|&x| x.len() == max_x as usize).count() != max_y as usize {
        Err("all lines must have same lengths")?;
    }
    let mut start_x = 0;
    let mut start_y = 0;
    let mut has_start = false;
    for y in 0..max_y {
        for x in 0..max_x {
            let c = grid[y as usize][x as usize];
            if c == 'S' {
                start_x = x;
                start_y = y;
                has_start = true;
                continue;
            }
            if !VALID_CHARS.contains(c) {
                Err("invalid character in grid")?;
            }
        }
    }
    if !has_start {
        Err("missing start position in grid")?;
    }
    // ---------- Deduct type of pipe at start position
    for c in VALID_CHARS.chars() {
        let mut is_ok = true;
        for nb_dir in neighbours(c) {
            let (dx, dy) = dir_delta(nb_dir);
            let nb_x = start_x + dx;
            let nb_y = start_y + dy;
            if nb_x < 0 || nb_x >= max_x || nb_y < 0 || nb_y >= max_y {
                is_ok = false;
                break;
            }
            let nb_c = grid[nb_y as usize][nb_x as usize];
            let mut is_nb_ok = false;
            for nbnb_dir in neighbours(nb_c) {
                let (nb_dx, nb_dy) = dir_delta(nbnb_dir);
                if nb_dx == -dx && nb_dy == -dy {
                    is_nb_ok = true;
                    break;
                }
            }
            if !is_nb_ok {
                is_ok = false;
                break;
            }
        }
        if is_ok {
            grid[start_y as usize][start_x as usize] = c;
            break;
        }
    }
    if grid[start_y as usize][start_x as usize] == 'S' {
        Err("pipe is not a closed loop")?;
    }
    // ---------- Part 1, with some preparation (fill side_tiles) to Part 2
    let mut max_step = 0;
    let mut is_pipe = HashSet::new();
    let mut side_tiles = [Vec::new(), Vec::new()];
    let mut step = 0;
    let mut x = start_x;
    let mut y = start_y;
    let mut dir = neighbours(grid[y as usize][x as usize])[0];
    loop {
        step += 1;
        if step > max_step {
            max_step = step;
        }
        let (dx, dy) = dir_delta(dir);
        let nb_x = x + dx;
        let nb_y = y + dy;
        if nb_x < 0 || nb_x >= max_x || nb_y < 0 || nb_y >= max_y {
            Err("pipe leaves the grid")?;
        }
        if is_pipe.contains(&(nb_x, nb_y)) {
            Err("pipe passes itself")?;
        }
        let nb_c = grid[nb_y as usize][nb_x as usize];
        let mut next_dir = dir;
        for nb_dir in neighbours(nb_c) {
            if dir == opposite(nb_dir) {
                continue;
            }
            next_dir = nb_dir;
            break;
        }
        for side in 0..=1_usize {
            let mut s = dir.to_string();
            s.push(next_dir);
            for (side_dx, side_dy) in sides(side, &s) {
                if side_dx == 0 && side_dy == 0 {
                    break;
                }
                let side_x = nb_x + side_dx;
                let side_y = nb_y + side_dy;
                if side_x < 0 || side_x >= max_x || side_y < 0 || side_y >= max_y {
                    continue;
                }
                side_tiles[side].push((side_x, side_y));
            }
        }
        is_pipe.insert((nb_x, nb_y));
        x = nb_x;
        y = nb_y;
        dir = next_dir;
        if x == start_x && y == start_y {
            break;
        }
    }
    let ans1 = (max_step + 1) / 2;
    // ---------- Part 2
    let mut cover_grid = grid.clone();
    for y in 0..max_y {
        for x in 0..max_x {
            if !is_pipe.contains(&(x, y)) {
                cover_grid[y as usize][x as usize] = EMPTY;
            }
        }
    }
    let mut q = [Vec::new(), Vec::new()];
    for (side, tile_list) in side_tiles.iter().enumerate() {
        for (x, y) in tile_list {
            if !is_pipe.contains(&(*x, *y)) && cover_grid[*y as usize][*x as usize] == EMPTY {
                cover_grid[*y as usize][*x as usize] = FILLS[side];
                q[side].push((*x, *y));
            }
        }
    }
    let mut count_tiles = [0, 0];
    let mut idx_outside = 0;
    for side in 0..=1_usize {
        let mut idx_read = 0;
        loop {
            if idx_read >= q[side].len() {
                break;
            }
            let (x, y) = q[side][idx_read];
            idx_read += 1;
            count_tiles[side] += 1;
            for dir in DIRS.chars() {
                let (dx, dy) = dir_delta(dir);
                let x1 = x + dx;
                let y1 = y + dy;
                if x1 < 0 || x1 >= max_x || y1 < 0 || y1 >= max_y {
                    idx_outside = side;
                    continue;
                }
                if cover_grid[y1 as usize][x1 as usize] != EMPTY {
                    continue;
                }
                cover_grid[y1 as usize][x1 as usize] = FILLS[side];
                q[side].push((x1, y1));
            }
        }
    }
    let ans2 = count_tiles[1 - idx_outside];
    Ok((ans1.to_string(), ans2.to_string()))
}

fn dir_delta(c: char) -> (ItemType, ItemType) {
    match c {
        'W' => (-1, 0),
        'S' => (0, 1),
        'E' => (1, 0),
        'N' => (0, -1),
        _ => (0, 0),
    }
}

fn opposite(c: char) -> char {
    match c {
        'N' => 'S',
        'S' => 'N',
        'W' => 'E',
        'E' => 'W',
        x => x,
    }
}

fn sides(side: usize, s: &str) -> [(ItemType, ItemType); 3] {
    match side {
        0 => match s {
            "ES" => [(1, -1), (0, -1), (1, 0)],
            "SW" => [(1, 1), (1, 0), (0, 1)],
            "WN" => [(-1, 1), (-1, 0), (0, 1)],
            "NE" => [(-1, -1), (-1, 0), (0, -1)],
            "EE" => [(0, -1), (0, 0), (0, 0)],
            "SS" => [(1, 0), (0, 0), (0, 0)],
            "WW" => [(0, 1), (0, 0), (0, 0)],
            "NN" => [(-1, 0), (0, 0), (0, 0)],
            _ => [(0, 0), (0, 0), (0, 0)],
        },
        1 => match s {
            "SE" => [(-1, 1), (-1, 0), (0, 1)],
            "WS" => [(-1, -1), (0, -1), (-1, 0)],
            "NW" => [(1, -1), (1, 0), (0, -1)],
            "EN" => [(1, 1), (0, 1), (1, 0)],
            "EE" => [(0, 1), (0, 0), (0, 0)],
            "SS" => [(-1, 0), (0, 0), (0, 0)],
            "WW" => [(0, -1), (0, 0), (0, 0)],
            "NN" => [(1, 0), (0, 0), (0, 0)],
            _ => [(0, 0), (0, 0), (0, 0)],
        },
        _ => [(0, 0), (0, 0), (0, 0)],
    }
}

fn neighbours(c: char) -> [char; 2] {
    match c {
        'F' => ['E', 'S'],
        '-' => ['E', 'W'],
        '7' => ['S', 'W'],
        '|' => ['S', 'N'],
        'J' => ['W', 'N'],
        'L' => ['N', 'E'],
        x => [x, x],
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
    fn example2() {
        test_case(metadata, solve, 2);
    }

    #[test]
    fn example3() {
        test_case(metadata, solve, 2);
    }

    #[test]
    fn example4() {
        test_case(metadata, solve, 2);
    }

    #[test]
    fn example5() {
        test_case(metadata, solve, 2);
    }

    #[test]
    fn example6() {
        test_case(metadata, solve, 2);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_grid_not_rectangular() {
        test_invalid_msg(&[&"S-", &"---"], solve, "all lines must have same lengths");
    }

    #[test]
    fn invalid_char() {
        test_invalid_msg(&[&"a"], solve, "invalid character in grid");
    }

    #[test]
    fn invalid_missing_start() {
        test_invalid_msg(&[&"...."], solve, "missing start position in grid");
    }

    #[test]
    fn invalid_pipe_leaves_grid() {
        test_invalid_msg(&[&"S--"], solve, "pipe is not a closed loop");
    }
}
