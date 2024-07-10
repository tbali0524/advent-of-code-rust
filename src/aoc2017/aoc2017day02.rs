// https://adventofcode.com/2017/day/2

use crate::aoc::runner::PuzzleMetaData;

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2017,
    day: 2,
    title: "Corruption Checksum",
    solutions: (48357, 351),
    example_solutions: [(18, 0), (0, 9)],
    example_string_inputs: ["", ""],
};

pub fn solve(input: &[String]) -> (String, String) {
    let data = input.iter().map(
        |line|line.split_whitespace().map(
            |x|x.parse::<i64>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    // ---------- Part 1
    let ans1: i64 = data.iter().map(|row|row.iter().max().unwrap() - row.iter().min().unwrap()).sum();
    // ---------- Part 2
    let mut ans2 = 0;
    for row0 in data {
        let mut row = row0.to_owned();
        row.sort();
        row.reverse();
        'to_break: for i in 0..row.len() - 1 {
            for j in (i + 1)..row.len() {
                if row[j] != 0 && row[i] % row[j] == 0 {
                    ans2 += row[i] / row[j];
                    break 'to_break;
                }
            }
        }
    }
    (ans1.to_string(), ans2.to_string())
}

// --- boilerplate below ---

pub fn run() -> bool {
    crate::aoc::runner::run_puzzle(&PUZZLE_METADATA, solve)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::runner::tests::test_case;

    #[test]
    fn example1_works() {
        test_case(&PUZZLE_METADATA, 1, solve);
    }

    #[test]
    fn example2_works() {
        test_case(&PUZZLE_METADATA, 2, solve);
    }

    #[test]
    fn puzzle_works() {
        test_case(&PUZZLE_METADATA, 0, solve);
    }
}
