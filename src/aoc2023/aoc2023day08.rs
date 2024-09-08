//! [aoc](https://adventofcode.com/2023/day/8)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::cmp;
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 8,
        title: "Haunted Wasteland",
        solution: ("21883", "12833235391111"),
        example_solutions: vec![("2", "0"), ("6", "0"), ("0", "6")],
    }
}

type ItemType = i64;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() < 3 || !input[1].is_empty() {
        Err("input must have at least 3 lines and the second line must be empty")?;
    }
    let dirs = input[0];
    let mut links = HashMap::new();
    for &line in input.iter().skip(2) {
        let mut a_iter = line.split(" = (");
        let from = a_iter.next().unwrap();
        let b = a_iter.next().ok_or("lines must contain ` = (`")?;
        if &b[b.len() - 1..] != ")" {
            Err("lines must end with `)`")?;
        }
        let mut b_iter = b[..b.len() - 1].split(", ");
        let to1 = b_iter.next().unwrap();
        let to2 = b_iter
            .next()
            .ok_or("each node must be connected to 2 nodes, 1 found")?;
        if b_iter.next().is_some() {
            Err("each node must be connected to 2 nodes, more found")?;
        }
        links.insert(from, (to1, to2));
    }
    // ---------- Part 1
    let mut ans1 = 0;
    if links.contains_key("AAA") {
        let mut node = "AAA";
        loop {
            if node == "ZZZ" {
                break;
            }
            node = match dirs.as_bytes()[ans1 as usize % dirs.len()] {
                b'L' => links.get(node).ok_or("invalid node")?.0,
                b'R' => links.get(node).ok_or("invalid node")?.1,
                _ => Err("directions must contain only L and R")?,
            };
            ans1 += 1;
        }
    }
    // ---------- Part 2
    let mut ans2 = 1;
    let mut start_nodes = Vec::new();
    for &node in links.keys() {
        if node.ends_with('A') {
            start_nodes.push(node);
        }
    }
    if start_nodes.len() <= 1 {
        return Ok((ans1.to_string(), "0".to_string()));
    }
    let mut cycle_starts = vec![0; start_nodes.len()];
    let mut cycle_lengths = vec![0; start_nodes.len()];
    let mut final_steps = vec![0; start_nodes.len()];
    for (idx, &start_node) in start_nodes.iter().enumerate() {
        let mut last_seen_at = HashMap::new();
        let mut step = 0;
        let mut node = start_node;
        loop {
            if node.ends_with('Z') && final_steps[idx] == 0 {
                final_steps[idx] = step;
            }
            let hash = node.to_owned() + " " + &format!("{}", step % dirs.len());
            if last_seen_at.contains_key(&hash) {
                cycle_starts[idx] = *last_seen_at.get(&hash).unwrap();
                cycle_lengths[idx] = step - *last_seen_at.get(&hash).unwrap();
                break;
            }
            last_seen_at.insert(hash, step);
            node = match dirs.as_bytes()[step % dirs.len()] {
                b'L' => links.get(node).ok_or("invalid node")?.0,
                b'R' => links.get(node).ok_or("invalid node")?.1,
                _ => Err("impossible")?,
            };
            step += 1;
        }
    }
    // for the input, cycle_length == final_steps, so the solution is the least common multiple of cycle_lengths.
    for cycle_length in cycle_lengths {
        ans2 = lcm(ans2, cycle_length as ItemType);
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

/// Greatest common divisor.
///
/// [see](https://en.wikipedia.org/wiki/Greatest_common_divisor)
fn gcd(a: ItemType, b: ItemType) -> ItemType {
    let mut a1 = cmp::max(a, b);
    let mut b1 = cmp::min(a, b);
    while b1 != 0 {
        let t = b1;
        b1 = a1 % b1;
        a1 = t;
    }
    a1
}

/// Least common multiple.
///
/// [see](https://en.wikipedia.org/wiki/Least_common_multiple)
fn lcm(a: ItemType, b: ItemType) -> ItemType {
    a.abs() * (b.abs() / gcd(a, b))
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
    fn example3() {
        test_case(metadata, solve, 3);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_at_least_3_lines() {
        test_invalid_msg(&[&"RL", &""], solve, "input must have at least 3 lines");
    }

    #[test]
    fn invalid_second_line_empty() {
        test_invalid_msg(
            &[&"RL", &"a", &"AAA = (BBB, CCC)"],
            solve,
            "second line must be empty",
        );
    }

    #[test]
    fn invalid_line_separator() {
        test_invalid_msg(
            &[&"RL", &"", &"AAA =a (BBB, CCC)"],
            solve,
            "lines must contain ` = (`",
        );
    }

    #[test]
    fn invalid_must_end_closing_par() {
        test_invalid_msg(
            &[&"RL", &"", &"AAA = (BBB, CCC"],
            solve,
            "lines must end with `)`",
        );
    }

    #[test]
    fn invalid_must_have_two_targets_not_one() {
        test_invalid_msg(
            &[&"RL", &"", &"AAA = (BBB)"],
            solve,
            "each node must be connected to 2 nodes, 1 found",
        );
    }

    #[test]
    fn invalid_must_have_two_targets_not_more() {
        test_invalid_msg(
            &[&"RL", &"", &"AAA = (BBB, CCC, DDD)"],
            solve,
            "each node must be connected to 2 nodes, more found",
        );
    }

    #[test]
    fn invalid_direction_must_be_lr() {
        test_invalid_msg(
            &[&"RaL", &"", &"AAA = (BBB, CCC)"],
            solve,
            "directions must contain only L and R",
        );
    }

    #[test]
    fn invalid_target_node() {
        test_invalid_msg(&[&"RL", &"", &"AAA = (BBB, CCC)"], solve, "invalid node");
    }
}
