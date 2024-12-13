//! [aoc](https://adventofcode.com/2024/day/13)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 13,
        title: "Claw Contraption",
        solution: ("26599", "106228669504887"),
        example_solutions: vec![("480", "0")],
    }
}

type ItemType = i64;

const COST_A: ItemType = 3;
const COST_B: ItemType = 1;
const P_BASE_PART2: ItemType = 10000000000000;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut machines = Vec::new();
    let mut i = 0;
    while i + 2 < input.len() {
        machines.push(ClawMachine::new(&input[i..=(i + 2)])?);
        if i + 3 < input.len() && !input[i + 3].is_empty() {
            Err("claw machine definitions must be separated by an empty line")?;
        }
        i += 4;
    }
    // ---------- Part 1 + 2
    let mut ans1 = 0;
    let mut ans2 = 0;
    for machine in &machines {
        ans1 += machine.cost(0);
        ans2 += machine.cost(P_BASE_PART2);
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

struct ClawMachine {
    ax: ItemType,
    ay: ItemType,
    bx: ItemType,
    by: ItemType,
    px: ItemType,
    py: ItemType,
}

impl ClawMachine {
    fn new(input: PuzzleInput) -> Result<Self, PuzzleError> {
        if input.len() < 3 {
            Err("missing line in claw machine definition")?;
        }
        if !input[0].starts_with("Button A: X+")
            || !input[1].starts_with("Button B: X+")
            || !input[2].starts_with("Prize: X=")
        {
            Err("invalid claw machine definition")?;
        }
        let axy = input[0][12..]
            .split(", Y+")
            .map(|x| {
                x.parse::<ItemType>().map_err(|_| {
                    format!("button config must be non-negative integer, found `{}`", x)
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let bxy = input[1][12..]
            .split(", Y+")
            .map(|x| {
                x.parse::<ItemType>().map_err(|_| {
                    format!("button config must be non-negative integer, found `{}`", x)
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let pxy = input[2][9..]
            .split(", Y=")
            .map(|x| {
                x.parse::<ItemType>().map_err(|_| {
                    format!(
                        "prize coordinates must be non-negative integers, found `{}`",
                        x
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        if axy.len() != 2 || bxy.len() != 2 || pxy.len() != 2 {
            Err("invalid claw machine definition")?;
        }
        Ok(Self {
            ax: axy[0],
            ay: axy[1],
            bx: bxy[0],
            by: bxy[1],
            px: pxy[0],
            py: pxy[1],
        })
    }

    /// Calculates the cost of a solution, or 0 if no solution exists.
    ///
    /// Analytic solution:
    ///
    /// ```txt
    /// a AX + b BX = PX
    /// a AY + b BY = PY
    ///
    /// a (AX / BX - AY / BY) = (PX / BX - PY / BY)
    ///
    /// a = (PX BY - PY BX) / (AX BY - AY BY)
    /// b = (PX - a AX) / BX
    /// solution is valid iff a and b are integers
    /// ```
    fn cost(&self, pbase: ItemType) -> ItemType {
        let nom_a = (pbase + self.px) * self.by - (pbase + self.py) * self.bx;
        let den_a = self.ax * self.by - self.ay * self.bx;
        if den_a == 0 || nom_a % den_a != 0 {
            return 0;
        }
        let a = nom_a / den_a;
        let b = (pbase + self.px - a * self.ax) / self.bx;
        if a * self.ax + b * self.bx != pbase + self.px
            || a * self.ay + b * self.by != pbase + self.py
        {
            return 0;
        }
        a * COST_A + b * COST_B
    }
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
    fn invalid_claw_def() {
        test_invalid_msg(
            &[
                &"Button A: X+1, Y+2",
                &"Button C: X+3, Y+4",
                &"Prize: X=5, Y=6",
            ],
            solve,
            "invalid claw machine definition",
        );
    }

    #[test]
    fn invalid_button_a_config_must_be_integer() {
        test_invalid_msg(
            &[
                &"Button A: X+1, Y+ZZ",
                &"Button B: X+3, Y+4",
                &"Prize: X=5, Y=6",
            ],
            solve,
            "button config must be non-negative integer",
        );
    }

    #[test]
    fn invalid_button_b_config_must_be_integer() {
        test_invalid_msg(
            &[
                &"Button A: X+1, Y+2",
                &"Button B: X+Q, Y+4",
                &"Prize: X=5, Y=6",
            ],
            solve,
            "button config must be non-negative integer",
        );
    }

    #[test]
    fn invalid_prize_coord_must_be_integer() {
        test_invalid_msg(
            &[
                &"Button A: X+1, Y+2",
                &"Button B: X+3, Y+4",
                &"Prize: X=P, Y=6",
            ],
            solve,
            "prize coordinates must be non-negative integers",
        );
    }

    #[test]
    fn invalid_incorrect_number_of_values() {
        test_invalid_msg(
            &[&"Button A: X+1, Y+2", &"Button B: X+3", &"Prize: X=5, Y=6"],
            solve,
            "invalid claw machine definition",
        );
    }

    #[test]
    fn invalid_claw_separator_must_be_empty_line() {
        test_invalid_msg(
            &[
                &"Button A: X+1, Y+2",
                &"Button B: X+3, Y+4",
                &"Prize: X=5, Y=6",
                &"a",
            ],
            solve,
            "claw machine definitions must be separated by an empty line",
        );
    }
}
