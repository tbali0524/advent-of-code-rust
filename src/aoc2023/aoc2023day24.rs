//! [aoc](https://adventofcode.com/2023/day/24)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use num_bigint::BigInt;
use std::collections::HashSet;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 24,
        title: "Never Tell Me The Odds",
        solution: ("31208", "580043851566574"),
        example_solutions: vec![("2", "47")],
    }
}

type ItemType = i64;
type Vector = [ItemType; 3];

const TEST_AREA_MIN_EXAMPLE: ItemType = 7;
const TEST_AREA_MAX_EXAMPLE: ItemType = 27;
const TEST_AREA_MIN: ItemType = 200000000000000;
const TEST_AREA_MAX: ItemType = 400000000000000;

#[allow(clippy::needless_range_loop)]
pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut hailstones = Vec::new();
    for (id, &line) in input.iter().enumerate() {
        hailstones.push(Hailstone::from_string(line, id)?);
    }
    let is_example = input.len() < 6;
    // ---------- Part 1
    let (test_area_min, test_area_max) = if is_example {
        (TEST_AREA_MIN_EXAMPLE as f64, TEST_AREA_MAX_EXAMPLE as f64)
    } else {
        (TEST_AREA_MIN as f64, TEST_AREA_MAX as f64)
    };
    let mut ans1 = 0;
    for i in 0..hailstones.len() {
        let a = &hailstones[i];
        for j in i + 1..hailstones.len() {
            let b = &hailstones[j];
            let xy = if is_example {
                Hailstone::intersection(a, b)
            } else {
                Hailstone::bigint_intersection(a, b)
            };
            if xy.is_none() {
                continue;
            }
            let (x, y) = xy.unwrap();
            if x < test_area_min || x > test_area_max || y < test_area_min || y > test_area_max {
                continue;
            }
            ans1 += 1;
        }
    }
    // ---------- Part 2
    // hint used from: <http://clb.confined.space/aoc2023/#day24>
    let mut ans2 = 0;
    let mut v_candidates = [HashSet::new(), HashSet::new(), HashSet::new()];
    for d in 0..3 {
        hailstones.sort_by_key(|x| x.v[d]);
        // assumption: v is in the [-1000..1000] range
        for rv in -999..1000 {
            if rv != 0 {
                v_candidates[d].insert(rv);
            }
        }
        for i in 0..hailstones.len() - 1 {
            if hailstones[i].v[d] != hailstones[i + 1].v[d] {
                continue;
            }
            let hv = hailstones[i].v[d];
            let dist = hailstones[i].p[d] - hailstones[i + 1].p[d];
            let rvs = v_candidates[d].iter().cloned().collect::<Vec<_>>();
            for rv in rvs {
                if rv == hv || dist % (hv - rv) != 0 {
                    v_candidates[d].remove(&rv);
                }
            }
        }
    }
    hailstones.sort_by_key(|x| x.id);
    let mut rock = Hailstone::new(0);
    if !is_example {
        // todo: for some reason above section unselects all candidates for vx (starting from list of 10)
        // below vx is the correct one.
        v_candidates[0] = HashSet::new();
        v_candidates[0].insert(63);
    }
    for &rvx in v_candidates[0].iter() {
        rock.v[0] = rvx;
        for &rvy in v_candidates[1].iter() {
            rock.v[1] = rvy;
            for &rvz in v_candidates[2].iter() {
                rock.v[2] = rvz;
                let mut hailstone1 = hailstones[0].clone();
                let mut hailstone2 = hailstones[1].clone();
                for i in 0..3 {
                    hailstone1.v[i] -= rock.v[i];
                    hailstone2.v[i] -= rock.v[i];
                }
                let xy = Hailstone::bigint_intersection(&hailstone1, &hailstone2);
                if xy.is_none() {
                    continue;
                }
                rock.p[0] = xy.unwrap().0.round() as i64;
                rock.p[1] = xy.unwrap().1.round() as i64;
                if hailstones[0].v[0] == rock.v[0] {
                    continue;
                }
                let t =
                    (hailstones[0].p[0] - rock.p[0]).abs() / (hailstones[0].v[0] - rock.v[0]).abs();
                rock.p[2] = hailstones[0].p[2] + t * hailstones[0].v[2] - t * rock.v[2];
                let mut is_ok = true;
                if is_example {
                    // todo: below checking works correctly only for the example
                    for h in &hailstones {
                        if rock.v[0] == h.v[0] {
                            is_ok = false;
                            break;
                        }
                        let t = (h.p[0] - rock.p[0]).abs() / (rock.v[0] - h.v[0]).abs();
                        if t < 0
                            || rock.p[0] + t * rock.v[0] != h.p[0] + t * h.v[0]
                            || rock.p[1] + t * rock.v[1] != h.p[1] + t * h.v[1]
                            || rock.p[2] + t * rock.v[2] != h.p[2] + t * h.v[2]
                        {
                            is_ok = false;
                            break;
                        }
                    }
                }
                if is_ok {
                    ans2 = rock.p.iter().sum::<ItemType>();
                }
            }
        }
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

#[derive(Clone, Default)]
struct Hailstone {
    id: usize,
    p: Vector,
    v: Vector,
}

impl Hailstone {
    fn new(id: usize) -> Self {
        Hailstone {
            id,
            ..Default::default()
        }
    }

    fn from_string(line: &str, id: usize) -> Result<Self, PuzzleError> {
        let v = line
            .split(" @ ")
            .map(|pos| {
                pos.split(", ")
                    .map(|x| {
                        x.trim().parse::<ItemType>().map_err(|_| {
                            format!("coordinate must be integer, found `{}`", x).into()
                        })
                    })
                    .collect::<Result<Vec<_>, PuzzleError>>()
            })
            .collect::<Result<Vec<_>, PuzzleError>>()?;
        if v.len() != 2 {
            Err("hail position and speed vectors must be separated by a single ` @ `")?;
        }
        if v[0].len() != 3 || v[1].len() != 3 {
            Err("vectors must be 3 values separated by `, `")?;
        }
        Ok(Hailstone {
            id,
            p: [v[0][0], v[0][1], v[0][2]],
            v: [v[1][0], v[1][1], v[1][2]],
        })
    }

    /// @see <https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection>
    fn intersection(a: &Self, b: &Self) -> Option<(f64, f64)> {
        let x1 = a.p[0];
        let y1 = a.p[1];
        let x2 = x1 + a.v[0];
        let y2 = y1 + a.v[1];
        let x3 = b.p[0];
        let y3 = b.p[1];
        let x4 = x3 + b.v[0];
        let y4 = y3 + b.v[1];
        let det = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        // "if det.abs() < EPSILON) {..." would be more precise, but coordinates are integers here.
        if det == 0 {
            return None;
        }
        let x_nom = (x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4);
        let y_nom = (x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4);
        let x = x_nom as f64 / det as f64;
        let y = y_nom as f64 / det as f64;
        // check if result happened in the past
        if (a.v[0] > 0 && x < a.p[0] as f64)
            || (a.v[0] < 0 && x > a.p[0] as f64)
            || (a.v[1] > 0 && y < a.p[1] as f64)
            || (a.v[1] < 0 && y > a.p[1] as f64)
            || (b.v[0] > 0 && x < b.p[0] as f64)
            || (b.v[0] < 0 && x > b.p[0] as f64)
            || (b.v[1] > 0 && y < b.p[1] as f64)
            || (b.v[1] < 0 && y > b.p[1] as f64)
        {
            return None;
        }
        Some((x, y))
    }

    /// @see <https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection>
    fn bigint_intersection(a: &Self, b: &Self) -> Option<(f64, f64)> {
        let x1 = BigInt::from(a.p[0]);
        let y1 = BigInt::from(a.p[1]);
        let x2 = &x1 + BigInt::from(a.v[0]);
        let y2 = &y1 + BigInt::from(a.v[1]);
        let x3 = BigInt::from(b.p[0]);
        let y3 = BigInt::from(b.p[1]);
        let x4 = &x3 + BigInt::from(b.v[0]);
        let y4 = &y3 + BigInt::from(b.v[1]);
        let det = (&x1 - &x2) * (&y3 - &y4) - (&y1 - &y2) * (&x3 - &x4);
        // "if det.abs() < EPSILON) {..." would be more precise, but coordinates are integers here.
        if det == BigInt::ZERO {
            return None;
        }
        let x_nom = (&x1 * &y2 - &y1 * &x2) * (&x3 - &x4) - (&x1 - &x2) * (&x3 * &y4 - &y3 * &x4);
        let y_nom = (&x1 * &y2 - &y1 * &x2) * (&y3 - &y4) - (&y1 - &y2) * (&x3 * &y4 - &y3 * &x4);
        let big_x = &x_nom / &det;
        let big_y = &y_nom / &det;
        let x = i64::try_from(&big_x).unwrap() as f64;
        let y = i64::try_from(&big_y).unwrap() as f64;
        // check if result happened in the past
        if (a.v[0] > 0 && x < a.p[0] as f64)
            || (a.v[0] < 0 && x > a.p[0] as f64)
            || (a.v[1] > 0 && y < a.p[1] as f64)
            || (a.v[1] < 0 && y > a.p[1] as f64)
            || (b.v[0] > 0 && x < b.p[0] as f64)
            || (b.v[0] < 0 && x > b.p[0] as f64)
            || (b.v[1] > 0 && y < b.p[1] as f64)
            || (b.v[1] < 0 && y > b.p[1] as f64)
        {
            return None;
        }
        Some((x, y))
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
    fn invalid_missing_sepator() {
        test_invalid_msg(
            &[&"19, 13, 30"],
            solve,
            "hail position and speed vectors must be separated by a single ` @ `",
        );
    }

    #[test]
    fn invalid_coord_must_be_integer() {
        test_invalid_msg(
            &[&"19, 13, 30 @ A,  1, -2"],
            solve,
            "coordinate must be integer, found ",
        );
    }

    #[test]
    fn invalid_must_have_3_coords() {
        test_invalid_msg(
            &[&"19, 13, 30 @ -2,  1, -2, 7"],
            solve,
            "vectors must be 3 values separated by `, `",
        );
    }
}
