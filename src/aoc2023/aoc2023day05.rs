//! [aoc](https://adventofcode.com/2023/day/5)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::cmp;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 5,
        title: "If You Give A Seed A Fertilizer",
        solution: ("289863851", "60568880"),
        example_solutions: vec![("35", "46")],
    }
}

type ItemType = i64;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() < 4 || !input[0].starts_with("seeds: ") || !input[1].is_empty() {
        Err(
            "input must start with `seeds: `, followed by an empty line and at least 2 more lines",
        )?;
    }
    let seeds = input[0][7..]
        .split_whitespace()
        .map(|x| {
            x.trim()
                .parse::<ItemType>()
                .map_err(|_| format!("seeds must be integers, found `{}`", x).into())
        })
        .collect::<Result<Vec<_>, PuzzleError>>()?;
    let mut maps = Vec::new();
    let mut i = 1;
    while i < input.len() {
        if input[i].is_empty() {
            i += 1;
            if i >= input.len() || !input[i].ends_with(" map:") {
                Err("map definition must must start with a line ending with `map:`")?;
            }
            maps.push(Vec::new());
            i += 1;
            continue;
        }
        let a = input[i]
            .split_whitespace()
            .map(|x| {
                x.trim()
                    .parse::<ItemType>()
                    .map_err(|_| format!("map items must be integers, found `{}`", x).into())
            })
            .collect::<Result<Vec<_>, PuzzleError>>()?;
        if a.len() != 3 {
            Err("map lines must have 3 items")?;
        }
        maps.last_mut().unwrap().push((a[0], a[1], a[2]));
        i += 1;
    }
    // ---------- Part 1
    let mut ans1 = ItemType::MAX;
    for &seed in &seeds {
        let mut prev = seed;
        let mut next = prev;
        for map in &maps {
            next = prev;
            for (dest, source, len) in map {
                if prev >= *source && prev < source + len {
                    next = dest + prev - source;
                    break;
                }
            }
            prev = next;
        }
        if next < ans1 {
            ans1 = next;
        }
    }
    // ---------- Part 2
    let mut prev_ranges = seeds
        .chunks_exact(2)
        .map(|x| (x[0], x[1]))
        .collect::<Vec<_>>();
    let mut next_ranges = Vec::new();
    for map in &maps {
        next_ranges = Vec::new();
        let mut q = prev_ranges;
        let mut idx_read = 0;
        while idx_read < q.len() {
            let range_from = q[idx_read].0;
            let range_len = q[idx_read].1;
            idx_read += 1;
            let mut is_processed = false;
            for (dest, source, map_len) in map {
                if range_from + range_len <= *source || source + map_len <= range_from {
                    continue;
                }
                if range_from < *source {
                    q.push((range_from, source - range_from));
                }
                if source + map_len < range_from + range_len {
                    q.push((
                        source + map_len,
                        range_from + range_len - (source + map_len),
                    ));
                }
                let overlap_from = cmp::max(*source, range_from);
                let overlap_to = cmp::min(source + map_len, range_from + range_len);
                let overlap_len = overlap_to - overlap_from;
                next_ranges.push((overlap_from + dest - source, overlap_len));
                is_processed = true;
                break;
            }
            if !is_processed {
                next_ranges.push((range_from, range_len));
            }
        }
        prev_ranges = next_ranges.clone();
    }
    let ans2 = next_ranges.iter().map(|x| x.0).min().unwrap();
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

    #[test]
    fn invalid_at_least_4_lines() {
        test_invalid(&[&"seeds: 79 14 55 13", &"", &"seed-to-soil map"], solve);
    }

    #[test]
    fn invalid_must_start_with_seeds() {
        test_invalid(
            &[&"a: 79 14 55 13", &"", &"seed-to-soil map:", &"50 98 2"],
            solve,
        );
    }

    #[test]
    fn invalid_seed_must_be_followed_by_empty_line() {
        test_invalid(
            &[
                &"seeds: 79 14 55 13",
                &"a",
                &"seed-to-soil map:0",
                &"50 98 2",
            ],
            solve,
        );
    }

    #[test]
    fn invalid_seed_must_be_integer() {
        test_invalid(
            &[&"seeds: 79 a 55 13", &"", &"seed-to-soil map:", &"50 98 2"],
            solve,
        );
    }

    #[test]
    fn invalid_map_sections_must_end_with_map() {
        test_invalid(
            &[&"seeds: 79 14 55 13", &"", &"seed-to-soil a", &"50 98 2"],
            solve,
        );
    }

    #[test]
    fn invalid_map_items_must_be_integer() {
        test_invalid(
            &[&"seeds: 79 14 55 13", &"", &"seed-to-soil map:", &"50 a 2"],
            solve,
        );
    }
}
