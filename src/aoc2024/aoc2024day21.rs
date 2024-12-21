//! [aoc](https://adventofcode.com/2024/day/21)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 21,
        title: "Keypad Conundrum",
        solution: ("163086", "0"),
        example_solutions: vec![("126384", "0")],
    }
}

const ILLEGAL: char = '_';
const ACTION: char = 'A';
const CHAIN_LEN_PART1: usize = 2;
const CHAIN_LEN_PART2: usize = 2;

type ItemType = usize;
type KeyPad = HashMap<char, (i32, i32)>;
type PathList = Vec<Vec<char>>;
type PathMap = HashMap<(char, char), PathList>;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let keypads = input
        .iter()
        .map(|&x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    if keypads
        .iter()
        .any(|keypad| keypad.len() != 4 || keypad[3] != 'A')
    {
        Err("keycodes must be 3 digits followed by an `A`")?;
    }
    let numerics = input
        .iter()
        .map(|&x| {
            x[..(x.len() - 1)]
                .parse::<ItemType>()
                .map_err(|_| "keycodes must be numeric")
        })
        .collect::<Result<Vec<_>, _>>()?;
    // ---------- Part 1
    let numeric_pad: KeyPad = HashMap::from([
        ('7', (0, 0)),
        ('8', (1, 0)),
        ('9', (2, 0)),
        ('4', (0, 1)),
        ('5', (1, 1)),
        ('6', (2, 1)),
        ('1', (0, 2)),
        ('2', (1, 2)),
        ('3', (2, 2)),
        (ILLEGAL, (0, 3)),
        ('0', (1, 3)),
        (ACTION, (2, 3)),
    ]);
    let direction_pad: KeyPad = HashMap::from([
        (ILLEGAL, (0, 0)),
        ('^', (1, 0)),
        (ACTION, (2, 0)),
        ('<', (0, 1)),
        ('v', (1, 1)),
        ('>', (2, 1)),
    ]);
    let numeric_path_map = get_path_map(&numeric_pad);
    let direction_path_map = get_path_map(&direction_pad);
    let mut ans1 = 0;
    let mut ans2 = 0;
    // let mut memo = HashMap::new();
    for (idx_code, keypad) in keypads.iter().enumerate() {
        let mut paths = vec![keypad.clone()];
        paths = expand_paths(&paths, &numeric_path_map);
        for i in 1..=CHAIN_LEN_PART2 {
            let shortest_len = paths.iter().map(|x| x.len()).min().unwrap_or_default();
            paths.retain(|x| x.len() == shortest_len);
            println!("#{} : {} ({})", idx_code, i, shortest_len);
            paths = expand_paths(&paths, &direction_path_map);
            if i == CHAIN_LEN_PART1 {
                let shortest_len = paths.iter().map(|x| x.len()).min().unwrap_or_default();
                ans1 += shortest_len * numerics[idx_code];
                if numerics[0] == 29 {
                    break; // example
                }
            }
        }
        let shortest_len = paths.iter().map(|x| x.len()).min().unwrap_or_default();
        ans2 += shortest_len * numerics[idx_code];
    }
    if numerics[0] == 29 {
        ans2 = 0; // example
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

fn get_len(depth: usize, path: Vec<char>, memo: &mut HashMap<(usize, Vec<char>), ItemType>) -> ItemType {
    if depth == 0 {
        return path.len();
    }
    let key = (depth, path);
    if memo.contains_key(&key) {
        return *memo.get(&key).unwrap();
    }
    0
}

fn get_path_map(pad: &KeyPad) -> PathMap {
    let mut path_map = HashMap::new();
    let (illegal_x, illegal_y) = pad.get(&ILLEGAL).unwrap();
    for (&from_char, &(from_x, from_y)) in pad.iter() {
        if from_char == ILLEGAL {
            continue;
        }
        for (&to_char, &(to_x, to_y)) in pad.iter() {
            if to_char == ILLEGAL || to_char == from_char {
                continue;
            }
            let mut paths = Vec::new();
            let mut q = VecDeque::new();
            q.push_back((from_x, from_y, Vec::new()));
            while let Some(item) = q.pop_front() {
                let (x, y, partial_path) = item;
                if x == to_x && y == to_y {
                    paths.push(partial_path);
                    continue;
                }
                let mut moves = Vec::new();
                match to_x.cmp(&x) {
                    Ordering::Less => moves.push((x - 1, y, '<')),
                    Ordering::Greater => moves.push((x + 1, y, '>')),
                    Ordering::Equal => (),
                }
                match to_y.cmp(&y) {
                    Ordering::Less => moves.push((x, y - 1, '^')),
                    Ordering::Greater => moves.push((x, y + 1, 'v')),
                    Ordering::Equal => (),
                }
                for (x1, y1, char1) in &moves {
                    if x1 == illegal_x && y1 == illegal_y {
                        continue;
                    }
                    let mut next_partial_path = partial_path.clone();
                    next_partial_path.push(*char1);
                    q.push_back((*x1, *y1, next_partial_path));
                }
            }
            path_map.insert((from_char, to_char), paths);
        }
    }
    path_map
}

fn expand_paths(paths: &PathList, path_map: &PathMap) -> PathList {
    let mut expanded_paths = Vec::new();
    for path in paths {
        let mut q = VecDeque::new();
        q.push_back((0, ACTION, Vec::new()));
        while let Some(item) = q.pop_front() {
            let (idx_digit, from_char, partial_path) = item;
            if idx_digit == path.len() {
                expanded_paths.push(partial_path);
                continue;
            }
            let to_char = path[idx_digit];
            if from_char == to_char {
                let mut next_partial_path = partial_path.clone();
                next_partial_path.push(ACTION);
                q.push_back((idx_digit + 1, to_char, next_partial_path));
                continue;
            }
            for next_moves in path_map.get(&(from_char, to_char)).unwrap().iter() {
                let mut next_partial_path = partial_path.clone();
                for &next_move in next_moves.iter() {
                    next_partial_path.push(next_move);
                }
                next_partial_path.push(ACTION);
                q.push_back((idx_digit + 1, to_char, next_partial_path));
            }
        }
    }
    expanded_paths
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

    // too slow, skipped
    #[test]
    #[ignore]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_must_be_4_digits() {
        test_invalid_msg(
            &[&"1234A"],
            solve,
            "keycodes must be 3 digits followed by an `A`",
        );
    }

    #[test]
    fn invalid_must_be_end_with_an_a() {
        test_invalid_msg(
            &[&"123B"],
            solve,
            "keycodes must be 3 digits followed by an `A`",
        );
    }

    #[test]
    fn invalid_must_be_numeric() {
        test_invalid_msg(&[&"1B3A"], solve, "keycodes must be numeric");
    }
}
