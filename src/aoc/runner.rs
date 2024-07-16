//! Helper module to run any puzzle solution.

use super::*;
use std::fs;
use std::path;
use std::time;

pub const MSG_TITLE: &str = "Advent of Code - solutions in Rust, (c) 2024 by TBali";
pub const ARG_HELP: &str = "--help";
pub const ARG_VERSION: &str = "--version";

pub const ANSI_RED_INV: &str = "\x1b[1;37;41m";
pub const ANSI_RESET: &str = "\x1b[0m";

const _ANSI_GREEN_INV: &str = "\x1b[1;37;42m";
const ANSI_GREEN: &str = "\x1b[32m";
const ANSI_YELLOW: &str = "\x1b[33m";
const MSG_NONE: &str = "      ";
const MSG_PASS: &str = "\x1b[1;37;42m[ OK ]\x1b[0m";
const MSG_FAIL: &str = "\x1b[1;37;41m[FAIL]\x1b[0m";
const MSG_PASS_TOTAL: &str = "\x1b[1;37;42m[ OK ] ALL TESTS PASSED \x1b[0m";
const MSG_FAIL_TOTAL: &str = "\x1b[1;37;41m[FAIL] SOME TESTS FAILED \x1b[0m";
const DURATION_THRESHOLD_MILLIS: u64 = 500;

// ------------------------------------------------------------
/// Runs multiple puzzles, prints results to stdout.
///
/// * `year == None && day == None` : run all seasons, all days
/// * `year == Some && day == None` : run a single season, all days
/// * `year == Some && day == Some` : run a single puzzle
pub fn run_puzzles(year: Option<usize>, day: Option<usize>) -> bool {
    let now = time::Instant::now();
    let mut all_passed = true;
    let mut count_puzzles = 0;
    for (idx, season_puzzles) in crate::aoc::PUZZLES.iter().enumerate() {
        if season_puzzles.is_none() {
            continue;
        }
        let season = crate::aoc::START_SEASON + idx;
        if year.is_some() && year.unwrap() != season {
            continue;
        }
        println!(
            "======= {} ===================================================",
            season
        );
        for (day0, run) in season_puzzles.unwrap().iter().enumerate() {
            if run.is_none() || (day.is_some() && day.unwrap() != day0 + 1) {
                continue;
            }
            let passed = run.unwrap()();
            all_passed = all_passed && passed;
            count_puzzles += 1;
        }
    }
    let elapsed = now.elapsed();
    println!(
        "=================== [Total time: {:5} ms]  [# of puzzles: {}{}{}]\n",
        elapsed.as_millis(),
        ANSI_GREEN,
        count_puzzles,
        ANSI_RESET,
    );
    let msg = if all_passed {
        MSG_PASS_TOTAL
    } else {
        MSG_FAIL_TOTAL
    };
    if count_puzzles > 0 {
        println!("{}\n", msg);
    }
    all_passed
}

// ------------------------------------------------------------
/// Runs a single puzzle, including all examples.
pub fn run_puzzle(puzzle: &PuzzleMetaData, solve: Solver) -> bool {
    let now = time::Instant::now();
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
    let threshold = time::Duration::from_millis(DURATION_THRESHOLD_MILLIS);
    let (msg_pre, msg_post) = if elapsed >= threshold {
        (ANSI_YELLOW, ANSI_RESET)
    } else {
        ("", "")
    };
    println!(
        "=== AoC {} Day {:2} ===== [time: {}{:5} ms{}] : {}",
        puzzle.year,
        puzzle.day,
        msg_pre,
        elapsed.as_millis(),
        msg_post,
        puzzle.title
    );
    print!("{}", all_message);
    all_passed
}

