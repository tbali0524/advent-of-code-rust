// helper module to run any puzzles

use std::fs;
use std::time::Instant;

type Solver = fn(&[String]) -> (String, String);

pub struct PuzzleMetaData<'a> {
    pub year: u32,
    pub day: u32,
    pub title: &'a str,
    pub solutions: (i64, i64),
    pub example_solutions: [(i64, i64); 2],
    pub example_string_inputs: [&'a str; 2],
}

const MSG_TITLE: &str = "Advent of Code - solutions in Rust, (c) 2024 by TBali";
const MSG_NONE: &str = "      ";
const MSG_PASS: &str = "\x1b[1;37;42m[ OK ]\x1b[0m";
const MSG_FAIL: &str = "\x1b[1;37;41m[FAIL]\x1b[0m";
const MSG_PASS_TOTAL: &str = "\x1b[1;37;42m[ OK ] ALL TESTS PASSED \x1b[0m";
const MSG_FAIL_TOTAL: &str = "\x1b[1;37;41m[FAIL] SOME TESTS FAILED \x1b[0m";

pub fn run_all() -> bool {
    let now = Instant::now();
    println!("{}", MSG_TITLE);
    let mut all_passed = true;
    for (season, puzzles) in crate::aoc::get_puzzles() {
        println!("======= {} =============================================", season);
        for run in puzzles {
            let passed = run();
            all_passed = all_passed && passed;
        }
    }
    let elapsed = now.elapsed();
    println!("=== Total time: {:.1?}\n", elapsed);
    println!("{}\n", if all_passed { MSG_PASS_TOTAL } else { MSG_FAIL_TOTAL });
    all_passed
}

pub fn run_puzzle(puzzle: &PuzzleMetaData, solve: Solver) -> bool {
    let now = Instant::now();
    let mut all_passed = true;
    let mut all_message = String::new();
    for case in [1, 2, 0] {
        let (passed, message) = run_case(puzzle, case, solve);
        all_passed = all_passed && passed;
        all_message += &message;
    }
    let elapsed = now.elapsed();
    println!("=== AoC {} Day {} : [time: {:.1?}]   {}", puzzle.year, puzzle.day, elapsed, puzzle.title);
    print!("{}", all_message);
    all_passed
}

pub fn run_case(puzzle: &PuzzleMetaData, case: usize, solve: Solver) -> (bool, String) {
    let input = read_input(puzzle, case);
    let (ans1, ans2) = solve(&input);
    let (expected1, expected2) = if case == 0 { puzzle.solutions } else { puzzle.example_solutions[case - 1] };
    let mut all_passed = true;
    let mut all_message = String::new();
    // part 1
    let mut pre_msg = MSG_NONE;
    let mut post_msg = String::new();
    if expected1 != 0 {
        if ans1 == expected1.to_string() {
            pre_msg = MSG_PASS;
        } else {
            all_passed = false;
            pre_msg = MSG_FAIL;
            post_msg = format!(" [expected: {}]", expected1);
        };
    }
    if case == 0 {
        all_message += &format!("{} Puzzle,     part #1: {}{}\n", pre_msg, ans1, post_msg);
    } else {
        all_message += &format!("{} Example #{}, part #1: {}{}\n", pre_msg, case, ans1, post_msg);
    }
    // part 2
    let mut pre_msg = MSG_NONE;
    let mut post_msg = String::new();
    if expected2 != 0 {
        if ans2 == expected2.to_string() {
            pre_msg = MSG_PASS;
        } else {
            all_passed = false;
            pre_msg = MSG_FAIL;
            post_msg = format!(" [expected: {}]", expected2);
        };
    }
    if case == 0 {
        all_message += &format!("{} Puzzle,     part #2: {}{}\n", pre_msg, ans2, post_msg);
    } else {
        all_message += &format!("{} Example #{}, part #2: {}{}\n", pre_msg, case, ans2, post_msg);
    }
    (all_passed, all_message)
}

// Gets the input for a specific test case by reading from the appropriate file or taken from a constant.
//
// case is 0 for the normal puzzle input; 1 or 2 for the example inputs
pub fn read_input(puzzle: &PuzzleMetaData, case: usize) -> Vec<String> {
    let input = if case > 0 && !puzzle.example_string_inputs[0].is_empty() {
        vec![puzzle.example_string_inputs[case - 1].to_owned()]
    } else {
        let input_path = if case == 0 {
            format!("input\\{}\\Aoc{}Day{:0>2}.txt", puzzle.year, puzzle.year, puzzle.day)
        } else {
            format!("input\\{}\\Aoc{}Day{:0>2}ex{}.txt", puzzle.year, puzzle.year, puzzle.day, case)
        };
        let binding = fs::read_to_string(input_path).unwrap();
        binding.lines().map(|x|x.to_owned()).collect()
    };
    input
}

#[cfg(test)]
pub mod tests {
    use super::*;

    // similar to run_case() but using assertions instead of printing to standard output.
    pub fn test_case(puzzle: &PuzzleMetaData, case: usize, solve: Solver) {
        let input = read_input(puzzle, case);
        let (ans1, ans2) = solve(&input);
        let (expected1, expected2) = if case == 0 { puzzle.solutions } else { puzzle.example_solutions[case - 1] };
        if expected1 != 0 {
            assert_eq!(ans1, expected1.to_string());
        }
        if expected2 != 0 {
            assert_eq!(ans2, expected2.to_string());
        }
    }
}
