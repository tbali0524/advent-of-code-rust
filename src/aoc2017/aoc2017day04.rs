//! [aoc](https://adventofcode.com/2017/day/4)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2017,
        day: 4,
        title: "High-Entropy Passphrases",
        solution: ("386", "208"),
        example_solutions: vec![("2", "0"), ("0", "3")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let data = input
        .iter()
        .map(|&line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    // ---------- Part 1
    let mut ans1 = 0;
    for row in &data {
        let mut row_unique = row.to_owned();
        row_unique.sort();
        row_unique.dedup();
        // dedup(&mut row_unique);
        if row_unique.len() == row.len() {
            ans1 += 1;
        }
    }
    // ---------- Part 2
    let mut ans2 = 0;
    for row in &data {
        let mut row_unique = Vec::new();
        for item in row {
            let mut word = item.chars().collect::<Vec<_>>();
            word.sort();
            row_unique.push(word.iter().collect::<String>());
        }
        row_unique.sort();
        row_unique.dedup();
        if row_unique.len() == row.len() {
            ans2 += 1;
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
    fn example2() {
        test_case(metadata, solve, 2);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }
}
