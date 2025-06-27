//! [aoc](https://adventofcode.com/2023/day/22)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashSet;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 22,
        title: "Sand Slabs",
        solution: ("434", "61209"),
        example_solutions: vec![("5", "7")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut bricks = Vec::new();
    for (id, &line) in input.iter().enumerate() {
        bricks.push(Brick::from_string(line, id)?);
    }
    // ---------- Part 1 + 2
    let mut area = Area::new(&bricks)?;
    area.freefall();
    area.calculate_supports();
    let ans1 = area.count_disintegratable();
    let ans2 = area.calculate_chain_destructions();
    Ok((ans1.to_string(), ans2.to_string()))
}

#[derive(Clone)]
struct Brick {
    id: usize,
    from: [usize; 3],
    to: [usize; 3],
}

impl Brick {
    fn from_string(line: &str, id: usize) -> Result<Self, PuzzleError> {
        let v = line
            .split('~')
            .map(|pos| {
                pos.split(',')
                    .map(|x| {
                        x.trim().parse::<usize>().map_err(|_| {
                            format!("coordinate must be non-negative integer, found `{x}`").into()
                        })
                    })
                    .collect::<Result<Vec<_>, PuzzleError>>()
            })
            .collect::<Result<Vec<_>, PuzzleError>>()?;
        if v.len() != 2 {
            Err("brick coordinates must be separated by a single `~`")?;
        }
        if v[0].len() != 3 || v[1].len() != 3 {
            Err("coordinates must be 3 values separated by `,`")?;
        }
        if v[0][0] > v[1][0] || v[0][1] > v[1][1] || v[0][2] > v[1][2] {
            Err("to coordinates must not be smaller than from coordinates")?;
        }
        if v[0][2] == 0 {
            Err("from z coordinates must not be zero")?;
        }
        Ok(Brick {
            id,
            from: [v[0][0], v[0][1], v[0][2]],
            to: [v[1][0], v[1][1], v[1][2]],
        })
    }
}

#[derive(Default)]
struct Area {
    bricks: Vec<Brick>,
    grid: Vec<Vec<Vec<Option<usize>>>>,
    supporting: Vec<HashSet<usize>>,
    supported_by: Vec<HashSet<usize>>,
}

impl Area {
    #[expect(clippy::needless_range_loop)]
    fn new(from_bricks: &[Brick]) -> Result<Self, PuzzleError> {
        let bricks = from_bricks.to_vec();
        let max_x = bricks.iter().map(|b| b.to[0]).max().unwrap_or_default();
        let max_y = bricks.iter().map(|b| b.to[1]).max().unwrap_or_default();
        let max_z = bricks.iter().map(|b| b.to[2]).max().unwrap_or_default();
        let mut grid = vec![vec![vec![None; max_z + 2]; max_y + 1]; max_x + 1];
        for (id, b) in bricks.iter().enumerate() {
            for x in b.from[0]..=b.to[0] {
                for y in b.from[1]..=b.to[1] {
                    for z in b.from[2]..=b.to[2] {
                        if grid[x][y][z].is_some() {
                            Err("overlapping bricks")?;
                        }
                        grid[x][y][z] = Some(id);
                    }
                }
            }
        }
        let mut supporting = Vec::new();
        let mut supported_by = Vec::new();
        for _ in 0..bricks.len() {
            supporting.push(HashSet::new());
            supported_by.push(HashSet::new());
        }
        Ok(Area {
            bricks,
            grid,
            supporting,
            supported_by,
        })
    }

    fn freefall(&mut self) {
        let mut sorted_bricks = self.bricks.clone();
        sorted_bricks.sort_by_key(|b| b.from[2]);
        for b in &sorted_bricks {
            let mut new_z = 0;
            'brick: for z in (0..b.from[2]).rev() {
                for x in b.from[0]..=b.to[0] {
                    for y in b.from[1]..=b.to[1] {
                        if self.grid[x][y][z].is_some() {
                            new_z = z + 1;
                            break 'brick;
                        }
                    }
                }
            }
            if new_z != b.from[2] {
                self.fall_brick_to_z(b.id, new_z);
            }
        }
    }

    fn fall_brick_to_z(&mut self, id: usize, new_z: usize) {
        let b = &self.bricks[id];
        let fall_by = b.from[2] - new_z;
        for x in b.from[0]..=b.to[0] {
            for y in b.from[1]..=b.to[1] {
                for z in b.from[2]..=b.to[2] {
                    self.grid[x][y][z] = None;
                }
                for z in (b.from[2] - fall_by)..=(b.to[2] - fall_by) {
                    self.grid[x][y][z] = Some(b.id);
                }
            }
        }
        self.bricks[id].from[2] -= fall_by;
        self.bricks[id].to[2] -= fall_by;
    }

    fn calculate_supports(&mut self) {
        self.supporting = Vec::new();
        self.supported_by = Vec::new();
        for _ in 0..self.bricks.len() {
            self.supporting.push(HashSet::new());
            self.supported_by.push(HashSet::new());
        }
        for (id, b) in self.bricks.iter().enumerate() {
            for x in b.from[0]..=b.to[0] {
                for y in b.from[1]..=b.to[1] {
                    if let Some(id_above) = self.grid[x][y][b.to[2] + 1] {
                        self.supporting[id].insert(id_above);
                        self.supported_by[id_above].insert(id);
                    }
                }
            }
        }
    }

    fn count_disintegratable(&self) -> usize {
        let mut ans = 0;
        for i in 0..self.bricks.len() {
            let mut is_ok = true;
            for &id_above in &self.supporting[i] {
                if self.supported_by[id_above].len() == 1 {
                    is_ok = false;
                    break;
                }
            }
            if is_ok {
                ans += 1;
            }
        }
        ans
    }

    fn calculate_chain_destructions(&self) -> usize {
        let mut ans = 0;
        for i in 0..self.bricks.len() {
            let mut supported_by = self.supported_by.clone();
            let mut q = Vec::new();
            q.push(i);
            let mut read_idx = 0;
            while read_idx < q.len() {
                let id = q[read_idx];
                read_idx += 1;
                for &id_above in &self.supporting[id] {
                    supported_by[id_above].remove(&id);
                    if supported_by[id_above].is_empty() {
                        q.push(id_above);
                    }
                }
            }
            ans += read_idx - 1;
        }
        ans
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
    fn invalid_missing_tilde() {
        test_invalid_msg(
            &["1,2,3"],
            solve,
            "brick coordinates must be separated by a single `~`",
        );
    }

    #[test]
    fn invalid_coord_must_be_integer() {
        test_invalid_msg(
            &["1,2,3~4,A,6"],
            solve,
            "coordinate must be non-negative integer, found ",
        );
    }

    #[test]
    fn invalid_must_have_3_coords() {
        test_invalid_msg(
            &["1,2,3~4,5,6,7"],
            solve,
            "coordinates must be 3 values separated by `,`",
        );
    }

    #[test]
    fn invalid_to_must_be_higher_than_from() {
        test_invalid_msg(
            &["1,2,3~4,5,1"],
            solve,
            "to coordinates must not be smaller than from coordinates",
        );
    }

    #[test]
    fn invalid_from_z_must_not_be_zero() {
        test_invalid_msg(
            &["1,2,0~4,5,6"],
            solve,
            "from z coordinates must not be zero",
        );
    }
}
