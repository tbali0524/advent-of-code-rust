//! [aoc](https://adventofcode.com/2025/day/9)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashSet;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2025,
        day: 9,
        title: "Movie Theater",
        solution: ("4774877510", "1560475800"),
        example_solutions: vec![("50", "24")],
    }
}

type ItemType = i64;

#[expect(clippy::int_plus_one)]
pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut points = Vec::new();
    for &line in input.iter() {
        let mut line_iter = line.split(',');
        let x = line_iter
            .next()
            .ok_or("invalid input")?
            .parse::<ItemType>()
            .map_err(|_| "x coordinate must be an integer")?;
        let y = line_iter
            .next()
            .ok_or("missing y coordinate")?
            .parse::<ItemType>()
            .map_err(|_| "y coordinate must be an integer")?;
        if line_iter.next().is_some() {
            Err("there must be 2 comma-separated coordinates per line")?;
        }
        points.push((x, y));
    }
    // ---------- Part 1
    let mut ans1 = 0;
    for a in 0..points.len() {
        for b in (a + 1)..points.len() {
            let (ax, ay) = points[a];
            let (bx, by) = points[b];
            let area = ((ax - bx).abs() + 1) * ((ay - by).abs() + 1);
            if area > ans1 {
                ans1 = area;
            }
        }
    }
    // ---------- Part 2
    let mut x_set = HashSet::new();
    let mut y_set = HashSet::new();
    let mut point_set = HashSet::new();
    for &(x, y) in points.iter() {
        x_set.insert(x);
        y_set.insert(y);
        point_set.insert((x, y));
    }
    let mut horiz_lines = Vec::new();
    let mut vert_lines = Vec::new();
    for a in 0..points.len() {
        let b = (a + 1) % points.len();
        let (mut ax, mut ay) = points[a];
        let (mut bx, mut by) = points[b];
        if ax > bx {
            (ax, bx) = (bx, ax);
        }
        if ay > by {
            (ay, by) = (by, ay);
        }
        if ay == by {
            horiz_lines.push((ay, ax, bx));
        } else if ax == bx {
            vert_lines.push((ax, ay, by));
        } else {
            Err("input lines most be horizontal or vertical")?;
        }
    }
    let mut rect_top = HashSet::new();
    let mut rect_left = HashSet::new();
    let mut rect_inside = HashSet::new();
    let mut rect_topleft = HashSet::new();
    for &ax in x_set.iter() {
        for &ay in y_set.iter() {
            let mut has_top = false;
            if !x_set.contains(&(ax + 1)) {
                let mut crossing = 0;
                for &(y, x1, x2) in &horiz_lines {
                    if y < ay && x1 <= ax + 1 && ax + 1 <= x2 {
                        crossing += 1;
                    } else if y == ay && x1 <= ax + 1 && ax + 1 <= x2 {
                        has_top = true;
                    }
                }
                if crossing % 2 == 1 {
                    rect_top.insert((ax, ay));
                }
            } else {
                rect_top.insert((ax, ay));
            }
            let mut has_left = false;
            if !y_set.contains(&(ay + 1)) {
                let mut crossing = 0;
                for &(x, y1, y2) in &vert_lines {
                    if x < ax && y1 <= ay + 1 && ay + 1 <= y2 {
                        crossing += 1;
                    } else if x == ax && y1 <= ay + 1 && ay + 1 <= y2 {
                        has_left = true;
                    }
                }
                if crossing % 2 == 1 {
                    rect_left.insert((ax, ay));
                }
            } else {
                rect_left.insert((ax, ay));
            }
            if rect_left.contains(&(ax, ay)) != has_left || rect_top.contains(&(ax, ay)) != has_top
            {
                rect_inside.insert((ax, ay));
            }
            if has_top {
                rect_top.insert((ax, ay));
            }
            if has_left {
                rect_left.insert((ax, ay));
            }
            if point_set.contains(&(ax, ay))
                || rect_top.contains(&(ax, ay))
                || rect_left.contains(&(ax, ay))
            {
                rect_topleft.insert((ax, ay));
            }
        }
    }
    let mut ans2 = 0;
    for a in 0..points.len() {
        for b in (a + 1)..points.len() {
            let (mut ax, mut ay) = points[a];
            let (mut bx, mut by) = points[b];
            if ax > bx {
                (ax, bx) = (bx, ax);
            }
            if ay > by {
                (ay, by) = (by, ay);
            }
            let mut is_ok = true;
            'outer: for &px in x_set.iter() {
                for &py in y_set.iter() {
                    if px < ax || px > bx || py < ay || py > by {
                        continue;
                    }
                    if !rect_topleft.contains(&(px, py)) {
                        is_ok = false;
                        break 'outer;
                    }
                    if px < bx && !rect_top.contains(&(px, py)) {
                        is_ok = false;
                        break 'outer;
                    }
                    if py < by && !rect_left.contains(&(px, py)) {
                        is_ok = false;
                        break 'outer;
                    }
                    if px < bx && py < by && !rect_inside.contains(&(px, py)) {
                        is_ok = false;
                        break 'outer;
                    }
                }
            }
            if !is_ok {
                continue;
            }
            let area = ((ax - bx).abs() + 1) * ((ay - by).abs() + 1);
            if area > ans2 {
                ans2 = area;
            }
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
    fn invalid_x_must_be_integer() {
        test_invalid_msg(&[&"a,2"], solve, "x coordinate must be an integer");
    }

    #[test]
    fn invalid_y_must_be_integer() {
        test_invalid_msg(&[&"1,a"], solve, "y coordinate must be an integer");
    }

    #[test]
    fn invalid_missing_y() {
        test_invalid_msg(&[&"1"], solve, "missing y coordinate");
    }

    #[test]
    fn invalid_must_be_3_coordinates() {
        test_invalid_msg(
            &[&"1,2,3"],
            solve,
            "there must be 2 comma-separated coordinates per line",
        );
    }

    #[test]
    fn invalid_lines_must_be_horizontal_or_vertical() {
        test_invalid_msg(
            &[&"1,1", &"2,3"],
            solve,
            "input lines most be horizontal or vertical",
        );
    }
}
