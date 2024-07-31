//! [aoc](https://adventofcode.com/2015/day/9)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::cmp;
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 9,
        title: "All in a Single Night",
        solution: ("207", "804"),
        example_solutions: vec![("605", "982")],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Part 1 + 2
    let g = CityGraph::new(input)?;
    let (ans1, ans2) = g.get_min_max_dist();
    Ok((ans1.to_string(), ans2.to_string()))
}

#[derive(Default)]
struct CityGraph {
    v: usize,
    nodes: HashMap<String, usize>,
    dist: HashMap<usize, HashMap<usize, ItemType>>,
}

impl CityGraph {
    fn new(input: PuzzleInput) -> Result<Self, PuzzleError> {
        let mut g = Self::default();
        for line in input {
            let a = line.split(" = ").collect::<Vec<_>>();
            if a.len() != 2 {
                return Err(PuzzleError(
                    "road and distance must be separated by =".into(),
                ));
            }
            let dist = a[1]
                .parse::<ItemType>()
                .map_err(|_| PuzzleError("distance must be an integer".into()))?;
            let b = a[0].split(" to ").collect::<Vec<_>>();
            if b.len() != 2 {
                return Err(PuzzleError(
                    "source and destination towns must be separated by 'to'".into(),
                ));
            }
            let default_id = g.nodes.len();
            let id1 = *g.nodes.entry(b[0].to_owned()).or_insert(default_id);
            let default_id = g.nodes.len();
            let id2 = *g.nodes.entry(b[1].to_owned()).or_insert(default_id);
            if id1 == id2 {
                return Err(PuzzleError(
                    "source and destination towns must be different".into(),
                ));
            }
            g.dist.entry(id1).or_default().insert(id2, dist);
            g.dist.entry(id2).or_default().insert(id1, dist);
        }
        g.v = g.nodes.len();
        Ok(g)
    }

    /// based on https://en.wikipedia.org/wiki/Heap%27s_algorithm
    fn get_min_max_dist(&self) -> (ItemType, ItemType) {
        let mut a = (0..self.v).collect::<Vec<_>>();
        let mut c = vec![0; self.v];
        let mut min = 0;
        for j in 1..self.v {
            min += self.dist.get(&a[j - 1]).unwrap().get(&a[j]).unwrap();
        }
        let mut max = min;
        let mut i = 1;
        while i < self.v {
            if c[i] < i {
                if i % 2 == 0 {
                    a.swap(0, i);
                } else {
                    a.swap(c[i], i)
                }
                let mut path = 0;
                for j in 1..self.v {
                    path += self.dist.get(&a[j - 1]).unwrap().get(&a[j]).unwrap();
                }
                min = cmp::min(min, path);
                max = cmp::max(max, path);
                c[i] += 1;
                i = 1;
                continue;
            }
            c[i] = 0;
            i += 1;
        }
        (min, max)
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
    fn invalid_missing_equal_sign() {
        test_invalid(&[&"a"], solve);
    }

    #[test]
    fn invalid_missing_to() {
        test_invalid(&[&"a = 1"], solve);
    }

    #[test]
    fn invalid_distance_must_be_integer() {
        test_invalid(&[&"a to b = c"], solve);
    }

    #[test]
    fn invalid_source_and_destination_must_be_different() {
        test_invalid(&[&"a to a = 1"], solve);
    }
}
