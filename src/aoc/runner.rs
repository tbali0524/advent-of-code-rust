// helper module to run any puzzles

use std::env;
use std::fs;
use std::path::Path;
use std::time::Instant;
use super::*;

pub const MSG_TITLE: &str = "Advent of Code - solutions in Rust, (c) 2024 by TBali";
pub const ANSI_RED: &str= "\x1b[1;37;41m";
pub const ANSI_YELLOW: &str= "\x1b[1;37;43m";
pub const _ANSI_GREEN: &str= "\x1b[1;37;42m";
pub const ANSI_RESET: &str= "\x1b[0m";
const MSG_NONE: &str = "      ";
const MSG_PASS: &str = "\x1b[1;37;42m[ OK ]\x1b[0m";
const MSG_FAIL: &str = "\x1b[1;37;41m[FAIL]\x1b[0m";
const MSG_PASS_TOTAL: &str = "\x1b[1;37;42m[ OK ] ALL TESTS PASSED \x1b[0m";
const MSG_FAIL_TOTAL: &str = "\x1b[1;37;41m[FAIL] SOME TESTS FAILED \x1b[0m";

// ------------------------------------------------------------
pub fn run_puzzles(year: Option<usize>, day: Option<usize>) -> bool {
    let now = Instant::now();
    let mut all_passed = true;
    for (idx, season_puzzles) in crate::aoc::PUZZLES.iter().enumerate() {
        if season_puzzles.is_none() {
            continue;
        }
        let season = crate::aoc::START_SEASON + idx;
        if year.is_some() && year.unwrap() != season {
            continue;
        }
        println!("======= {} =============================================", season);
        for (day0, run) in season_puzzles.unwrap().iter().enumerate() {
            if run.is_none() || (day.is_some() && day.unwrap() != day0 + 1) {
                continue;
            }
            let passed = run.unwrap()();
            all_passed = all_passed && passed;
        }
    }
    let elapsed = now.elapsed();
    println!("==================== [Total time: {:.1?}]\n", elapsed);
    println!("{}\n", if all_passed { MSG_PASS_TOTAL } else { MSG_FAIL_TOTAL });
    all_passed
}

// ------------------------------------------------------------
pub fn run_puzzle(puzzle: &PuzzleMetaData, solve: Solver) -> bool {
    let now = Instant::now();
    let mut all_passed = true;
    let mut all_message = String::new();
    let count_examples = get_example_count(puzzle);
    let mut cases = Vec::new();
    for case in 1..=count_examples {
        cases.push(case);
    }
    cases.push(0);
    for case in cases {
        let (passed, message) = run_case(puzzle, case, solve);
        all_passed = all_passed && passed;
        all_message += &message;
    }
    let elapsed = now.elapsed();
    println!("=== AoC {} Day {:2} ===== [time: {:.1?}] : {}", puzzle.year, puzzle.day, elapsed, puzzle.title);
    print!("{}", all_message);
    all_passed
}

// ------------------------------------------------------------
fn run_case(puzzle: &PuzzleMetaData, case: usize, solve: Solver) -> (bool, String) {
    let mut all_message = String::new();
    let input_result = read_input(puzzle, case);
    if let Err(e) = input_result {
        return if case == 0 {
            (false, format!("{MSG_FAIL} Puzzle             : {}\n", e))
        } else {
            (false, format!("{MSG_FAIL} Example #{}         : {}\n", case, e))
        }
    }
    let input = input_result.unwrap();
    let result = solve(&input);
    if let Err(e) = result {
        return if case == 0 {
            (false, format!("{MSG_FAIL} Puzzle             : {}\n", e))
        } else {
            (false, format!("{MSG_FAIL} Example #{}         : {}\n", case, e))
        }
    }
    let (ans1, ans2) = result.unwrap();
    let (expected1, expected2) = if case == 0 { puzzle.solutions } else { puzzle.example_solutions[case - 1] };
    let mut all_passed = true;
    // part 1
    let mut pre_msg = MSG_NONE;
    let mut post_msg = String::new();
    if expected1 != 0 {
        if ans1 == expected1.to_string() {
            pre_msg = MSG_PASS;
        } else {
            all_passed = false;
            pre_msg = MSG_FAIL;
            post_msg = format!(" [expected: {ANSI_YELLOW}{}{ANSI_RESET}]", expected1);
        };
    }
    if case == 0 {
        all_message += &format!("{} Puzzle     part #1 : {}{}\n", pre_msg, ans1, post_msg);
    } else {
        all_message += &format!("{} Example #{} part #1 : {}{}\n", pre_msg, case, ans1, post_msg);
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
            post_msg = format!(" [expected: {ANSI_YELLOW}{}{ANSI_RESET}]", expected2);
        };
    }
    if case == 0 {
        all_message += &format!("{} Puzzle     part #2 : {}{}\n", pre_msg, ans2, post_msg);
    } else {
        all_message += &format!("{} Example #{} part #2 : {}{}\n", pre_msg, case, ans2, post_msg);
    }
    (all_passed, all_message)
}

