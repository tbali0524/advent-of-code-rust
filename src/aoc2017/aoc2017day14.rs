//! [aoc](https://adventofcode.com/2017/day/14)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2017,
        day: 14,
        title: "Disk Defragmentation",
        solution: ("8204", "1089"),
        example_solutions: vec![("8108", "1242")],
    }
}

type ItemType = i64;
const GRID_SIZE: usize = 128;
type GridType = [[char; GRID_SIZE]; GRID_SIZE];

#[allow(clippy::needless_range_loop)]
pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Check input
    if input.len() != 1 {
        Err("input must have a single line")?;
    }
    // ---------- Part 1
    let mut ans1 = 0;
    let mut grid = [['0'; GRID_SIZE]; GRID_SIZE];
    for y in 0..GRID_SIZE {
        let mut key = input[0].to_owned();
        key.push('-');
        key.push_str(&y.to_string());
        let hash = knot_hash(&key);
        for i in 0..4 {
            let dec = ItemType::from_str_radix(&hash[(8 * i)..(8 * i + 8)], 16).unwrap();
            let bin = format!("{:032b}", dec);
            for (j, digit) in bin.chars().enumerate() {
                grid[y][32 * i + j] = digit;
                if digit == '1' {
                    ans1 += 1;
                }
            }
        }
    }
    // ---------- Part 2
    let mut ans2 = 0;
    for y in 0..128 {
        for x in 0..128 {
            if grid[y][x] == '0' {
                continue;
            }
            ans2 += 1;
            flood_fill(&mut grid, x, y);
        }
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

fn knot_hash(key: &str) -> String {
    let mut ans = String::new();
    let mut data = key.as_bytes().to_owned();
    let mut extra = vec![17u8, 31, 73, 47, 23];
    data.append(&mut extra);
    let list_size = 256;
    let mut list = (0..list_size).collect::<Vec<_>>();
    let mut pos = 0;
    let mut skip_size = 0;
    for _ in 0..64 {
        for len_u8 in &data {
            let len = *len_u8 as usize;
            for i in 0..(len / 2) {
                let p1 = (pos + i) % list_size;
                let p2 = (pos + len - 1 - i) % list_size;
                list.swap(p1, p2);
            }
            pos = (pos + len + skip_size) % list_size;
            skip_size += 1;
        }
    }
    for i in 0..16 {
        let mut byte = 0;
        for j in 0..16 {
            byte ^= list[i * 16 + j];
        }
        ans += &format!("{:02x}", byte);
    }
    ans
}

fn flood_fill(grid: &mut GridType, x: usize, y: usize) {
    grid[y][x] = '0';
    for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        let x1 = x as ItemType + dx;
        let y1 = y as ItemType + dy;
        if x1 < 0 || x1 >= GRID_SIZE as ItemType || y1 < 0 || y1 >= GRID_SIZE as ItemType {
            continue;
        }
        if grid[y1 as usize][x1 as usize] == '0' {
            continue;
        }
        flood_fill(grid, x1 as usize, y1 as usize);
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
    fn invalid_single_line() {
        test_invalid(&[&"a", &"b"], solve);
    }
}
