//! [aoc](https://adventofcode.com/2016/day/9)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2016,
        day: 9,
        title: "Explosives in Cyberspace",
        solution: ("74532", "11558231665"),
        example_solutions: vec![("57", "0"), ("0", "242394")], // 9 + 20 + 241920 + 445
    }
}

type ItemType = usize;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut data = Vec::new();
    for &row in input.iter() {
        for c in row.chars() {
            if c != ' ' {
                data.push(c);
            }
        }
    }
    // ---------- Part 1 + 2
    let ans1 = get_decompress_len(&data, false)?;
    let ans2 = get_decompress_len(&data, true)?;
    Ok((ans1.to_string(), ans2.to_string()))
}

fn get_decompress_len(data: &[char], recursive: bool) -> Result<usize, PuzzleError> {
    let mut ans = 0;
    let mut start = 0;
    while start < data.len() {
        let mut end = data[start..]
            .iter()
            .position(|&c| c == '(')
            .unwrap_or(data.len() - start)
            + start;
        if start != end {
            ans += end - start;
            start = end;
            continue;
        }
        start += 1;
        let Some(len_to_end) = data[start..].iter().position(|&c| c == ')') else {
            continue;
        };
        end = len_to_end + start;
        if end - start < 3 {
            Err("marker in parenthesis must be at least 3 chars long")?;
        }
        let a = data[start..end]
            .iter()
            .collect::<String>()
            .split('x')
            .map(|x| {
                x.parse::<ItemType>()
                    .map_err(|_| "marker must contain integers")
            })
            .collect::<Result<Vec<_>, _>>()?;
        if a.len() != 2 {
            Err("marker must contain 2 integers separated by `x`")?;
        }
        let len = a[0];
        let count = a[1];
        if recursive {
            ans += get_decompress_len(&data[(end + 1)..(end + 1 + len)], true)? * count;
        } else {
            ans += len * count;
        }
        start = end + 1 + len;
    }
    Ok(ans)
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
    fn invalid_marker_must_be_at_least_3_chars() {
        test_invalid_msg(
            &[&"A(1x)BC"],
            solve,
            "marker in parenthesis must be at least 3 chars long",
        );
    }

    #[test]
    fn invalid_marker_must_be_at_2_integers() {
        test_invalid_msg(&[&"A(1xa)BC"], solve, "marker must contain integers");
    }

    #[test]
    fn invalid_marker_must_be_separated_by_x() {
        test_invalid_msg(
            &[&"A(1x2x3)BC"],
            solve,
            "marker must contain 2 integers separated by `x`",
        );
    }
}
