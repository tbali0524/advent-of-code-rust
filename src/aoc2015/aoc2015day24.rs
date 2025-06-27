//! [aoc](https://adventofcode.com/2015/day/24)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 24,
        title: "It Hangs in the Balance",
        solution: ("10439961859", "72050269"),
        example_solutions: vec![("99", "44")],
    }
}

type ItemType = i64;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Part 1 + 2
    let mut balancer = SleighBalancer::from_input(input)?;
    let ans1 = balancer.solve_for_group_count(3);
    let ans2 = balancer.solve_for_group_count(4);
    Ok((ans1.to_string(), ans2.to_string()))
}

#[derive(Default)]
struct SleighBalancer {
    weights: Vec<ItemType>,
    target: ItemType,
    total_remaining: Vec<ItemType>,
    grp: Vec<ItemType>,
    grp_sum: ItemType,
    candidates: Vec<Vec<ItemType>>,
}

impl SleighBalancer {
    fn from_input(input: PuzzleInput) -> Result<Self, PuzzleError> {
        let mut weights = input
            .iter()
            .map(|&line| {
                line.parse::<ItemType>()
                    .map_err(|_| format!("input must contain only integers, found `{line}`"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        weights.sort();
        weights.reverse();
        let mut total_remaining = vec![0; weights.len() + 1];
        for i in (0..weights.len()).rev() {
            total_remaining[i] = total_remaining[i + 1] + weights[i];
        }
        Ok(Self {
            weights,
            total_remaining,
            ..Default::default()
        })
    }

    fn solve_for_group_count(&mut self, count_groups: ItemType) -> ItemType {
        self.target = self.weights.iter().sum::<ItemType>() / count_groups;
        self.grp = Vec::new();
        self.grp_sum = 0;
        self.candidates = Vec::new();
        self.find_candidates(0);
        let min_count = self.candidates.iter().map(|x| x.len()).min().unwrap();
        self.candidates
            .iter()
            .filter(|&x| x.len() == min_count)
            .map(|x| x.iter().product::<ItemType>())
            .min()
            .unwrap()
    }

    fn find_candidates(&mut self, idx: usize) {
        if self.grp_sum == self.target {
            self.candidates.push(self.grp.clone());
            return;
        }
        if idx >= self.weights.len() {
            return;
        }
        let w = self.weights[idx];
        if self.grp_sum + w <= self.target {
            self.grp.push(w);
            self.grp_sum += w;
            self.find_candidates(idx + 1);
            self.grp.pop();
            self.grp_sum -= w;
        }
        if self.total_remaining[idx + 1] >= self.target - self.grp_sum {
            self.find_candidates(idx + 1);
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
    fn invalid_must_contain_only_integers() {
        test_invalid(&["1", "a"], solve);
    }
}
