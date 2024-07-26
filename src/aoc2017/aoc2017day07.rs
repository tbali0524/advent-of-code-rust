//! [aoc](https://adventofcode.com/2017/day/7)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashMap;

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2017,
    day: 7,
    title: "Recursive Circus",
    solution: (0, 0),
    example_solutions: [(0, 0), (0, 0)],
    string_solution: Some(("gynfwly", "1526")),
    example_string_solutions: Some([("tknk", "60"), ("", "")]),
    example_string_inputs: None,
};

type ItemType = i32;

#[derive(Default)]
struct Node {
    name: String,
    parent: Option<String>,
    children: Vec<String>,
    weight: ItemType,
    total: ItemType,
}

impl Node {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Default)]
struct Tree {
    nodes: HashMap<String, Node>,
    root: Option<String>,
}

impl Tree {
    fn new() -> Self {
        Default::default()
    }

    fn from_input(input: PuzzleInput) -> Result<Tree, &'static str> {
        let mut tree = Tree::new();
        let mut parent_child_pairs = Vec::new();
        for line in input {
            let mut a = line.split(" -> ");
            let mut b = a.next().unwrap().split(" (");
            let name = b.next().unwrap();
            let c = b.next().ok_or("missing (weight) in input line")?;
            let weight = c[..(c.len() - 1)]
                .parse::<ItemType>()
                .map_err(|_| "weight must be an integer")?;
            let d = a.next();
            let children = if let Some(e) = d {
                e.split(", ").map(|x| x.to_owned()).collect::<Vec<_>>()
            } else {
                Vec::new()
            };
            if a.next().is_some() || b.next().is_some() {
                return Err("Invalid input");
            }
            let mut node = Node::new();
            node.name = name.to_string();
            for child in &children {
                parent_child_pairs.push((name.to_owned(), child.to_owned()));
            }
            node.children = children;
            node.weight = weight;
            tree.nodes.insert(name.to_owned(), node);
        }
        for (parent, child) in &parent_child_pairs {
            tree.nodes
                .get_mut(child)
                .ok_or("invalid node referenced in children list")?
                .parent = Some(parent.to_owned());
        }
        Ok(tree)
    }

    fn find_root(&mut self) -> Result<String, &'static str> {
        if let Some(r) = &self.root {
            return Ok(r.to_owned());
        }
        for (name, node) in &self.nodes {
            if node.parent.is_none() {
                self.root = Some(name.to_owned());
                return Ok(name.to_owned());
            }
        }
        Err("No root node found")
    }

    fn calc_total_weight(&mut self, name: &str) -> Result<ItemType, &'static str> {
        let mut total = 0;
        let children = self
            .nodes
            .get(name)
            .ok_or("invalid node name")?
            .children
            .to_owned();
        for child in &children {
            total += self.calc_total_weight(child)?;
        }
        let node = self.nodes.get_mut(name).unwrap();
        node.total = total + node.weight;
        Ok(node.total)
    }

    fn find_unbalanced(&mut self, name: &str) -> Result<ItemType, &'static str> {
        let children = self
            .nodes
            .get(name)
            .ok_or("invalid node name")?
            .children
            .to_owned();
        let mut count_same_totals = HashMap::<ItemType, usize>::new();
        let mut total_to_name = HashMap::<ItemType, String>::new();
        for child in &children {
            let child_result = self.find_unbalanced(child)?;
            if child_result > 0 {
                return Ok(child_result);
            }
            let child_total = self.nodes.get(child).unwrap().total;
            count_same_totals
                .entry(child_total)
                .and_modify(|x| *x += 1)
                .or_insert(1);
            total_to_name.insert(child_total, child.to_owned());
        }
        match count_same_totals.len() {
            0 => Ok(0),
            1 => Ok(0),
            2 => {
                let mut iter = count_same_totals.iter();
                let (&total_first, &count_first) = iter.next().unwrap();
                let (&total_second, &_count_second) = iter.next().unwrap();
                if count_first == 1 {
                    Ok(total_second - total_first
                        + self
                            .nodes
                            .get(total_to_name.get(&total_first).unwrap())
                            .unwrap()
                            .weight)
                } else {
                    Ok(total_first - total_second
                        + self
                            .nodes
                            .get(total_to_name.get(&total_second).unwrap())
                            .unwrap()
                            .weight)
                }
            }
            _ => Err("Invalid input, multiple unbalance nodes"),
        }
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut tree = Tree::from_input(input)?;
    // ---------- Part 1
    let ans1 = tree.find_root()?;
    // ---------- Part 2
    tree.calc_total_weight(&ans1)?;
    let ans2 = tree.find_unbalanced(&ans1)?;
    Ok((ans1, ans2.to_string()))
}

// ------------------------------------------------------------
// --- boilerplate below ---

pub fn run() -> bool {
    crate::aoc::runner::run_puzzle(&PUZZLE_METADATA, solve)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::runner::tests::*;

    #[test]
    fn example1() {
        test_case(&PUZZLE_METADATA, 1, solve);
    }

    #[test]
    fn puzzle() {
        test_case(&PUZZLE_METADATA, 0, solve);
    }

    #[test]
    fn invalid_missing_weight() {
        test_invalid(&PUZZLE_METADATA, &[String::from("a")], solve);
    }

    #[test]
    fn invalid_weight_must_be_int() {
        test_invalid(&PUZZLE_METADATA, &[String::from("a (b)")], solve);
    }

    #[test]
    fn invalid_node_reference() {
        test_invalid(
            &PUZZLE_METADATA,
            &[String::from("a (1)"), String::from("b (2) -> c")],
            solve,
        );
    }
}
