//! Helper module to run any puzzle solution.

use super::ansi::{ANSI_GREEN, ANSI_RED, ANSI_RESET, ANSI_YELLOW};
use super::{DURATION_THRESHOLD_MILLIS, PUZZLES_TO_SKIP, SKIP_SLOW};
use super::{MAX_DAYS, PUZZLES, START_SEASON};
use super::{PuzzleError, PuzzleExpected, PuzzleMetaData, Solver};
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::cmp::min;
use std::fs;
use std::path;
use std::time;

const MSG_NONE: &str = "      ";
const MSG_PASS: &str = "\x1b[1;37;42m[ OK ]\x1b[0m"; // cannot build &str const from mod ansi &str constants...
const MSG_FAIL: &str = "\x1b[1;37;41m[FAIL]\x1b[0m";
const MSG_SKIP: &str = "\x1b[1;37;43m[SKIP]\x1b[0m";
const MSG_PASS_TOTAL: &str = "\x1b[1;37;42m[ OK ] All tests passed. \x1b[0m";
const MSG_FAIL_TOTAL: &str = "\x1b[1;37;41m[FAIL] Some tests failed. \x1b[0m";

// ------------------------------------------------------------
/// Runs multiple puzzles.
///
/// * `year == None && day == None` : runs all seasons, all days
/// * `year == Some && day == None` : runs a single season, all days
/// * `year == Some && day == Some` : runs a single puzzle
/// * parallel == true  : run solutions in multiple threads, using rayon
/// * parallel == false : run solutions in sequence
///
/// Prints results to `stdout`.
///
/// Returns true if all puzzles that were run passed.
pub fn run_puzzles(year: Option<usize>, day: Option<usize>, parallel: bool) -> bool {
    let now = time::Instant::now();
    let mut count_seasons = 0;
    let mut count_puzzles = 0;
    let mut count_skipped_puzzles = 0;
    let mut count_failed_puzzles = 0;
    let mut count_examples = 0;
    let mut puzzle_list = Vec::new();
    for (idx_season, season_puzzles) in PUZZLES.iter().enumerate() {
        if season_puzzles.is_none() {
            continue;
        }
        let season = START_SEASON + idx_season;
        if year.is_some() && year.unwrap() != season {
            continue;
        }
        count_seasons += 1;
        let season_puzzles = season_puzzles.unwrap();
        for (idx_day, puzzle_functions) in season_puzzles.iter().enumerate() {
            if puzzle_functions.is_none() || (day.is_some() && day.unwrap() != idx_day + 1) {
                continue;
            }
            let mut to_skip = false;
            for (skip_year, skip_day) in PUZZLES_TO_SKIP.iter() {
                if SKIP_SLOW
                    && (year.is_none() || day.is_none())
                    && (season == *skip_year && idx_day + 1 == *skip_day)
                {
                    to_skip = true;
                    count_skipped_puzzles += 1;
                    break;
                }
            }
            let (metadata, solver) = puzzle_functions.unwrap();
            let puzzle = metadata();
            count_puzzles += 1;
            count_examples += puzzle.example_solutions.len();
            puzzle_list.push((puzzle, solver, to_skip));
        }
    }
    let results = if parallel {
        puzzle_list
            .par_iter()
            .progress_count(puzzle_list.len() as u64)
            .map(|(puzzle, solver, to_skip)| run_puzzle(puzzle, *solver, *to_skip))
            .collect::<Vec<_>>()
    } else {
        puzzle_list
            .iter()
            .map(|(puzzle, solver, to_skip)| run_puzzle(puzzle, *solver, *to_skip))
            .collect::<Vec<_>>()
    };
    let elapsed = now.elapsed();
    let mut all_passed = true;
    let mut prev_season = 0;
    for (idx, (passed, message)) in results.iter().enumerate() {
        all_passed = all_passed && *passed;
        if !passed {
            count_failed_puzzles += 1;
        }
        let season = puzzle_list[idx].0.year;
        if season != prev_season {
            println!("======= {season} ===================================================");
        }
        prev_season = season;
        print!("{message}");
    }
    let msg_skip_fail = if count_skipped_puzzles > 0 && count_failed_puzzles > 0 {
        format!(
            " ({ANSI_YELLOW}{count_skipped_puzzles}{ANSI_RESET} skipped, {ANSI_RED}{count_failed_puzzles}{ANSI_RESET} failed)"
        )
    } else if count_skipped_puzzles > 0 {
        format!(" ({ANSI_YELLOW}{count_skipped_puzzles}{ANSI_RESET} skipped)")
    } else if count_failed_puzzles > 0 {
        format!(" ({ANSI_RED}{count_failed_puzzles}{ANSI_RESET} failed)")
    } else {
        String::new()
    };
    println!(
        "======= Total =========== [time: {:5} ms] : {} season{}, {}{}{} puzzle{}{}, {} example{}\n",
        elapsed.as_millis(),
        count_seasons,
        get_plural(count_seasons),
        ANSI_GREEN,
        count_puzzles,
        ANSI_RESET,
        get_plural(count_puzzles),
        msg_skip_fail,
        count_examples,
        get_plural(count_examples),
    );
    let msg = if all_passed {
        MSG_PASS_TOTAL
    } else {
        MSG_FAIL_TOTAL
    };
    if count_puzzles > 0 {
        println!("{msg}\n");
    }
    all_passed
}

