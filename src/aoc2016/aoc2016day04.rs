//! [aoc](https://adventofcode.com/2016/day/4)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::cmp::Reverse;
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2016,
        day: 4,
        title: "Security Through Obscurity",
        solution: ("137896", "501"),
        example_solutions: vec![("1514", "0")],
    }
}

type ItemType = u32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut orig_names = Vec::new();
    let mut names = Vec::new();
    let mut ids = Vec::new();
    let mut checksums = Vec::new();
    for &line in input.iter() {
        if line.len() < 12 {
            Err("line must be at least 12 characters")?;
        }
        orig_names.push(line[..(line.len() - 11)].to_owned());
        names.push(line[..(line.len() - 11)].replace("-", ""));
        ids.push(
            line[(line.len() - 10)..(line.len() - 7)]
                .parse::<ItemType>()
                .map_err(|_| "id part must be integer")?,
        );
        checksums.push(line[(line.len() - 6)..(line.len() - 1)].to_owned());
    }
    // ---------- Part 1
    let mut ans1 = 0;
    let mut real_room_indices = Vec::new();
    for (idx, name) in names.iter().enumerate() {
        let mut freq = HashMap::new();
        for c in name.chars() {
            freq.entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        let mut freq_vec = freq.into_iter().collect::<Vec<_>>();
        freq_vec.sort_by_key(|(c, counter)| (Reverse(*counter), *c));
        let top = freq_vec.iter().take(5).map(|(c, _)| *c).collect::<String>();
        if top == checksums[idx] {
            ans1 += ids[idx];
            real_room_indices.push(idx);
        }
    }
    // ---------- Part 2
    let mut ans2 = 0;
    for &idx in real_room_indices.iter() {
        let mut s = orig_names[idx].chars().collect::<Vec<_>>();
        for c in &mut s {
            if *c == '-' {
                *c = ' ';
                continue;
            }
            *c = ((b'a' as ItemType + (ids[idx] + *c as ItemType - b'a' as ItemType) % 26) as u8)
                as char;
        }
        if s.iter().collect::<String>().contains("northpole") {
            ans2 = ids[idx];
            break;
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
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_id_must_be_integer() {
        test_invalid_msg(
            &[&"aaaaa-bbb-z-y-x-aaa[abxyz]"],
            solve,
            "id part must be integer",
        );
    }

    #[test]
    fn invalid_line_must_be_at_least_12_chars_long() {
        test_invalid_msg(
            &[&"-123[abxyz]"],
            solve,
            "line must be at least 12 characters",
        );
    }
}