// ------------------------------------------------------------
/// Runs a single puzzle with a single input test case.
pub fn run_case(puzzle: &PuzzleMetaData, case: usize, solve: Solver) -> (bool, String) {
    let mut all_message = String::new();
    let input_result = read_input(puzzle, case);
    if let Err(e) = input_result {
        return (false, get_case_error(case, e));
    }
    let input = input_result.unwrap();
    let result = solve(&input);
    if let Err(e) = result {
        return (false, get_case_error(case, e));
    }
    let ans = result.unwrap();
    let expected = get_expected(puzzle, case);
    let mut all_passed = true;
    for part in 1..=2 {
        if part == 2 && puzzle.day == crate::aoc::MAX_DAYS {
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
        let mut post_msg = String::new();
        if !expected_case.is_empty() && expected_case != "0" {
            if ans_case == expected_case {
                pre_msg = MSG_PASS;
            } else {
                all_passed = false;
                pre_msg = MSG_FAIL;
                post_msg = format!(" [expected: {ANSI_YELLOW}{}{ANSI_RESET}]", expected_case);
            };
        } else if case == 0 {
            post_msg = format!(" {ANSI_YELLOW}[missing expected solution]{ANSI_RESET}");
        }
        if case == 0 {
            all_message += &format!(
                "{} Puzzle     part #{} : {}{}\n",
                pre_msg, part, ans_case, post_msg
            );
        } else {
            all_message += &format!(
                "{} Example #{} part #{} : {}{}\n",
                pre_msg, case, part, ans_case, post_msg
            );
        }
    }
    (all_passed, all_message)
}

// ------------------------------------------------------------
fn get_case_error(case: usize, e: &str) -> String {
    if case == 0 {
        format!("{MSG_FAIL} Puzzle             : {}\n", e)
    } else {
        format!("{MSG_FAIL} Example #{}         : {}\n", case, e)
    }
}

// ------------------------------------------------------------
fn get_expected(puzzle: &PuzzleMetaData, case: usize) -> (String, String) {
    if puzzle.string_solution.is_some() {
        let expected = if case == 0 {
            puzzle.string_solution.unwrap()
        } else {
            puzzle.example_string_solutions.unwrap()[case - 1]
        };
        (expected.0.to_string(), expected.1.to_string())
    } else {
        let expected = if case == 0 {
            puzzle.solution
        } else {
            puzzle.example_solutions[case - 1]
        };
        (expected.0.to_string(), expected.1.to_string())
    }
}

// ------------------------------------------------------------
/// Calculates how many examples exists for a given puzzle, based on input file availability.
fn get_example_count(puzzle: &PuzzleMetaData) -> usize {
    if puzzle.example_string_inputs.is_some() {
        return if puzzle.example_string_inputs.unwrap()[1].is_empty() {
            1
        } else {
            2
        };
    }
    let mut case = 0;
    loop {
        let input_path = format!(
            "./input/{}/Aoc{}Day{:0>2}ex{}.txt",
            puzzle.year,
            puzzle.year,
            puzzle.day,
            case + 1
        );
        if !path::Path::new(&input_path).exists() {
            return case;
        }
        case += 1;
    }
}

// ------------------------------------------------------------
/// Gets the input for a specific test case by reading from the input file (or taken from a constant).
///
/// `case == 0` means the the normal puzzle input; 1 or 2 means an example input
pub fn read_input(puzzle: &PuzzleMetaData, case: usize) -> PuzzleInput {
    let input = if case > 0 && puzzle.example_string_inputs.is_some() {
        if case > 2 {
            return Err("missing input");
        }
        vec![puzzle.example_string_inputs.unwrap()[case - 1].to_owned()]
    } else {
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
        let binding = fs::read_to_string(path::Path::new(&input_path));
        if binding.is_err() {
            return Err("missing input");
        }
        let v = binding
            .unwrap()
            .lines()
            .map(|x| x.to_owned())
            .collect::<Vec<_>>();
        if v.is_empty() {
            return Err("empty input");
        }
        v
    };
    Ok(input)
}

// ------------------------------------------------------------
pub fn print_help() {
    println!(
        "You can run the solutions for a specific puzzle, for a full seasonm or for all seasons."
    );
    println!("Usage:  aoc.bat [year] [day]\n");
}

// ------------------------------------------------------------
/// Tries to parse CLI arguments to year and day, no output.
pub fn parse_args(args: &[String]) -> Result<(Option<usize>, Option<usize>), &'static str> {
    match args.len() {
        1 => Ok((None, None)),
        2 => {
            if args[1] == ARG_HELP {
                return Err(ARG_HELP);
            }
            if args[1] == ARG_VERSION {
                return Err(ARG_VERSION);
            }
            let year = args[1]
                .parse::<usize>()
                .map_err(|_| "Invalid argument: year must be integer")?;
            if !(START_SEASON..START_SEASON + MAX_SEASONS).contains(&year) {
                return Err("Invalid argument: year out of range (valid: 2015-2024)");
            }
            Ok((Some(year), None))
        }
        3 => {
            let year = args[1]
                .parse::<usize>()
                .map_err(|_| "Invalid argument: year must be integer")?;
            let day = args[2]
                .parse::<usize>()
                .map_err(|_| "Invalid argument: day must be integer")?;
            if !(START_SEASON..START_SEASON + MAX_SEASONS).contains(&year) {
                return Err("Invalid argument: year out of range (valid: 2015-2024)");
            }
            if !(1..=MAX_DAYS).contains(&day) {
                return Err("Invalid argument: day out of range (valid: 1-25)");
            }
            Ok((Some(year), Some(day)))
        }
        _ => Err("Too many arguments"),
    }
}