pub fn get_example_count(puzzle: &PuzzleMetaData) -> usize {
    if !puzzle.example_string_inputs[0].is_empty() {
        return if puzzle.example_string_inputs[1].is_empty() { 1 } else { 2 }
    }
    let mut case = 0;
    loop {
        let input_path = format!("input\\{}\\Aoc{}Day{:0>2}ex{}.txt", puzzle.year, puzzle.year, puzzle.day, case + 1);
        if !Path::new(&input_path).exists() {
            return case
        }
        case += 1;
    }
}

// ------------------------------------------------------------
// Gets the input for a specific test case by reading from the appropriate file or taken from a constant.
// case is 0 for the normal puzzle input; 1 or 2 for the example inputs
pub fn read_input(puzzle: &PuzzleMetaData, case: usize) -> PuzzleInput {
    let input = if case > 0 && !puzzle.example_string_inputs[0].is_empty() {
        if case > 2 {
            return Err("missing input");
        }
        vec![puzzle.example_string_inputs[case - 1].to_owned()]
    } else {
        let input_path = if case == 0 {
            format!("input\\{}\\Aoc{}Day{:0>2}.txt", puzzle.year, puzzle.year, puzzle.day)
        } else {
            format!("input\\{}\\Aoc{}Day{:0>2}ex{}.txt", puzzle.year, puzzle.year, puzzle.day, case)
        };
        let binding = fs::read_to_string(input_path);
        if binding.is_err() {
            return Err("missing input");
        }
        let v = binding.unwrap().lines().map(|x|x.to_owned()).collect::<Vec<_>>();
        if v.is_empty() {
            return Err("empty input");
        }
        v
    };
    Ok(input)
}

// ------------------------------------------------------------
pub fn print_help() {
    println!("You can run the solutions for a specific puzzle, for a full seasonm or for all seasons.");
    println!("Usage:  aoc.bat [year] [day]\n");
}

// ------------------------------------------------------------
pub fn parse_args() -> Result<(Option<usize>, Option<usize>), &'static str> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            Ok((None, None))
        },
        2 => {
            if let Ok(year) = args[1].parse::<usize>() {
                Ok((Some(year), None))
            } else {
                Err("Invalid argument")
            }
        },
        3 => {
            let year = args[1].parse::<usize>();
            let day = args[2].parse::<usize>();
            if year.is_err() || day.is_err() {
                Err("Invalid argument")
            } else {
                Ok((year.ok(), day.ok()))
            }
        },
        _ => {
            Err("Too many arguments")
        }
    }
}

// ------------------------------------------------------------
#[cfg(test)]
pub mod tests {
    use super::*;

    // similar to run_case() but using assertions instead of printing to standard output.
    pub fn test_case(puzzle: &PuzzleMetaData, case: usize, solve: Solver) {
        let input = read_input(puzzle, case).unwrap();
        let result = solve(&input);
        if result.is_err() {
            assert!(false);
        }
        let (ans1, ans2) = result.unwrap();
        let (expected1, expected2) = if case == 0 { puzzle.solutions } else { puzzle.example_solutions[case - 1] };
        if expected1 != 0 {
            assert_eq!(ans1, expected1.to_string());
        }
        if expected2 != 0 {
            assert_eq!(ans2, expected2.to_string());
        }
    }

    pub fn test_invalid(_puzzle: &PuzzleMetaData, input: &[String], solve: Solver) {
        let result = solve(&input);
        assert!(result.is_err());
    }
}
