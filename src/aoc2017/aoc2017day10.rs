//! [aoc](https://adventofcode.com/2017/day/10)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2017,
        day: 10,
        title: "Knot Hash",
        solution: ("52070", "7f94112db4e32e19cf6502073c66f9bb"),
        example_solutions: vec![("12", "0"), ("0", "3efbe78a8d82f29979031a4aa0b16a9d")],
    }
}

type ItemType = usize;

#[allow(clippy::explicit_counter_loop)]
pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() != 1 {
        return Err(PuzzleError("input must have a single line".into()));
    }
    let data = input[0]
        .split(',')
        .map(|x| {
            x.parse::<ItemType>()
                .map_err(|_| PuzzleError("input must contain only integers".into()))
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
            Err(PuzzleError("invalid input".into()))?
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::runner::tests::*;

    #[test]
    fn example1() {
        test_case(metadata, solve, 1);
    }

    #[test]
    fn example2() {
        test_case(metadata, solve, 2);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_single_line() {
        test_invalid(&[&"123", &"1"], solve);
    }

    #[test]
    fn invalid_only_contains_int() {
        test_invalid(&[&"1,a,3"], solve);
    }
}