// ------------------------------------------------------------
#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn get_example_count_works() {
        assert_eq!(get_example_count(&TEST_PUZZLE_METADATA), 1);
    }

    #[test]
    fn read_input_invalid_input_files() {
        let result = read_input(&TEST_PUZZLE_METADATA, 1);
        assert_eq!(result, Err("empty input"));
        let result = read_input(&TEST_PUZZLE_METADATA, 2);
        assert_eq!(result, Err("missing input"));
    }

    #[test]
    fn parse_args_invalid_arguments() {
        let args = [
            String::from("aoc"),
            String::from("2017"),
            String::from("2"),
            String::from("3"),
        ];
        assert_eq!(parse_args(&args), Err("Too many arguments"));
        let args = [String::from("aoc"), String::from("year")];
        assert_eq!(
            parse_args(&args),
            Err("Invalid argument: year must be integer")
        );
        let args = [String::from("aoc"), String::from("2014")];
        assert_eq!(
            parse_args(&args),
            Err("Invalid argument: year out of range (valid: 2015-2024)")
        );
        let args = [String::from("aoc"), String::from("2030")];
        assert_eq!(
            parse_args(&args),
            Err("Invalid argument: year out of range (valid: 2015-2024)")
        );
        let args = [String::from("aoc"), String::from("year"), String::from("2")];
        assert_eq!(
            parse_args(&args),
            Err("Invalid argument: year must be integer")
        );
        let args = [String::from("aoc"), String::from("2014"), String::from("2")];
        assert_eq!(
            parse_args(&args),
            Err("Invalid argument: year out of range (valid: 2015-2024)")
        );
        let args = [String::from("aoc"), String::from("2030"), String::from("2")];
        assert_eq!(
            parse_args(&args),
            Err("Invalid argument: year out of range (valid: 2015-2024)")
        );
        let args = [
            String::from("aoc"),
            String::from("2017"),
            String::from("day"),
        ];
        assert_eq!(
            parse_args(&args),
            Err("Invalid argument: day must be integer")
        );
        let args = [String::from("aoc"), String::from("2017"), String::from("0")];
        assert_eq!(
            parse_args(&args),
            Err("Invalid argument: day out of range (valid: 1-25)")
        );
        let args = [
            String::from("aoc"),
            String::from("2017"),
            String::from("26"),
        ];
        assert_eq!(
            parse_args(&args),
            Err("Invalid argument: day out of range (valid: 1-25)")
        );
    }

    #[test]
    fn parse_args_valid_arguments() {
        let args = [String::from("aoc"), String::from("--version")];
        assert_eq!(parse_args(&args), Err("--version"));
        let args = [String::from("aoc"), String::from("--help")];
        assert_eq!(parse_args(&args), Err("--help"));
        let args = [String::from("aoc")];
        assert_eq!(parse_args(&args), Ok((None, None)));
        let args = [String::from("aoc"), String::from("2017")];
        assert_eq!(parse_args(&args), Ok((Some(2017), None)));
        let args = [String::from("aoc"), String::from("2017"), String::from("2")];
        assert_eq!(parse_args(&args), Ok((Some(2017), Some(2))));
    }

    const TEST_PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
        year: 2024,
        day: 0,
        title: "Test",
        solution: (0, 0),
        example_solutions: [(0, 0), (0, 0)],
        string_solution: None,
        example_string_solutions: None,
        example_string_inputs: None,
    };

    // ------------------------------------------------------------
    /// Helper function to be used in puzzle solution tests.
    ///
    /// Similar to `run_case()` but using assertions and no output.
    pub fn test_case(puzzle: &PuzzleMetaData, case: usize, solve: Solver) {
        let input = read_input(puzzle, case).unwrap();
        let result = solve(&input);
        if result.is_err() {
            assert!(false);
        }
        let ans = result.unwrap();
        let expected = get_expected(puzzle, case);
        if !expected.0.is_empty() && expected.0 != "0" {
            assert_eq!(&ans.0, &expected.0);
        }
        if !expected.1.is_empty() && expected.1 != "0" {
            assert_eq!(&ans.1, &expected.1);
        }
    }

    /// Helper function to test the checking for invalid puzzle input.
    pub fn test_invalid(_puzzle: &PuzzleMetaData, input: &[String], solve: Solver) {
        let result = solve(&input);
        assert!(result.is_err());
    }
}