// ------------------------------------------------------------
/// Runs a single puzzle, including all examples.
/// Returns tuple of a bool with true if all test cases passed, and a multiple-line message.
pub fn run_puzzle(puzzle: &PuzzleMetaData, solve: Solver, to_skip: bool) -> (bool, String) {
    let now = time::Instant::now();
    let mut all_passed = true;
    let mut all_message = String::new();
    if to_skip {
        all_message = format!("{MSG_SKIP}\n");
    } else {
        let count_examples = puzzle.example_solutions.len();
        let mut cases = (1..=count_examples).collect::<Vec<_>>();
        cases.push(0);
        for case in cases {
            let (passed, message) = run_case(puzzle, solve, case);
            all_passed = all_passed && passed;
            all_message += &message;
        }
    }
    let elapsed = now.elapsed();
    let threshold = time::Duration::from_millis(DURATION_THRESHOLD_MILLIS);
    let (msg_pre, msg_post) = if elapsed >= threshold {
        (ANSI_YELLOW, ANSI_RESET)
    } else {
        ("", "")
    };
    let message = format!(
        "=== AoC {} Day {:2} ===== [time: {}{:5} ms{}] : {}\n{}\n",
        puzzle.year,
        puzzle.day,
        msg_pre,
        elapsed.as_millis(),
        msg_post,
        puzzle.title,
        all_message,
    );
    (all_passed, message)
}

// ------------------------------------------------------------
/// Runs a single puzzle with a single input test case.
///
/// Returns tuple of a bool with true if test case passed, and a multiple-line message.
pub fn run_case(puzzle: &PuzzleMetaData, solve: Solver, case: usize) -> (bool, String) {
    let mut all_message = String::new();
    let input_result = read_input(puzzle, case);
    if let Err(e) = input_result {
        return (false, get_case_error(case, e));
    }
    let input_s = input_result.unwrap();
    let input = input_s.lines().collect::<Vec<&str>>();
    let result = solve(&input);
    if let Err(e) = result {
        return (false, get_case_error(case, e));
    }
    let ans = result.unwrap();
    let expected = get_expected(puzzle, case);
    let mut all_passed = true;
    for part in 1..=2 {
        if part == 2 && puzzle.day == MAX_DAYS {
            continue;
        }
        let expected_case;
        let ans_case;
        if part == 1 {
            expected_case = &expected.0;
            ans_case = &ans.0;
        } else {
            expected_case = &expected.1;
            ans_case = &ans.1;
        }
        let mut pre_msg = MSG_NONE;
        let mut ans_msg = String::new();
        let mut post_msg = String::new();
        if !expected_case.is_empty() && expected_case != &"0" {
            if ans_case == expected_case {
                pre_msg = MSG_PASS;
                ans_msg = ans_case.to_string();
            } else {
                all_passed = false;
                pre_msg = MSG_FAIL;
                ans_msg = format!("{ANSI_YELLOW}{ans_case}{ANSI_RESET}");
                post_msg = format!(
                    "{}[expected: {}]",
                    " ".repeat(20 - min(20, ans_case.to_string().len())),
                    expected_case
                );
            };
        } else if case == 0 {
            if !ans_case.is_empty() {
                ans_msg = ans_case.to_string() + " ";
            }
            post_msg = format!("{ANSI_YELLOW}[missing expected solution]{ANSI_RESET}");
        } else {
            ans_msg = format!("{ANSI_YELLOW}n/a{ANSI_RESET}");
        }
        if case == 0 {
            all_message += &format!("{pre_msg} Puzzle     part #{part} : {ans_msg}{post_msg}\n");
        } else {
            all_message +=
                &format!("{pre_msg} Example #{case} part #{part} : {ans_msg}{post_msg}\n");
        }
    }
    (all_passed, all_message)
}

