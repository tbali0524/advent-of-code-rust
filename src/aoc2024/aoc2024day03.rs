//! [aoc](https://adventofcode.com/2024/day/3)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 3,
        title: "Mull It Over",
        solution: ("174561379", "106921067"),
        example_solutions: vec![("161", "0"), ("0", "48")],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Part 1 + 2
    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut is_enabled = true;
    for &row in input {
        let dos = row.match_indices("do()").map(|(i, _)| (i, true));
        let donts = row.match_indices("don't()").map(|(i, _)| (i, false));
        let mut commands = dos.chain(donts).collect::<Vec<_>>();
        commands.sort_by_key(|&x| x.0);
        let mut start = 0;
        while start < row.len() {
            let next_start = row[start..].find("mul(");
            if next_start.is_none() {
                break;
            }
            start = next_start.unwrap() + start + 4;
            let next_comma = row[(start + 1)..].find(',');
            if next_comma.is_none() {
                break;
            }
            let comma = next_comma.unwrap() + start + 1;
            let next_close = row[(comma + 2)..].find(')');
            if next_close.is_none() {
                break;
            }
            let close = next_close.unwrap() + comma + 2;
            let next_op1 = row[start..comma].parse::<ItemType>();
            if next_op1.is_err() {
                continue;
            }
            let op1 = next_op1.unwrap();
            let next_op2 = row[(comma + 1)..close].parse::<ItemType>();
            if next_op2.is_err() {
                continue;
            }
            let op2 = next_op2.unwrap();
            ans1 += op1 * op2;
            let last_command = commands.iter().rfind(|(i, _)| *i < start);
            if last_command.is_some() {
                is_enabled = last_command.unwrap().1;
            }
            if is_enabled {
                ans2 += op1 * op2;
            }
            start = close + 1;
        }
        let last_command = commands.iter().last();
        if last_command.is_some() {
            is_enabled = last_command.unwrap().1;
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
}
