//! [aoc](https://adventofcode.com/2025/day/10)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
// use std::cmp::Reverse;
use std::collections::HashSet;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2025,
        day: 10,
        title: "Factory",
        solution: ("509", "0"),
        example_solutions: vec![("7", "0")], // 33
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut machines = Vec::new();
    for &line in input.iter() {
        machines.push(Machine::from_str(line)?);
    }
    // eprintln!(
    //     "len but {}",
    //     machines.iter().map(|x| x.buttons.len()).max().unwrap()
    // );
    // eprintln!(
    //     "len jolt {}",
    //     machines.iter().map(|x| x.joltages.len()).max().unwrap()
    // );
    // eprintln!(
    //     "max jolt {}",
    //     machines
    //         .iter()
    //         .map(|x| x.joltages.iter().max().unwrap())
    //         .max()
    //         .unwrap()
    // );
    // eprintln!(
    //     "sum jolt {}",
    //     machines
    //         .iter()
    //         .map(|x| x.joltages.iter().sum::<usize>())
    //         .max()
    //         .unwrap()
    // );
    // ---------- Part 1
    let mut ans1 = 0;
    for machine in &machines {
        let mut visited = HashSet::new();
        let mut q = Vec::new();
        q.push((0, 0));
        visited.insert(0);
        let mut idx_read = 0;
        while idx_read < q.len() {
            let (state, step) = q[idx_read];
            idx_read += 1;
            if state == machine.target {
                ans1 += step;
                break;
            }
            for wiring in &machine.buttons {
                let next_state = state ^ wiring;
                if visited.contains(&next_state) {
                    continue;
                }
                visited.insert(next_state);
                q.push((next_state, step + 1));
            }
        }
    }
    // ---------- Part 2
    let ans2 = 0;
    // for machine in &machines {}

    // greedy method -> fast, but not correct
    // -----------------------------
    // for machine in &machines {
    //     let mut res = 0;
    //     let mut wirings = machine.levers.iter().zip(machine.buttons.iter()).collect::<Vec<_>>();
    //     wirings.sort_by_key(|x| Reverse(x.0));
    //     let mut remaining = machine.joltages.clone();
    //     let mut remaining_map = machine.joltages_map;
    //     if input.len() == 3 {
    //         eprintln!("========= M: {:?}", machine);
    //         eprintln!("wirings: {:?}", wirings);
    //     }
    //     for (_, button) in wirings {
    //         if input.len() == 3 {
    //             eprintln!("trying: {}", button);
    //         }
    //         while remaining_map & *button == *button {
    //             res += 1;
    //             if input.len() == 3 {
    //                 eprintln!("...pushed");
    //             }
    //             let mut bit_pos = 0;
    //             let mut w = *button;
    //             while w != 0 {
    //                 if w & 1 != 0 {
    //                     remaining[bit_pos] -= 1;
    //                     if remaining[bit_pos] == 0 {
    //                         remaining_map &= !(1 << bit_pos);
    //                     }
    //                 }
    //                 bit_pos += 1;
    //                 w >>= 1;
    //             }
    //             if input.len() == 3 {
    //                 eprintln!("...... rem: {:?}", remaining);
    //                 eprintln!("...... rem_map: {}", remaining_map);
    //             }
    //         }
    //     }
    //     if input.len() == 3 {
    //         eprintln!("res: {}", res);
    //     }
    //     ans2 += res;
    // }
    // eprintln!("ans2: {}", ans2);

    // BFS -> correct but too slow
    // -----------------------------
    // if input.len() > 3 {
    //     return Ok((ans1.to_string(), ans2.to_string()));
    // }
    // for machine in &machines {
    //     let mut visited = HashSet::new();
    //     let mut q = VecDeque::new();
    //     let counters = vec![0; machine.joltages.len()];
    //     visited.insert(counters.to_owned());
    //     q.push_back((counters, 0));
    //     while let Some(item) = q.pop_front() {
    //         let (counters, step) = item;
    //         if *counters == machine.joltages {
    //             ans2 += step;
    //             break;
    //         }
    //         for wiring in &machine.buttons {
    //             let mut next_counters = counters.clone();
    //             let mut w = *wiring;
    //             let mut bit_pos = 0;
    //             let mut is_ok = true;
    //             while w != 0 {
    //                 if w & 1 != 0 {
    //                     next_counters[bit_pos] += 1;
    //                     if next_counters[bit_pos] > machine.joltages[bit_pos] {
    //                         is_ok = false;
    //                         break;
    //                     }
    //                 }
    //                 w >>= 1;
    //                 bit_pos += 1;
    //             }
    //             if !is_ok || visited.contains(&next_counters) {
    //                 continue;
    //             }
    //             visited.insert(next_counters.to_owned());
    //             q.push_back((next_counters, step + 1));
    //         }
    //     }
    // }
    Ok((ans1.to_string(), ans2.to_string()))
}

