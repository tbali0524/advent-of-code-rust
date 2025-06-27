//! [aoc](https://adventofcode.com/2023/day/6)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 6,
        title: "Wait For It",
        solution: ("3316275", "27102791"),
        example_solutions: vec![("288", "71503")],
    }
}

type ItemType = i64;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() != 2
        || !input[0].starts_with("Time:      ")
        || !input[1].starts_with("Distance:  ")
    {
        Err("input must be 2 lines, first starting with `Time:`, second with `Distance:`")?;
    }
    let mut times = input[0][11..]
        .split_whitespace()
        .map(|x| {
            x.trim()
                .parse::<ItemType>()
                .map_err(|_| format!("times must be integers, found `{x}`").into())
        })
        .collect::<Result<Vec<_>, PuzzleError>>()?;
    let mut distances = input[1][11..]
        .split_whitespace()
        .map(|x| {
            x.trim()
                .parse::<ItemType>()
                .map_err(|_| format!("distances must be integers, found `{x}`").into())
        })
        .collect::<Result<Vec<_>, PuzzleError>>()?;
    if times.len() != distances.len() {
        Err("time and distance lists must contain same number of elements")?;
    }
    times.push(
        input[0][11..]
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("")
            .parse::<ItemType>()
            .map_err(|_| "cannot parse concatenation of times")?,
    );
    distances.push(
        input[1][11..]
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("")
            .parse::<ItemType>()
            .map_err(|_| "cannot parse concatenation of times")?,
    );
    // ---------- Part 1 + 2
    let mut ans1 = 1;
    let mut ans2 = 0;
    for (idx, &t) in times.iter().enumerate() {
        let d = distances[idx];
        // solve for: x(t-x)>d
        let discriminant = t.pow(2) - 4 * d;
        if discriminant < 0 {
            ans1 = 0;
            break;
        }
        let root_disc = (discriminant as f64).sqrt();
        let mut x1 = (t as f64 - root_disc) / 2.0;
        let mut x2 = (t as f64 + root_disc) / 2.0;
        if x1 <= 0.0 || x2 <= 0.0 {
            ans1 = 0;
            break;
        }
        if x1 % 1.0 != 0.0 {
            x1 = x1.ceil();
        } else {
            x1 = x1.floor() + 1.0;
        }
        if x2 % 1.0 != 0.0 {
            x2 = x2.floor();
        } else {
            x2 = x2.floor() - 1.0;
        }
        if x2 < x1 {
            ans1 = 0;
            break;
        }
        if idx < times.len() - 1 {
            ans1 *= (x2 - x1 + 1.0) as ItemType;
        } else {
            ans2 = (x2 - x1 + 1.0) as ItemType;
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
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_must_be_2_lines() {
        test_invalid_msg(
            &["Time:      7  15   30", "Distance:  9  40  200", "a"],
            solve,
            "input must be 2 lines",
        );
    }

    #[test]
    fn invalid_first_line_must_start_with_time() {
        test_invalid_msg(
            &["a:      7  15   30", "Distance:  9  40  200"],
            solve,
            "first starting with `Time:`",
        );
    }

    #[test]
    fn invalid_second_line_must_start_with_distance() {
        test_invalid_msg(
            &["Time:      7  15   30", "a:  9  40  200"],
            solve,
            "second with `Distance:`",
        );
    }

    #[test]
    fn invalid_times_must_be_integer() {
        test_invalid_msg(
            &["Time:      7  a   30", "Distance:  9  40  200"],
            solve,
            "times must be integers",
        );
    }

    #[test]
    fn invalid_distances_must_be_integer() {
        test_invalid_msg(
            &["Time:      7  15   30", "Distance:  9  a  200"],
            solve,
            "distances must be integers",
        );
    }
}
