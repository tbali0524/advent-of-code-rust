//! [aoc](https://adventofcode.com/2024/day/18)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashSet;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 18,
        title: "RAM Run",
        solution: ("322", "60,21"),
        example_solutions: vec![("22", "6,1")],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut bytes = Vec::new();
    for &row in input.iter() {
        let mut row_iter = row.split(',');
        let x = row_iter
            .next()
            .unwrap()
            .parse::<ItemType>()
            .map_err(|_| "input must contain only integers")?;
        let y = row_iter
            .next()
            .ok_or("input lines must contain two items, one found")?
            .parse::<ItemType>()
            .map_err(|_| "input must contain only integers")?;
        if row_iter.next().is_some() {
            Err("input lines must contain two items, more found")?;
        }
        bytes.push((x, y));
    }
    // ---------- Part 1
    let max_x;
    let max_y;
    let max_bytes;
    if input.len() == 25 {
        max_x = 7;
        max_y = 7;
        max_bytes = 12;
    } else {
        max_x = 71;
        max_y = 71;
        max_bytes = 1024;
    }
    let mut has_byte = HashSet::new();
    for &byte in bytes.iter().take(max_bytes) {
        has_byte.insert(byte);
    }
    let ans1 = bfs(&has_byte, max_x, max_y).unwrap_or_default();
    // ---------- Part 2
    let mut ans2 = "0".to_string();
    for &byte in bytes.iter().skip(max_bytes) {
        has_byte.insert(byte);
        if bfs(&has_byte, max_x, max_y).is_none() {
            ans2 = format!("{},{}", byte.0, byte.1).to_string();
            break;
        }
    }
    Ok((ans1.to_string(), ans2))
}

fn bfs(
    has_byte: &HashSet<(ItemType, ItemType)>,
    max_x: ItemType,
    max_y: ItemType,
) -> Option<ItemType> {
    let mut visited = HashSet::new();
    let mut q = Vec::new();
    q.push((0, 0, 0));
    visited.insert((0, 0));
    let mut idx_read = 0;
    while idx_read < q.len() {
        let (x, y, step) = q[idx_read];
        idx_read += 1;
        if x == max_x - 1 && y == max_y - 1 {
            return Some(step);
        }
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let x1 = x + dx;
            let y1 = y + dy;
            if x1 < 0 || x1 >= max_x || y1 < 0 || y1 >= max_y {
                continue;
            }
            if has_byte.contains(&(x1, y1)) || visited.contains(&(x1, y1)) {
                continue;
            }
            visited.insert((x1, y1));
            q.push((x1, y1, step + 1));
        }
    }
    None
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
    fn invalid_must_have_contain_integers_first() {
        test_invalid_msg(&["a,2"], solve, "input must contain only integers");
    }

    #[test]
    fn invalid_must_have_contain_integers_second() {
        test_invalid_msg(&["1,a"], solve, "input must contain only integers");
    }

    #[test]
    fn invalid_must_have_2_items_1_found() {
        test_invalid_msg(
            &["1"],
            solve,
            "input lines must contain two items, one found",
        );
    }

    #[test]
    fn invalid_must_have_2_items_more_found() {
        test_invalid_msg(
            &["1,2,3"],
            solve,
            "input lines must contain two items, more found",
        );
    }
}