#[allow(dead_code)]
#[derive(Debug)]
struct Machine {
    target: usize,       // indicator light diagram as bitmap
    buttons: Vec<usize>, // button wiring schematics as bitmap
    levers: Vec<usize>,  // number of levers for each button
    joltages: Vec<usize>,
    joltages_map: usize, // which buttons have nonzero joltage, as bitmap
}

impl Machine {
    fn from_str(line: &str) -> Result<Self, PuzzleError> {
        let mut target = 0;
        let mut buttons = Vec::new();
        let mut levers = Vec::new();
        let mut joltages = Vec::new();
        let mut joltages_map = 0;
        for (idx, item) in line.split(' ').enumerate() {
            if item.len() < 3 {
                Err("all items must be at least 3 chars long")?;
            }
            let inside = &item[1..item.len() - 1];
            match item.chars().next().unwrap_or(' ') {
                '[' => {
                    if idx != 0 {
                        Err("indicator light diagram must be the first item in line")?;
                    }
                    for (bit_pos, c) in inside.chars().enumerate() {
                        let bit = match c {
                            '#' => 1,
                            '.' => 0,
                            _ => Err("indicator light diagram must contain only `.` or `#`")?,
                        };
                        target |= bit << bit_pos;
                    }
                }
                '(' => {
                    if !joltages.is_empty() {
                        Err("joltage list must be the last item in the line")?;
                    }
                    let mut wiring = 0;
                    let mut count_lever = 0;
                    for num in inside.split(',') {
                        let pos = num
                            .parse::<usize>()
                            .map_err(|_| "wiring schematics must be comma-separated integers")?;
                        wiring |= 1 << pos;
                        count_lever += 1;
                    }
                    buttons.push(wiring);
                    levers.push(count_lever);
                }
                '{' => {
                    if !joltages.is_empty() {
                        Err("there must be only 1 joltage list per line, more found")?;
                    }
                    for (bit_pos, num) in inside.split(',').enumerate() {
                        let joltage = num
                            .parse::<usize>()
                            .map_err(|_| "joltages must be comma-separated integers")?;
                        joltages.push(joltage);
                        if joltage != 0 {
                            joltages_map |= 1 << bit_pos;
                        }
                    }
                }
                _ => Err("all items in input must start with `[`, `(` or `{`")?,
            }
        }
        if target == 0 {
            Err("missing indicator light diagram")?;
        }
        if buttons.is_empty() {
            Err("missing wiring schematics")?;
        }
        if joltages.is_empty() {
            Err("missing joltages list")?;
        }
        Ok(Machine {
            target,
            buttons,
            levers,
            joltages,
            joltages_map,
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
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_item_too_short() {
        test_invalid_msg(
            &[&"[.#] () {1}"],
            solve,
            "all items must be at least 3 chars long",
        );
    }

    #[test]
    fn invalid_target_must_be_first() {
        test_invalid_msg(
            &[&"(0,1) [.#] {1}"],
            solve,
            "indicator light diagram must be the first item in line",
        );
    }

    #[test]
    fn invalid_indicator_char() {
        test_invalid_msg(
            &[&"[a] (0,1) {1}"],
            solve,
            "indicator light diagram must contain only `.` or `#`",
        );
    }

    #[test]
    fn invalid_joltages_must_be_last() {
        test_invalid_msg(
            &[&"[.#] {1} (0,1)"],
            solve,
            "joltage list must be the last item in the line",
        );
    }

    #[test]
    fn invalid_wiring_must_be_integers() {
        test_invalid_msg(
            &[&"[.#] (0,a) {1}"],
            solve,
            "wiring schematics must be comma-separated integers",
        );
    }

    #[test]
    fn invalid_joltige_must_be_single() {
        test_invalid_msg(
            &[&"[.#] (0,1) {1} {2}"],
            solve,
            "there must be only 1 joltage list per line, more found",
        );
    }

    #[test]
    fn invalid_joltige_must_be_integers() {
        test_invalid_msg(
            &[&"[.#] (0,1) {a}"],
            solve,
            "joltages must be comma-separated integers",
        );
    }

    #[test]
    fn invalid_item_start_char() {
        test_invalid_msg(
            &[&"[.#] a0,1) {1}"],
            solve,
            "all items in input must start with `[`, `(` or `{`",
        );
    }

    #[test]
    fn invalid_missing_indicator() {
        test_invalid_msg(&[&"(0,1) {1}"], solve, "missing indicator light diagram");
    }

    #[test]
    fn invalid_missing_wiring() {
        test_invalid_msg(&[&"[.#] {1}"], solve, "missing wiring schematics");
    }

    #[test]
    fn invalid_missing_joltages() {
        test_invalid_msg(&[&"[.#] (0,1)"], solve, "missing joltages list");
    }
}
