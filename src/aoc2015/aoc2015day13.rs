//! [aoc](https://adventofcode.com/2015/day/13)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::cmp;
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 13,
        title: "Knights of the Dinner Table",
        solution: ("709", "668"),
        example_solutions: vec![("330", "0")],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Part 1 + 2
    let mut g = KnightsTable::new(input)?;
    let ans1 = g.get_max_happiness();
    g.add_zero_node();
    let ans2 = g.get_max_happiness();
    Ok((ans1.to_string(), ans2.to_string()))
}

#[derive(Default)]
struct KnightsTable {
    v: usize,
    nodes: HashMap<String, usize>,
    dist: HashMap<usize, HashMap<usize, ItemType>>,
}

impl KnightsTable {
    fn new(input: PuzzleInput) -> Result<Self, PuzzleError> {
        let mut g = Self::default();
        for line in input {
            let a = line.split(' ').collect::<Vec<_>>();
            if a.len() != 11
                || a[1] != "would"
                || !line.contains(" happiness units by sitting next to ")
            {
                Err("invalid input sentence")?;
            }
            let happiness = a[3]
                .parse::<ItemType>()
                .map_err(|_| format!("happiness must be an integer, found `{}", a[3]))?;
            let default_id = g.nodes.len();
            let id1 = *g.nodes.entry(a[0].to_owned()).or_insert(default_id);
            let default_id = g.nodes.len();
            let id2 = *g
                .nodes
                .entry(a[10][0..a[10].len() - 1].to_owned())
                .or_insert(default_id);
            if id1 == id2 {
                Err("source and destination knight must be different")?;
            }
            let sign = match a[2] {
                "gain" => 1,
                "lose" => -1,
                _ => Err(format!("verb must be gain or lose, found `{}`", a[2]))?,
            };
            g.dist.entry(id1).or_default().insert(id2, sign * happiness);
        }
        g.v = g.nodes.len();
        Ok(g)
    }

    fn add_zero_node(&mut self) {
        self.nodes.insert("myself".to_owned(), self.v);
        for i in 0..self.v {
            self.dist.entry(i).or_default().insert(self.v, 0);
            self.dist.entry(self.v).or_default().insert(i, 0);
        }
        self.v += 1;
    }

    /// based on <https://en.wikipedia.org/wiki/Heap%27s_algorithm>
    fn get_max_happiness(&self) -> ItemType {
        let mut a = (0..self.v).collect::<Vec<_>>();
        let mut c = vec![0; self.v];
        let mut max = self.dist.get(&a[self.v - 1]).unwrap().get(&a[0]).unwrap()
            + self.dist.get(&a[0]).unwrap().get(&a[self.v - 1]).unwrap();
        for j in 1..self.v {
            max += self.dist.get(&a[j - 1]).unwrap().get(&a[j]).unwrap()
                + self.dist.get(&a[j]).unwrap().get(&a[j - 1]).unwrap();
        }
        let mut i = 1;
        while i < self.v {
            if c[i] < i {
                if i % 2 == 0 {
                    a.swap(0, i);
                } else {
                    a.swap(c[i], i)
                }
                let mut circle = self.dist.get(&a[self.v - 1]).unwrap().get(&a[0]).unwrap()
                    + self.dist.get(&a[0]).unwrap().get(&a[self.v - 1]).unwrap();
                for j in 1..self.v {
                    circle += self.dist.get(&a[j - 1]).unwrap().get(&a[j]).unwrap()
                        + self.dist.get(&a[j]).unwrap().get(&a[j - 1]).unwrap();
                }
                max = cmp::max(max, circle);
                c[i] += 1;
                i = 1;
                continue;
            }
            c[i] = 0;
            i += 1;
        }
        max
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
    fn invalid_wrong_number_of_words() {
        test_invalid(&[&"a"], solve);
    }

    #[test]
    fn invalid_verb() {
        test_invalid(
            &[&"Alice would GET 1 happiness units by sitting next to Bob."],
            solve,
        );
    }

    #[test]
    fn invalid_happiness_must_be_integer() {
        test_invalid(
            &[&"Alice would gain X happiness units by sitting next to Bob."],
            solve,
        );
    }

    #[test]
    fn invalid_source_and_destination_must_be_different() {
        test_invalid(
            &[&"Alice would gain 1 happiness units by sitting next to Alice."],
            solve,
        );
    }
}
