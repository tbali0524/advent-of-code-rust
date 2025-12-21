//! [aoc](https://adventofcode.com/2025/day/10)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::cmp::Reverse;
use std::collections::HashSet;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2025,
        day: 10,
        title: "Factory",
        solution: ("509", "0"),
        example_solutions: vec![("7", "33")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut machines = Vec::new();
    for &line in input.iter() {
        machines.push(Machine::from_str(line)?);
    }
    // ---------- Part 1
    let mut ans1 = 0;
    for machine in machines.iter() {
        ans1 += machine.solve_part1();
    }
    // ---------- Part 2
    let mut ans2 = 0;
    for machine in machines.iter_mut() {
        ans2 += machine.solve_part2();
        // if machine.best_part2.is_some() {
        //     eprint!(".");
        // } else {
        //     eprint!("?");
        // }
        // eprintln!("======== M: {:?}", machine);
    }
    eprintln!();
    Ok((ans1.to_string(), ans2.to_string()))
}

#[derive(Debug)]
struct Machine {
    // used for part 1:
    target_bitmap: usize, // input: indicator light diagram target as bitmap
    button_bitmaps: Vec<usize>, // input: 7button wiring schematics as bitmap
    // used for part 2:
    buttons: Vec<Vec<usize>>, // input: buttons as list of counter ids
    joltages: Vec<usize>,     // input: joltage targets
    sorted_button_ids: Vec<usize>,
    total_joltage: usize,
    counters: Vec<usize>,
    count_push: usize,
    sum_joltage: usize,
    best_part2: Option<usize>,
    count_iter: usize,
}

impl Machine {
    fn from_str(line: &str) -> Result<Self, PuzzleError> {
        let mut target_bitmap = 0;
        let mut button_bitmaps = Vec::new();
        let mut buttons = Vec::new();
        let mut joltages = Vec::new();
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
                        target_bitmap |= bit << bit_pos;
                    }
                }
                '(' => {
                    if !joltages.is_empty() {
                        Err("joltage list must be the last item in the line")?;
                    }
                    let button = inside
                        .split(',')
                        .map(|x| {
                            x.parse::<usize>()
                                .map_err(|_| "wiring schematics must be comma-separated integers")
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    let mut wiring = 0;
                    for &pos in &button {
                        wiring |= 1 << pos;
                    }
                    button_bitmaps.push(wiring);
                    buttons.push(button);
                }
                '{' => {
                    if !joltages.is_empty() {
                        Err("there must be only 1 joltage list per line, more found")?;
                    }
                    joltages = inside
                        .split(',')
                        .map(|x| {
                            x.parse::<usize>()
                                .map_err(|_| "joltages must be comma-separated integers")
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                }
                _ => Err("all items in input must start with `[`, `(` or `{`")?,
            }
        }
        if target_bitmap == 0 {
            Err("missing indicator light diagram")?;
        }
        if button_bitmaps.is_empty() {
            Err("missing wiring schematics")?;
        }
        if joltages.is_empty() {
            Err("missing joltages list")?;
        }
        let total_joltage = joltages.iter().sum();
        Ok(Machine {
            target_bitmap,
            button_bitmaps,
            buttons,
            joltages,
            total_joltage,
            sorted_button_ids: Vec::new(),
            counters: Vec::new(),
            count_push: 0,
            sum_joltage: 0,
            best_part2: None,
            count_iter: 0,
        })
    }

    fn solve_part1(&self) -> usize {
        let mut visited = HashSet::new();
        let mut q = Vec::new();
        q.push((0, 0));
        visited.insert(0);
        let mut idx_read = 0;
        while idx_read < q.len() {
            let (state, step) = q[idx_read];
            idx_read += 1;
            if state == self.target_bitmap {
                return step;
            }
            for button_bitmap in &self.button_bitmaps {
                let next_state = state ^ button_bitmap;
                if visited.contains(&next_state) {
                    continue;
                }
                visited.insert(next_state);
                q.push((next_state, step + 1));
            }
        }
        0
    }

    fn solve_part2(&mut self) -> usize {
        self.sorted_button_ids = (0..self.buttons.len()).collect::<Vec<_>>();
        self.sorted_button_ids
            .sort_by_key(|x| Reverse(self.buttons[*x].len()));
        self.counters = vec![0; self.joltages.len()];
        self.count_push = 0;
        self.best_part2 = None;
        self.backtrack(0);
        self.best_part2.unwrap_or_default()
    }

    fn backtrack(&mut self, next_button: usize) {
        // current implementation is either too slow, or does not find all solutions...
        if self.count_iter >= 100_000 {
            return;
        }
        self.count_iter += 1;
        if next_button == self.buttons.len() {
            let result = self.counters == self.joltages;
            if result && (self.best_part2.is_none() || self.best_part2.unwrap() > self.count_push) {
                self.best_part2 = Some(self.count_push);
            }
            return;
        }
        let button_id = self.sorted_button_ids[next_button];
        if let Some(best) = self.best_part2 {
            let remaining_push = best - self.count_push;
            let remaining_joltage = self.total_joltage - self.sum_joltage;
            if remaining_push * self.buttons[button_id].len() < remaining_joltage {
                return;
            }
        }
        let mut max_push = usize::MAX;
        for &counter_id in self.buttons[button_id].iter() {
            let push = self.joltages[counter_id].saturating_sub(self.counters[counter_id]);
            if push < max_push {
                max_push = push;
            }
        }
        self.count_push += max_push;
        self.sum_joltage += max_push * self.buttons[button_id].len();
        for &counter_id in self.buttons[button_id].iter() {
            self.counters[counter_id] += max_push;
        }
        for _ in 0..max_push {
            if self.best_part2.is_none() || self.best_part2.unwrap() > self.count_push {
                self.backtrack(next_button + 1);
            }
            self.count_push -= 1;
            self.sum_joltage -= self.buttons[button_id].len();
            for &counter_id in self.buttons[button_id].iter() {
                self.counters[counter_id] -= 1;
            }
        }
        if self.best_part2.is_none() || self.best_part2.unwrap() > self.count_push {
            self.backtrack(next_button + 1);
        }
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
