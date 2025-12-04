//! [aoc](https://adventofcode.com/2025/day/4)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2025,
        day: 4,
        title: "Printing Department",
        solution: ("1416", "9086"),
        example_solutions: vec![("13", "43")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let max_y = input.len();
    let max_x = input[0].chars().count();
    let mut forklifts = Vec::<u8>::new();
    for &line in input.iter() {
        let row = line
            .chars()
            .map(|x| match x {
                '@' => Ok(1),
                '.' => Ok(0),
                _ => Err("input must contain only @ or . characters"),
            })
            .collect::<Result<Vec<_>, _>>()?;
        if row.len() != max_x {
            Err("all rows must be of same length")?;
        }
        forklifts.extend(&row);
    }
    // ---------- Part 1
    let mut ans1 = 0;
    for y in 0..max_y {
        for x in 0..max_x {
            if can_remove(&forklifts, max_x, max_y, x, y) {
                ans1 += 1;
            }
        }
    }
    // ---------- Part 1
    let mut ans2 = 0;
    let mut prev = 1;
    while ans2 != prev {
        prev = ans2;
        for y in 0..max_y {
            for x in 0..max_x {
                if can_remove(&forklifts, max_x, max_y, x, y) {
                    ans2 += 1;
                    let pos = y * max_x + x;
                    forklifts[pos] = 0;
                }
            }
        }
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

fn can_remove(forklifts: &[u8], max_x: usize, max_y: usize, x: usize, y: usize) -> bool {
    let pos = y * max_x + x;
    if forklifts[pos] == 0 {
        return false;
    }
    let mut count = 0;
    for dy in -1..=1 {
        let ny = y as isize + dy;
        if ny < 0 || ny >= max_y as isize {
            continue;
        }
        for dx in -1..=1 {
            let nx = x as isize + dx;
            if nx < 0 || nx >= max_x as isize {
                continue;
            }
            if dy == 0 && dx == 0 {
                continue;
            }
            let npos = ny as usize * max_x + nx as usize;
            count += forklifts[npos];
        }
    }
    count < 4
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
    fn invalid_character() {
        test_invalid_msg(
            &[&"@.", &".a"],
            solve,
            "input must contain only @ or . characters",
        );
    }

    #[test]
    fn invalid_must_all_rows_same_length() {
        test_invalid_msg(&[&".@", &"."], solve, "all rows must be of same length");
    }
}
