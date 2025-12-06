//! [aoc](https://adventofcode.com/2025/day/6)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2025,
        day: 6,
        title: "Trash Compactor",
        solution: ("6378679666679", "11494432585168"),
        example_solutions: vec![("4277556", "3263827")],
    }
}

type ItemType = u64;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() < 3 {
        Err("input must have at least 3 lines")?;
    }
    let mut operators = Vec::new();
    let mut positions = Vec::new();
    let mut after_op: bool = false;
    for (pos, c) in input.last().unwrap().chars().enumerate() {
        match c {
            ' ' => after_op = false,
            '+' | '*' => {
                if after_op {
                    Err("operators in last line must be space separated")?;
                }
                operators.push(c);
                positions.push(pos);
                after_op = true;
            }
            _ => Err("last line must contain only +, * and space")?,
        }
    }
    positions.push(input.last().unwrap().len() + 1);
    let mut columns = vec![Vec::new(); operators.len()];
    let mut digits = Vec::new();
    for &row in input.iter().take(input.len() - 1) {
        let row_digits = row.chars().collect::<Vec<_>>();
        digits.push(row_digits);
        for i in 0..positions.len() - 1 {
            let v = row[positions[i]..positions[i + 1] - 1]
                .trim()
                .parse::<ItemType>()
                .map_err(|_| "input lines except last line must contain only integers")?;
            columns[i].push(v);
        }
    }
    // ---------- Part 1
    let mut ans1 = 0;
    for (i, column) in columns.iter().enumerate() {
        let mut partial = match operators[i] {
            '+' => 0,
            '*' => 1,
            _ => unreachable!(),
        };
        for v in column.iter() {
            partial = match operators[i] {
                '+' => partial + v,
                '*' => partial * v,
                _ => unreachable!(),
            };
        }
        ans1 += partial;
    }
    // ---------- Part 2
    let mut ans2 = 0;
    for i in 0..positions.len() - 1 {
        let mut partial = match operators[i] {
            '+' => 0,
            '*' => 1,
            _ => unreachable!(),
        };
        for x in positions[i]..positions[i + 1] - 1 {
            let mut v = 0;
            for digits_row in &digits {
                let c = digits_row[x];
                if c == ' ' && v == 0 {
                    continue;
                }
                if c == ' ' && v != 0 {
                    break;
                }
                let digit = c.to_digit(10).unwrap() as ItemType;
                v = 10 * v + digit;
            }
            partial = match operators[i] {
                '+' => partial + v,
                '*' => partial * v,
                _ => unreachable!(),
            };
        }
        ans2 += partial;
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
    fn invalid_must_have_at_least_3_lines() {
        test_invalid_msg(&[&"+ *"], solve, "input must have at least 3 lines");
    }

    #[test]
    fn invalid_last_line_must_contain_operator() {
        test_invalid_msg(
            &[&"1 2", &"3 4", &"+ a"],
            solve,
            "last line must contain only +, * and space",
        );
    }

    #[test]
    fn invalid_last_line_operators_must_be_space_separated() {
        test_invalid_msg(
            &[&"1 2", &"3 4", &"+*"],
            solve,
            "operators in last line must be space separated",
        );
    }

    #[test]
    fn invalid_must_contain_integer() {
        test_invalid_msg(
            &[&"1 a", &"3 4", &"+ *"],
            solve,
            "input lines except last line must contain only integers",
        );
    }
}
