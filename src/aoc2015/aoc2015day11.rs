//! [aoc](https://adventofcode.com/2015/day/11)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 11,
        title: "Corporate Policy",
        solution: ("vzbxxyzz", "vzcaabcc"),
        example_solutions: vec![("abcdffaa", "0"), ("ghjaabcc", "0")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Check input
    if input.len() != 1 {
        return Err(PuzzleError("input must have a single line".into()));
    }
    // ---------- Part 1
    let ans1 = next_password(input[0])?;
    // ---------- Part 2
    let ans2 = next_password(&ans1)?;
    Ok((ans1, ans2))
}

fn next_password(prev: &str) -> Result<String, PuzzleError> {
    let mut pw = Vec::from(prev);
    loop {
        let mut i = pw.len() as isize - 1;
        while i >= 0 && pw[i as usize] == b'z' {
            pw[i as usize] = b'a';
            i -= 1;
        }
        if i < 0 {
            return Err(PuzzleError("password overflow".into()));
        }
        pw[i as usize] += 1;
        let mut is_ok = false;
        for i in 2..pw.len() {
            if pw[i] == pw[i - 1] + 1 && pw[i - 1] == pw[i - 2] + 1 {
                is_ok = true;
                break;
            }
        }
        if !is_ok {
            continue;
        }
        let mut count = 0;
        for needle in [b'i', b'o', b'l'] {
            count += pw.iter().filter(|&x| *x == needle).count();
        }
        if count > 0 {
            continue;
        }
        let mut first_pos = HashMap::new();
        count = 0;
        for i in 1..pw.len() {
            if pw[i] != pw[i - 1] {
                continue;
            }
            let v = *first_pos.entry(&pw[i]).or_insert(i);
            if i == v + 1 {
                continue;
            }
            count += 1;
        }
        if count >= 2 {
            break;
        }
    }
    Ok(String::from_utf8(pw).unwrap())
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
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_single_line() {
        test_invalid(&[&"a", &"b"], solve);
    }
}
