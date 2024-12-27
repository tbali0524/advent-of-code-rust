//! [aoc](https://adventofcode.com/2023/day/13)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 13,
        title: "Point of Incidence",
        solution: ("29165", "32192"),
        example_solutions: vec![("405", "400")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut patterns = Vec::new();
    let mut start = 0;
    while start < input.len() {
        let mut to = start;
        while to < input.len() && !input[to].is_empty() {
            to += 1;
        }
        let pattern = Pattern::new(&input[start..to])?;
        patterns.push(pattern);
        start = to + 1;
    }
    // ---------- Part 1
    let ans1 = patterns.iter().map(|x| x.note()).sum::<u64>();
    // ---------- Part 2
    let ans2 = patterns.iter_mut().map(|x| x.smudge_note()).sum::<u64>();
    Ok((ans1.to_string(), ans2.to_string()))
}

struct Pattern {
    max_x: usize,
    max_y: usize,
    rows: Vec<u64>,
    columns: Vec<u64>,
}

impl Pattern {
    #[expect(clippy::needless_range_loop)]
    fn new(grid: &[&str]) -> Result<Self, PuzzleError> {
        let max_y = grid.len();
        if max_y == 0 {
            Err("empty pattern")?;
        }
        let max_x = grid[0].len();
        if max_y >= 64 || max_x >= 64 {
            Err("pattern too large")?;
        }
        let mut rows = Vec::new();
        for y in 0..max_y {
            let mut item = 0_u64;
            if grid[y].len() != max_x {
                Err("pattern must be rectangular")?;
            }
            for x in 0..max_x {
                match grid[y].as_bytes()[x] {
                    b'.' => (),
                    b'#' => item |= 1 << x,
                    _ => Err("invalid character in pattern")?,
                }
            }
            rows.push(item);
        }
        let mut columns = Vec::new();
        for x in 0..max_x {
            let mut item = 0_u64;
            for y in 0..max_y {
                if grid[y].as_bytes()[x] == b'#' {
                    item |= 1 << y;
                }
            }
            columns.push(item);
        }
        Ok(Self {
            max_x,
            max_y,
            rows,
            columns,
        })
    }

    fn note(&self) -> u64 {
        100 * single_dir_reflections(&self.rows, -1) + single_dir_reflections(&self.columns, -1)
    }

    fn smudge_note(&mut self) -> u64 {
        let note_r = single_dir_reflections(&self.rows, -1);
        let note_c = single_dir_reflections(&self.columns, -1);
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                self.rows[y] ^= 1 << x;
                self.columns[x] ^= 1 << y;
                let smudge_note_r = single_dir_reflections(&self.rows, note_r as i64);
                let smudge_note_c = single_dir_reflections(&self.columns, note_c as i64);
                let smudge_note = 100 * smudge_note_r + smudge_note_c;
                self.rows[y] ^= 1 << x;
                self.columns[x] ^= 1 << y;
                if smudge_note != 0 {
                    return smudge_note;
                }
            }
        }
        0
    }
}

fn single_dir_reflections(lines: &[u64], original: i64) -> u64 {
    for pos in 0..(lines.len() - 1) {
        let mut is_ok = true;
        for i in 0..lines.len() {
            if pos < i || pos + 1 + i >= lines.len() {
                break;
            }
            if lines[pos - i] != lines[pos + 1 + i] {
                is_ok = false;
                break;
            }
        }
        if is_ok && (pos as i64) + 1 != original {
            return pos as u64 + 1;
        }
    }
    0
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
    fn invalid_must_be_rectangular() {
        test_invalid_msg(&[&".#", &"#.#"], solve, "pattern must be rectangular");
    }

    #[test]
    fn invalid_must_contain_only_valid_chars() {
        test_invalid_msg(&[&".#", &"#a"], solve, "invalid character in pattern");
    }
}
