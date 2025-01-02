//! [aoc](https://adventofcode.com/2016/day/7)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashSet;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2016,
        day: 7,
        title: "Internet Protocol Version 7",
        solution: ("118", "260"),
        example_solutions: vec![("2", "0"), ("0", "3")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Part 1 + 2
    let mut ans1 = 0;
    let mut ans2 = 0;
    for row in input.iter() {
        let line = row.chars().collect::<Vec<_>>();
        let mut start = 0;
        let mut has_abba_out = false;
        let mut has_abba_in = false;
        let mut all_aba_out = HashSet::new();
        let mut all_bab_in = HashSet::new();
        while start < line.len() {
            let mut end = line[start..]
                .iter()
                .position(|&c| c == '[')
                .unwrap_or(line.len() - start)
                + start;
            if start != end {
                let sub = &line[start..end];
                if has_abba(sub) {
                    has_abba_out = true;
                }
                all_aba_out.extend(get_all_aba(sub));
                start = end;
                continue;
            }
            start += 1;
            let Some(len_to_end) = line[start..].iter().position(|&c| c == ']') else {
                continue;
            };
            end = len_to_end + start;
            if start != end {
                let sub = &line[start..end];
                if has_abba(sub) {
                    has_abba_in = true;
                }
                all_bab_in.extend(get_all_bab(sub));
                start = end + 1;
                continue;
            }
        }
        if has_abba_out && !has_abba_in {
            ans1 += 1;
        }
        for ab in all_aba_out.iter() {
            if all_bab_in.contains(ab) {
                ans2 += 1;
                break;
            }
        }
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

fn has_abba(s: &[char]) -> bool {
    if s.len() > 3 {
        for i in 3..s.len() {
            if s[i] == s[i - 3] && s[i - 1] == s[i - 2] && s[i] != s[i - 1] {
                return true;
            }
        }
    }
    false
}

fn get_all_aba(s: &[char]) -> HashSet<(char, char)> {
    let mut a = HashSet::new();
    if s.len() > 2 {
        for i in 2..s.len() {
            if s[i] == s[i - 2] && s[i] != s[i - 1] {
                a.insert((s[i], s[i - 1]));
            }
        }
    }
    a
}

fn get_all_bab(s: &[char]) -> HashSet<(char, char)> {
    let mut a = HashSet::new();
    if s.len() > 2 {
        for i in 2..s.len() {
            if s[i] == s[i - 2] && s[i] != s[i - 1] {
                a.insert((s[i - 1], s[i]));
            }
        }
    }
    a
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
}
