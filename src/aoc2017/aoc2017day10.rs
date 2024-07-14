//! [aoc](https://adventofcode.com/2017/day/10)

use crate::aoc::PuzzleMetaData;
use crate::aoc::PuzzleResult;

pub const PUZZLE_METADATA: PuzzleMetaData<'static> = PuzzleMetaData {
    year: 2017,
    day: 10,
    title: "Knot Hash",
    solution: (0, 0),
    example_solutions: [(0, 0), (0, 0)],
    string_solution: Some(("52070", "7f94112db4e32e19cf6502073c66f9bb")),
    example_string_solutions: Some([("12", "0"), ("0", "3efbe78a8d82f29979031a4aa0b16a9d")]),
    example_string_inputs: Some(["3,4,1,5", "1,2,3"]),
};

type ItemType = usize;

#[allow(clippy::explicit_counter_loop)]
pub fn solve(input: &[String]) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() != 1 {
        return Err("Input must have a single line");
    }
    let data = input[0]
        .split(',')
        .map(|x| {
            x.parse::<ItemType>()
                .map_err(|_| "Input must contain only integers")
        })
        .collect::<Result<Vec<_>, _>>()?;
    // ---------- Part 1
    const LIST_SIZE_EXAMPLE1_PART1: usize = 5;
    const LIST_SIZE_INPUT: usize = 256;
    let list_size = if data.len() == 4 {
        LIST_SIZE_EXAMPLE1_PART1
    } else {
        LIST_SIZE_INPUT
    };
    let mut list = (0..list_size).collect::<Vec<_>>();
    let mut pos = 0;
    let mut skip_size = 0;
    for len in data {
        if len > list_size {
            Err("Invalid input")?
        }
        for i in 0..(len / 2) {
            let p1 = (pos + i) % list_size;
            let p2 = (pos + len - 1 - i) % list_size;
            list.swap(p1, p2);
        }
        pos = (pos + len + skip_size) % list_size;
        skip_size += 1;
    }
    let ans1 = if list.len() >= 2 {
        list[0] * list[1]
    } else {
        0
    };
    // ---------- Part 2
    let mut ans2 = String::new();
    let mut data = input[0].as_bytes().to_owned();
    let mut extra = vec![17u8, 31, 73, 47, 23];
    data.append(&mut extra);
    let list_size = LIST_SIZE_INPUT;
    let mut list = (0..list_size).collect::<Vec<_>>();
    let mut pos = 0;
    let mut skip_size = 0;
    for _ in 0..64 {
        for len_u8 in &data {
            let len = *len_u8 as usize;
            for i in 0..(len / 2) {
                let p1 = (pos + i) % list_size;
                let p2 = (pos + len - 1 - i) % list_size;
                list.swap(p1, p2);
            }
            pos = (pos + len + skip_size) % list_size;
            skip_size += 1;
        }
    }
    for i in 0..16 {
        let mut byte = 0;
        for j in 0..16 {
            byte ^= list[i * 16 + j];
        }
        ans2 += &format!("{:02x}", byte);
    }
    Ok((ans1.to_string(), ans2.to_string()))
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
    fn example2() {
        test_case(&PUZZLE_METADATA, 2, solve);
    }

    #[test]
    fn puzzle() {
        test_case(&PUZZLE_METADATA, 0, solve);
    }

    #[test]
    fn invalid_single_line() {
        test_invalid(
            &PUZZLE_METADATA,
            &[String::from("123"), String::from("1")],
            solve,
        );
    }

    #[test]
    fn invalid_only_contains_int() {
        test_invalid(&PUZZLE_METADATA, &[String::from("1,a,3")], solve);
    }
}
