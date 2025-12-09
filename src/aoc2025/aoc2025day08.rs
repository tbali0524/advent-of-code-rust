//! [aoc](https://adventofcode.com/2025/day/8)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2025,
        day: 8,
        title: "Playground",
        solution: ("42315", "8079278220"),
        example_solutions: vec![("40", "25272")],
    }
}

type ItemType = i64;

const CONNECTIONS_PART1_EXAMPLE: usize = 10;
const CONNECTIONS_PART1: usize = 1000;

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
        let z = line_iter
            .next()
            .ok_or("missing z coordinate")?
            .parse::<ItemType>()
            .map_err(|_| "z coordinate must be an integer")?;
        if line_iter.next().is_some() {
            Err("there must be 3 comma-separated coordinates per line")?;
        }
        points.push((x, y, z));
    }
    // ---------- Part 1
    let mut distances = Vec::new();
    for a in 0..points.len() {
        for b in (a + 1)..points.len() {
            let (ax, ay, az) = points[a];
            let (bx, by, bz) = points[b];
            let dist2 = (ax - bx) * (ax - bx) + (ay - by) * (ay - by) + (az - bz) * (az - bz);
            distances.push((dist2, a, b));
        }
    }
    distances.sort_by_key(|d| d.0);
    let connections_part1 = if points.len() < 25 {
        CONNECTIONS_PART1_EXAMPLE
    } else {
        CONNECTIONS_PART1
    };
    let mut adj_list = vec![Vec::<usize>::new(); points.len()];
    for &(_, a, b) in distances.iter().take(connections_part1) {
        adj_list[a].push(b);
        adj_list[b].push(a);
    }
    let mut visited = vec![false; points.len()];
    let mut component_sizes = Vec::new();
    let mut count_components = 0;
    for from in 0..points.len() {
        if visited[from] {
            continue;
        }
        count_components += 1;
        component_sizes.push(1);
        let mut stack = Vec::new();
        stack.push(from);
        visited[from] = true;
        while let Some(p) = stack.pop() {
            for &next in adj_list[p].iter() {
                if visited[next] {
                    continue;
                }
                visited[next] = true;
                stack.push(next);
                component_sizes[count_components - 1] += 1;
            }
        }
    }
    component_sizes.sort_by(|a, b| b.cmp(a));
    if component_sizes.len() < 3 {
        Err("invalid input, there are less than 3 components")?;
    }
    let ans1 = component_sizes[0] * component_sizes[1] * component_sizes[2];
    // ---------- Part 2
    let mut ans2 = 0;
    let mut circuit = vec![false; points.len()];
    let &(_, a, b) = distances.first().unwrap();
    circuit[a] = true;
    circuit[b] = true;
    let mut circuit_size = 2;
    while circuit_size < points.len() {
        for &(_, a, b) in distances.iter() {
            if circuit[a] == circuit[b] {
                continue;
            }
            circuit_size += 1;
            circuit[a] = true;
            circuit[b] = true;
            if circuit_size == points.len() {
                ans2 = points[a].0 * points[b].0;
            }
            break;
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
        test_invalid_msg(&[&"a,2,3"], solve, "x coordinate must be an integer");
    }

    #[test]
    fn invalid_y_must_be_integer() {
        test_invalid_msg(&[&"1,a,3"], solve, "y coordinate must be an integer");
    }

    #[test]
    fn invalid_z_must_be_integer() {
        test_invalid_msg(&[&"1,2,a"], solve, "z coordinate must be an integer");
    }

    #[test]
    fn invalid_missing_y() {
        test_invalid_msg(&[&"1"], solve, "missing y coordinate");
    }

    #[test]
    fn invalid_missing_z() {
        test_invalid_msg(&[&"1,2"], solve, "missing z coordinate");
    }

    #[test]
    fn invalid_must_be_3_coordinates() {
        test_invalid_msg(
            &[&"1,2,3,4"],
            solve,
            "there must be 3 comma-separated coordinates per line",
        );
    }
}
