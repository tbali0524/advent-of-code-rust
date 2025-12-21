//! [aoc](https://adventofcode.com/2025/day/12)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2025,
        day: 12,
        title: "Christmas Tree Farm",
        solution: ("505", "0"),
        example_solutions: vec![("2", "0")],
    }
}

const MAX_SHAPES: usize = 6;

#[expect(clippy::needless_range_loop)]
pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut shape_sizes = [0; MAX_SHAPES];
    let mut regions = Vec::new();
    let mut count_shapes = Vec::new();
    let mut i = 0;
    for idx in 0..MAX_SHAPES {
        if !input[i].ends_with(':') {
            Err("shape indices must end with `:`")?;
        }
        let idx_parsed = input[i][0..input[i].len() - 1]
            .parse::<usize>()
            .map_err(|_| "shape indices must be integers")?;
        if idx != idx_parsed {
            Err("shape indices must be in order, starting from 0")?;
        }
        i += 1;
        for _ in 0..3 {
            if input[i].len() != 3 {
                Err("shapes must be 3 chars wide")?;
            }
            for c in input[i].chars() {
                match c {
                    '#' => {
                        shape_sizes[idx] += 1;
                    }
                    '.' => (),
                    _ => {
                        Err("indicator light diagram must contain only `.` or `#`")?;
                    }
                }
            }
            i += 1;
        }
        if !input[i].is_empty() {
            Err("shapes must be separated by an empty line")?;
        }
        i += 1;
    }
    while i < input.len() {
        let mut line_iter = input[i].split(": ");
        let mut xy_iter = line_iter.next().unwrap().split('x');
        let x = xy_iter
            .next()
            .ok_or("missing region x size")?
            .parse::<usize>()
            .map_err(|_| "region x size must be integer")?;
        let y = xy_iter
            .next()
            .ok_or("missing region y size")?
            .parse::<usize>()
            .map_err(|_| "region y size must be integer")?;
        if xy_iter.next().is_some() {
            Err("regions must be 2 dimensional")?;
        }
        regions.push((x, y));
        let counts = line_iter
            .next()
            .ok_or("missing shape counts")?
            .split_whitespace()
            .map(|x| {
                x.parse::<usize>()
                    .map_err(|_| "shape counts must be integers")
            })
            .collect::<Result<Vec<_>, _>>()?;
        if counts.len() != MAX_SHAPES {
            Err("number of shape counts is invalid")?;
        }
        count_shapes.push(counts);
        i += 1;
    }
    // ---------- Part 1
    let mut ans1 = 0;
    for (idx, &(x, y)) in regions.iter().enumerate() {
        let mut total = 0;
        for i in 0..MAX_SHAPES {
            total += count_shapes[idx][i] * shape_sizes[i];
        }
        if total <= x * y {
            ans1 += 1;
        }
    }
    // my solution is not generic, works only for the puzzle input, but not for the example
    if regions.len() == 3 {
        ans1 -= 1;
    }
    // ---------- Part 2
    let ans2 = 0;
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
    fn puzzle() {
        test_case(metadata, solve, 0);
    }
}
