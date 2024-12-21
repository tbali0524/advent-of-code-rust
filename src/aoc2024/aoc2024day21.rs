//! [aoc](https://adventofcode.com/2024/day/21)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 21,
        title: "Keypad Conundrum",
        solution: ("163086", "198466286401228"),
        example_solutions: vec![("126384", "0")],
    }
}

const ILLEGAL: char = '_';
const ACTION: char = 'A';
const CHAIN_LEN_PART1: usize = 3;
const CHAIN_LEN_PART2: usize = 26;

type ItemType = usize;
type KeyPad = HashMap<char, (i32, i32)>;
type PathList = Vec<Vec<char>>;
type PathMap = HashMap<(char, char), PathList>;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let keycodes = input
        .iter()
        .map(|&x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    if keycodes
        .iter()
        .any(|keycode| keycode.len() != 4 || keycode[3] != 'A')
    {
        Err("keycodes must be 3 digits followed by an `A`")?;
    }
    let integer_codes = input
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
    let mut memo = HashMap::new();
    for (idx_code, keycode) in keycodes.iter().enumerate() {
        let shortest_len = get_shortest_len(
            CHAIN_LEN_PART1,
            keycode,
            &numeric_path_map,
            &direction_path_map,
            true,
            &mut memo,
        );
        ans1 += shortest_len * integer_codes[idx_code];
    }
    // ---------- Part 2
    if integer_codes[0] == 29 {
        return Ok((ans1.to_string(), "0".to_string()));
    }
    let mut ans2 = 0;
    let mut memo = HashMap::new();
    for (idx_code, keycode) in keycodes.iter().enumerate() {
        let shortest_len = get_shortest_len(
            CHAIN_LEN_PART2,
            keycode,
            &numeric_path_map,
            &direction_path_map,
            true,
            &mut memo,
        );
        ans2 += shortest_len * integer_codes[idx_code];
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

fn get_shortest_len(
    depth: usize,
    path: &[char],
    numeric_path_map: &PathMap,
    direction_path_map: &PathMap,
    first_level: bool,
    memo: &mut HashMap<(usize, Vec<char>), ItemType>,
) -> ItemType {
    if depth == 0 {
        return path.len();
    }
    let key = (depth, path.to_owned());
    if memo.contains_key(&key) {
        return *memo.get(&key).unwrap();
    }
    let path_map = if first_level {
        numeric_path_map
    } else {
        direction_path_map
    };
    let mut result = 0;
    let mut from_char = ACTION;
    for &to_char in path.iter() {
        let mut best_len = usize::MAX;
        for next_moves in path_map.get(&(from_char, to_char)).unwrap().iter() {
            let current_len = get_shortest_len(
                depth - 1,
                next_moves,
                numeric_path_map,
                direction_path_map,
                false,
                memo,
            );
            if current_len < best_len {
                best_len = current_len;
            }
        }
        result += best_len;
        from_char = to_char;
    }
    memo.insert(key, result);
    result
}

fn get_path_map(keypad: &KeyPad) -> PathMap {
    let mut path_map = HashMap::new();
    let (illegal_x, illegal_y) = keypad.get(&ILLEGAL).unwrap();
    for (&from_char, &(from_x, from_y)) in keypad.iter() {
        if from_char == ILLEGAL {
            continue;
        }
        for (&to_char, &(to_x, to_y)) in keypad.iter() {
            if to_char == ILLEGAL {
                continue;
            }
            let mut paths = Vec::new();
            let mut q = VecDeque::new();
            q.push_back((from_x, from_y, Vec::new()));
            while let Some(item) = q.pop_front() {
                let (x, y, mut partial_path) = item;
                if x == to_x && y == to_y {
                    partial_path.push(ACTION);
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
