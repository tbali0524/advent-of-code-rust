//! [aoc](https://adventofcode.com/2015/day/6)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 6,
        title: "Probably a Fire Hazard",
        solution: ("377891", "14110788"),
        example_solutions: vec![("1000000", "2000000"), ("1", "1")],
    }
}

type ItemType = usize;

const GRID_SIZE: usize = 1000;

#[allow(clippy::needless_range_loop)]
pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut instructions = Vec::new();
    for line in input {
        instructions.push(Instruction::try_from(*line)?);
    }
    // ---------- Part 1
    let mut grid = vec![[0usize; GRID_SIZE]; GRID_SIZE];
    for instr in &instructions {
        for y in instr.y0..=instr.y1 {
            for x in instr.x0..=instr.x1 {
                grid[y][x] = match instr.verb.as_str() {
                    "toggle" => 1 - grid[y][x],
                    "turn on" => 1,
                    "turn off" => 0,
                    _ => return Err(PuzzleError("invalid instruction verb".into())),
                };
            }
        }
    }
    let ans1 = grid
        .iter()
        .map(|row| row.iter().sum::<ItemType>())
        .sum::<ItemType>();
    // ---------- Part 2
    let mut grid = vec![[0usize; GRID_SIZE]; GRID_SIZE];
    for instr in &instructions {
        for y in instr.y0..=instr.y1 {
            for x in instr.x0..=instr.x1 {
                grid[y][x] = match instr.verb.as_str() {
                    "toggle" => grid[y][x] + 2,
                    "turn on" => grid[y][x] + 1,
                    "turn off" => {
                        if grid[y][x] > 0 {
                            grid[y][x] - 1
                        } else {
                            0
                        }
                    }
                    _ => return Err(PuzzleError("invalid instruction verb".into())),
                };
            }
        }
    }
    let ans2 = grid
        .iter()
        .map(|row| row.iter().sum::<ItemType>())
        .sum::<ItemType>();
    Ok((ans1.to_string(), ans2.to_string()))
}

struct Instruction {
    verb: String,
    x0: ItemType,
    y0: ItemType,
    x1: ItemType,
    y1: ItemType,
}

impl TryFrom<&str> for Instruction {
    type Error = PuzzleError;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let count_verb_words = if line.starts_with("turn") { 2 } else { 1 };
        let a = line.split(' ').collect::<Vec<_>>();
        if a.len() != 3 + count_verb_words {
            return Err(PuzzleError(
                "incorrect number of words in input line".into(),
            ));
        }
        let b = a[count_verb_words].split(',').collect::<Vec<_>>();
        let c = a[count_verb_words + 2].split(',').collect::<Vec<_>>();
        if b.len() != 2 || c.len() != 2 || a[1 + count_verb_words] != "through" {
            return Err(PuzzleError(
                "x,y positions must be separated by , and through".into(),
            ));
        }
        let verb = if count_verb_words == 1 {
            a[0].to_owned()
        } else {
            a[0].to_owned() + " " + a[1]
        };
        Ok(Self {
            verb,
            x0: b[0]
                .parse::<ItemType>()
                .map_err(|_| PuzzleError("position must be an integer".into()))?,
            y0: b[1]
                .parse::<ItemType>()
                .map_err(|_| PuzzleError("position must be an integer".into()))?,
            x1: c[0]
                .parse::<ItemType>()
                .map_err(|_| PuzzleError("position must be an integer".into()))?,
            y1: c[1]
                .parse::<ItemType>()
                .map_err(|_| PuzzleError("position must be an integer".into()))?,
        })
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
    fn example2() {
        test_case(metadata, solve, 2);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_number_of_words() {
        test_invalid(&[&"a"], solve);
    }

    #[test]
    fn invalid_position_number() {
        test_invalid(&[&"turn on 0,0,1 through 0,0"], solve);
    }

    #[test]
    fn invalid_positions_must_be_integers() {
        test_invalid(&[&"turn on 0,a through 0,0"], solve);
    }

    #[test]
    fn invalid_verb() {
        test_invalid(&[&"discard 0,0 through 0,0"], solve);
    }
}
