//! [aoc](https://adventofcode.com/2016/day/5)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2016,
        day: 5,
        title: "How About a Nice Game of Chess?",
        solution: ("c6697b55", "8c35d1ab"),
        // examples excluded because taking too long even in release mode
        example_solutions: vec![],
        // example_solutions: vec![("18f47a30", "05ace8e3")],
    }
}

pub const LEN: usize = 8;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() != 1 {
        Err("input must have a single line")?;
    }
    // ---------- Part 1
    let mut ans1 = String::new();
    let mut count = 0;
    let mut idx = 0;
    while count < LEN {
        let item = format!("{}{}", input[0], idx);
        let hash = format!("{:x}", md5::compute(&item));
        idx += 1;
        if &hash[0..5] != "00000" {
            continue;
        }
        ans1.push_str(&hash[5..=5]);
        count += 1;
    }
    // ---------- Part 2
    let mut result = ['-'; LEN];
    let mut count = 0;
    let mut idx = 0;
    while count < LEN {
        let item = format!("{}{}", input[0], idx);
        let hash = format!("{:x}", md5::compute(&item));
        idx += 1;
        if &hash[0..5] != "00000" {
            continue;
        }
        let Some(pos) = (hash.as_bytes()[5] as char).to_digit(10) else {
            continue;
        };
        if pos as usize >= LEN || result[pos as usize] != '-' {
            continue;
        }
        result[pos as usize] = hash.as_bytes()[6] as char;
        count += 1;
    }
    let ans2 = result.iter().collect::<String>();
    Ok((ans1, ans2))
}

// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::runner::tests::*;

    // too slow, skipped
    #[test]
    #[ignore]
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
    fn invalid_single_line() {
        test_invalid_msg(&[&"a", &"b"], solve, "input must have a single line");
    }
}
