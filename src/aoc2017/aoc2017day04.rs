// https://adventofcode.com/2017/day/4

use crate::aoc::PuzzleMetaData;
use crate::aoc::PuzzleResult;

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2017,
    day: 4,
    title: "High-Entropy Passphrases",
    solution: (386, 208),
    example_solutions: [(2, 0), (0, 3)],
    string_solution: None,
    example_string_solutions: None,
    example_string_inputs: None,
};

pub fn solve(input: &[String]) -> PuzzleResult {
    // ---------- Parse and Check input
    let data = input
        .iter()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
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
// --- boilerplate below ---

pub fn run() -> bool {
    crate::aoc::runner::run_puzzle(&PUZZLE_METADATA, solve)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::runner::tests::*;

    #[test]
    fn example1() {
        test_case(&PUZZLE_METADATA, 1, solve);
    }

    #[test]
    fn example2() {
        test_case(&PUZZLE_METADATA, 2, solve);
    }

    #[test]
    fn puzzle() {
        test_case(&PUZZLE_METADATA, 0, solve);
    }
}
