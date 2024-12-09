//! [aoc](https://adventofcode.com/2024/day/9)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 9,
        title: "Disk Fragmenter",
        solution: ("6332189866718", "6353648390778"),
        example_solutions: vec![("1928", "2858")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() != 1 {
        Err("input must have a single line")?;
    }
    let data = input[0]
        .chars()
        .map(|x| x.to_digit(10).ok_or("input must contain only digits"))
        .collect::<Result<Vec<_>, _>>()?;
    // ---------- Part 1
    let mut disk = Vec::new();
    for (idx, &length) in data.iter().enumerate() {
        for _ in 0..length {
            match idx % 2 {
                0 => disk.push(Some(idx / 2)),
                1 => disk.push(None),
                _ => unreachable!(),
            }
        }
    }
    let mut left = 0;
    let mut right = disk.len() - 1;
    loop {
        while left < right && disk[left].is_some() {
            left += 1;
        }
        while left < right && disk[right].is_none() {
            right -= 1;
        }
        if left == right {
            break;
        }
        disk[left] = disk[right];
        disk[right] = None;
    }
    let mut pos = 0;
    let mut ans1 = 0;
    while pos < disk.len() && disk[pos].is_some() {
        ans1 += pos * disk[pos].unwrap();
        pos += 1;
    }
    // ---------- Part 2
    let mut files = Vec::new();
    let mut gaps = Vec::new();
    let mut pos = 0;
    for (idx, &length) in data.iter().enumerate() {
        match idx % 2 {
            0 => files.push((pos, length, idx / 2)),
            1 => gaps.push((pos, length)),
            _ => unreachable!(),
        }
        pos += length;
    }
    let mut idx_file = files.len() - 1;
    loop {
        let (file_pos, file_len, file_id) = files[idx_file];
        let find_gap = gaps
            .iter()
            .position(|&(gap_pos, gap_len)| gap_pos < file_pos && gap_len >= file_len);
        if let Some(idx_gap) = find_gap {
            let (gap_pos, gap_len) = gaps[idx_gap];
            if gap_len > file_len {
                gaps[idx_gap] = (gap_pos + file_len, gap_len - file_len);
            } else {
                gaps.remove(idx_gap);
            }
            files[idx_file] = (gap_pos, file_len, file_id);
        }
        if idx_file == 0 {
            break;
        }
        idx_file -= 1;
    }
    files.sort_by_key(|&x| x.0);
    let mut ans2 = 0u64;
    for &file in files.iter() {
        let (file_pos, file_len, file_id) = file;
        for i in 0..file_len {
            ans2 += (file_pos + i) as u64 * file_id as u64;
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
    fn invalid_single_line() {
        test_invalid_msg(&[&"1", &"2"], solve, "input must have a single line");
    }

    #[test]
    fn invalid_digit() {
        test_invalid_msg(&[&"a"], solve, "input must contain only digits");
    }
}