// ------------------------------------------------------------
fn get_plural(item: usize) -> String {
    if item == 1 {
        String::new()
    } else {
        String::from('s')
    }
}

// ------------------------------------------------------------
fn get_case_error(case: usize, e: PuzzleError) -> String {
    if case == 0 {
        format!("{MSG_FAIL} Puzzle             : {e:?}\n")
    } else {
        format!("{MSG_FAIL} Example #{case}         : {e:?}\n")
    }
}

// ------------------------------------------------------------
fn get_expected<'a>(puzzle: &'a PuzzleMetaData, case: usize) -> PuzzleExpected<'a> {
    if case == 0 {
        puzzle.solution
    } else {
        puzzle.example_solutions[case - 1]
    }
}

// ------------------------------------------------------------
/// Reads input from file for a specific test case.
///
/// * `case == 0` for the puzzle input
/// * `case == 1, 2, ...` for example inputs
pub fn read_input(puzzle: &PuzzleMetaData, case: usize) -> Result<String, PuzzleError> {
    if case > puzzle.example_solutions.len() {
        Err(format!("missing expected solution for example #{case}"))?;
    }
    let input_path = if case == 0 {
        format!(
            "./input/{}/Aoc{}Day{:0>2}.txt",
            puzzle.year, puzzle.year, puzzle.day
        )
    } else {
        format!(
            "./input/{}/Aoc{}Day{:0>2}ex{}.txt",
            puzzle.year, puzzle.year, puzzle.day, case
        )
    };
    let input = fs::read_to_string(path::Path::new(&input_path))
        .map_err(|_| format!("cannot read input file: {input_path}"))?;
    if input.is_empty() {
        Err("empty input")?;
    }
    Ok(input)
}

// ------------------------------------------------------------
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::aoc::{MetaData, PuzzleInput};

    fn invalid_puzzle_metadata() -> PuzzleMetaData<'static> {
        PuzzleMetaData {
            year: 2015,
            day: 0,
            title: "Test",
            solution: ("0", "0"),
            example_solutions: vec![("0", "0"), ("0", "0")],
        }
    }

    #[test]
    fn read_input_invalid_input_files() {
        let puzzle = invalid_puzzle_metadata();
        let result = read_input(&puzzle, 1);
        assert_eq!(result, Err(PuzzleError("empty input".into())));
        let result = read_input(&puzzle, 3);
        assert_eq!(
            result,
            Err(PuzzleError(
                "missing expected solution for example #3".into()
            ))
        );
        let result = read_input(&puzzle, 2);
        assert!(result.is_err()); // cannot read input file: ...
    }

    // ------------------------------------------------------------
    /// Helper function to be used in puzzle solution tests, running a single test case from file input.
    ///
    /// Similar to [`run_case()`] but using assertions and no output.
    pub fn test_case(metadata: MetaData, solve: Solver, case: usize) {
        let puzzle = metadata();
        let input_s = read_input(&puzzle, case).unwrap();
        let input = input_s.lines().collect::<Vec<&str>>();
        let ans = solve(&input).unwrap();
        let expected = get_expected(&puzzle, case);
        if !expected.0.is_empty() && expected.0 != "0" {
            assert_eq!(&ans.0, expected.0);
        }
        if !expected.1.is_empty() && expected.1 != "0" {
            assert_eq!(&ans.1, expected.1);
        }
    }

    /// Helper function to be used in puzzle solution tests, for checking the handling of invalid puzzle inputs.
    pub fn test_invalid(input: PuzzleInput, solve: Solver) {
        assert!(solve(input).is_err());
    }

    /// Similar to `test_invalid()`, but also checks if the error message contains the givent string slice.
    pub fn test_invalid_msg(input: PuzzleInput, solve: Solver, msg: &str) {
        let result = solve(input);
        assert!(result.is_err());
        if !msg.is_empty() {
            let e = result.unwrap_err().0;
            if !e.contains(msg) {
                eprintln!("*** Error message does not match the expected: {e} != {msg}");
            }
            assert!(e.contains(msg));
        }
    }
}
