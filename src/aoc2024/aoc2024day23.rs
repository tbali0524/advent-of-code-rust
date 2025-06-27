//! [aoc](https://adventofcode.com/2024/day/23)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 23,
        title: "LAN Party",
        solution: ("1200", "ag,gh,hh,iv,jx,nq,oc,qm,rb,sm,vm,wu,zr"),
        example_solutions: vec![("7", "co,de,ka,ta")],
    }
}

type Vertex = [u8; 2];
type EdgeSet = HashMap<Vertex, HashSet<Vertex>>;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut vertices = HashSet::<Vertex>::new();
    let mut edges = EdgeSet::new();
    for &row in input {
        if row.len() != 5 {
            Err("input lines must have 5 chars")?;
        }
        let v1: [u8; 2] = row.as_bytes()[0..2].try_into().unwrap();
        let v2: [u8; 2] = row.as_bytes()[3..5].try_into().unwrap();
        vertices.insert(v1.to_owned());
        vertices.insert(v2.to_owned());
        edges
            .entry(v1.to_owned())
            .or_default()
            .insert(v2.to_owned());
        edges
            .entry(v2.to_owned())
            .or_default()
            .insert(v1.to_owned());
    }
    // ---------- Part 1
    let mut ans1 = 0;
    for (v1, adj_set) in edges.iter() {
        for v2 in adj_set.iter() {
            for v3 in adj_set.iter() {
                if v2 == v3 {
                    continue;
                }
                if v1[0] != b't' && v2[0] != b't' && v3[0] != b't' {
                    continue;
                }
                if !edges.get(v2).unwrap().contains(v3) {
                    continue;
                }
                ans1 += 1;
            }
        }
    }
    ans1 /= 6;
    // ---------- Part 2
    let r = HashSet::<Vertex>::new();
    let p = vertices.to_owned();
    let x = HashSet::<Vertex>::new();
    let mut maximal_cliques = Vec::<HashSet<Vertex>>::new();
    bron_kerbosch_1(&mut maximal_cliques, &edges, &r, &p, &x);
    maximal_cliques.sort_by_key(|a| Reverse(a.len()));
    let result_vertices = maximal_cliques.first().unwrap();
    let mut result = result_vertices
        .iter()
        .map(|x| std::str::from_utf8(x).unwrap().to_string())
        .collect::<Vec<_>>();
    result.sort();
    let mut ans2 = String::new();
    let mut sep = "";
    for item in result.iter() {
        ans2.push_str(sep);
        ans2.push_str(item);
        sep = ",";
    }
    Ok((ans1.to_string(), ans2))
}

/// Find all maximal cliques in a graph.
///
/// [see](https://https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm)
fn bron_kerbosch_1(
    maximal_cliques: &mut Vec<HashSet<Vertex>>,
    edges: &EdgeSet,
    r: &HashSet<Vertex>,
    p: &HashSet<Vertex>,
    x: &HashSet<Vertex>,
) {
    if p.is_empty() && x.is_empty() {
        maximal_cliques.push(r.to_owned());
    }
    let mut p1 = p.to_owned();
    let mut x1 = x.to_owned();
    for v in p.iter() {
        let nb = edges.get(v).unwrap();
        let mut r2 = r.to_owned();
        r2.insert(v.to_owned());
        let p2 = p1.intersection(nb).copied().collect::<HashSet<_>>();
        let x2 = x1.intersection(nb).copied().collect::<HashSet<_>>();
        bron_kerbosch_1(maximal_cliques, edges, &r2, &p2, &x2);
        p1.remove(v);
        x1.insert(v.to_owned());
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
    fn invalid_must_be_5_chars_with() {
        test_invalid_msg(&["a"], solve, "input lines must have 5 chars");
    }
}
