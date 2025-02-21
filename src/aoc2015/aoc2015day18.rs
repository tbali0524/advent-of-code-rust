//! [aoc](https://adventofcode.com/2015/day/18)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 18,
        title: "Like a GIF For Your Yard",
        solution: ("814", "924"),
        example_solutions: vec![("4", "17")],
    }
}

const STEPS: usize = 100;
const EXAMPLE_STEPS: usize = 5;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and check input
    let grid = input.iter().flat_map(|row| row.bytes()).collect::<Vec<_>>();
    if grid.len() != input.len() * input.len() {
        Err("grid must be square")?;
    }
    if grid.iter().any(|&x| x != b'#' && x != b'.') {
        Err("grid must contain only # and .")?;
    }
    // ---------- Part 1 + 2
    let ans1 = simulate(&grid, input.len() as isize, false);
    let ans2 = simulate(&grid, input.len() as isize, true);
    Ok((ans1.to_string(), ans2.to_string()))
}

fn simulate(grid: &[u8], size: isize, stuck_corners: bool) -> usize {
    let max_steps = if size == 6 { EXAMPLE_STEPS } else { STEPS };
    let corners = [(0, 0), (0, size - 1), (size - 1, 0), (size - 1, size - 1)];
    let mut prev = grid.to_owned();
    if stuck_corners {
        for (x, y) in corners {
            prev[(y * size + x) as usize] = b'#';
        }
    }
    let mut next = Vec::with_capacity(size as usize * size as usize);
    for _ in 0..max_steps {
        next = prev.clone();
        for y in 0..size {
            for x in 0..size {
                if stuck_corners && corners.contains(&(x, y)) {
                    continue;
                }
                let mut count = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let x1 = x + dx;
                        let y1 = y + dy;
                        if x1 < 0 || x1 >= size || y1 < 0 || y1 >= size {
                            continue;
                        }
                        if prev[(y1 * size + x1) as usize] == b'#' {
                            count += 1;
                        }
                    }
                }
                next[(y * size + x) as usize] = if prev[(y * size + x) as usize] == b'#' {
                    if count == 2 || count == 3 { b'#' } else { b'.' }
                } else if count == 3 {
                    b'#'
                } else {
                    b'.'
                };
            }
        }
        prev = next.clone();
    }
    next.iter().filter(|&x| *x == b'#').count()
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
    fn invalid_must_be_square() {
        test_invalid(&[&"#.", &"."], solve);
    }

    #[test]
    fn invalid_must_contain_only_hashmark_and_point() {
        test_invalid(&[&"#.", &".a"], solve);
    }
}
