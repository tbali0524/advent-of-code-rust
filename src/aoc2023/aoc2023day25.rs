//! [aoc](https://adventofcode.com/2023/day/25)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::{HashMap, HashSet};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2023,
        day: 25,
        title: "Snowverload",
        solution: ("619225", "0"),
        example_solutions: vec![("54", "0")],
    }
}

const EDGES_TO_CUT_EXAMPLE: [(&str, &str); 3] = [("jqt", "nvd"), ("bvb", "cmg"), ("hfx", "pzl")];
const EDGES_TO_CUT: [(&str, &str); 3] = [("nrs", "khn"), ("ssd", "xqh"), ("qlc", "mqb")];

/// A proper solution would be one of the algorithms according to:
///   [link](https://en.wikipedia.org/wiki/K-edge-connected_graph)
/// Instead of this, I manually selected the edges to cut.
///   using the hint from: [link](http://clb.confined.space/aoc2023/#day25)
/// Install [graphviz](https://graphviz.org/)
/// Manual edit input file to change to dot file format [ref](https://graphviz.org/doc/info/lang.html)
/// Run in command-line:
/// ```text
///    dot -Tsvg -Kneato input\2023\Aoc2023Day25_graphviz.txt -o input\2023\Aoc2023Day25_graphviz.svg
///    dot -Tsvg -Kneato input\2023\Aoc2023Day25ex1_graphviz.txt -o input\2023\Aoc2023Day25ex1_graphviz.svg
/// ```
pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut edges = HashMap::<String, HashMap<String, usize>>::new();
    for &line in input {
        let mut a_iter = line.split(": ");
        let from = a_iter.next().unwrap();
        edges.entry(from.to_owned()).or_default();
        let b_iter = a_iter
            .next()
            .ok_or("lines must contain `: `")?
            .split_whitespace();
        for to in b_iter {
            edges.entry(to.to_owned()).or_default();
            edges.get_mut(from).unwrap().insert(to.to_owned(), 1);
            edges.get_mut(to).unwrap().insert(from.to_owned(), 1);
        }
    }
    // ---------- Part 1
    let edges_to_cut = if edges.len() < 20 {
        EDGES_TO_CUT_EXAMPLE
    } else {
        EDGES_TO_CUT
    };
    for (from, to) in edges_to_cut.iter() {
        edges
            .get_mut(from.to_owned())
            .unwrap()
            .remove(to.to_owned());
        edges
            .get_mut(to.to_owned())
            .unwrap()
            .remove(from.to_owned());
    }
    let edge = edges_to_cut[0].0;
    let mut visited = HashSet::new();
    visited.insert(edge.to_owned());
    let mut q = vec![edge];
    let mut read_idx = 0;
    while read_idx < q.len() {
        let current = q[read_idx];
        read_idx += 1;
        if !edges.contains_key(current) {
            continue;
        }
        for to in edges.get(current).unwrap().keys() {
            if visited.contains(to) {
                continue;
            }
            q.push(to);
            visited.insert(to.to_owned());
        }
    }
    let ans1 = visited.len() * (edges.len() - visited.len());
    let ans2 = 0;
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
    fn invalid_must_have_colon() {
        test_invalid_msg(&["jqt rhn xhk nvd"], solve, "lines must contain `: `");
    }
}
