//! [aoc](https://adventofcode.com/2017/day/21)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashMap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2017,
        day: 21,
        title: "Fractal Art",
        solution: ("125", "1782917"),
        example_solutions: vec![("12", "12")],
    }
}

type ImageType = Vec<u8>;

const MAX_STEPS_EXAMPLE_PART1: usize = 2;
const MAX_STEPS_INPUT_PART1: usize = 5;
const MAX_STEPS_INPUT_PART2: usize = 18;
const START_IMAGE: &str = ".#./..#/###";

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Check input
    let mut input_rules = HashMap::new();
    for line in input {
        let mut s = line.split(" => ");
        let a = s.next().unwrap().replace('/', "").into_bytes();
        let b = s
            .next()
            .ok_or("invalid input")?
            .replace('/', "")
            .into_bytes();
        input_rules.insert(a, b);
    }
    // ---------- Part 1 + 2
    let mut ans1 = 0;
    let mut rules = HashMap::new();
    for (from_pattern, to_pattern) in &input_rules {
        let orientations = get_orientations(from_pattern)?;
        for image in orientations {
            rules.insert(image, to_pattern.to_owned());
        }
    }
    let mut image = START_IMAGE.replace('/', "").into_bytes();
    let mut size = 3;
    let part1_steps = if input_rules.len() == 2 {
        MAX_STEPS_EXAMPLE_PART1
    } else {
        MAX_STEPS_INPUT_PART1
    };
    let max_steps = if input_rules.len() == 2 {
        MAX_STEPS_EXAMPLE_PART1
    } else {
        MAX_STEPS_INPUT_PART2
    };
    for step in 1..=max_steps {
        let size_tile = if size % 2 == 0 { 2 } else { 3 };
        let max_tile = size / size_tile;
        let new_size_tile = size_tile + 1;
        let new_size = max_tile * new_size_tile;
        let mut new_image = vec![0; new_size * new_size];
        for y in 0..max_tile {
            for x in 0..max_tile {
                let mut tile = vec![0; size_tile * size_tile];
                for yt in 0..size_tile {
                    for xt in 0..size_tile {
                        tile[yt * size_tile + xt] =
                            image[(y * size_tile + yt) * size + x * size_tile + xt];
                    }
                }
                let new_tile = rules.get(&tile).ok_or("no rule exists this tile")?;
                for yt in 0..=size_tile {
                    for xt in 0..=size_tile {
                        new_image[(y * new_size_tile + yt) * new_size + x * new_size_tile + xt] =
                            new_tile[yt * new_size_tile + xt];
                    }
                }
            }
        }
        image = new_image;
        size = new_size;
        if step == part1_steps {
            ans1 = image.iter().filter(|&x| *x == b'#').count();
        }
    }
    let ans2 = image.iter().filter(|&x| *x == b'#').count();
    Ok((ans1.to_string(), ans2.to_string()))
}

fn image_size(image: &ImageType) -> Result<usize, PuzzleError> {
    match image.len() {
        4 => Ok(2),
        9 => Ok(3),
        _ => Err("invalid image size")?,
    }
}

fn rotate_right(image: &ImageType) -> Result<ImageType, PuzzleError> {
    let size = image_size(image)?;
    let mut ans = vec![0; image.len()];
    for y in 0..size {
        for x in 0..size {
            ans[x * size + size - 1 - y] = image[y * size + x];
        }
    }
    Ok(ans)
}

fn flip_x(image: &ImageType) -> Result<ImageType, PuzzleError> {
    let size = image_size(image)?;
    let mut ans = vec![0; image.len()];
    for y in 0..size {
        for x in 0..size {
            ans[y * size + x] = image[y * size + size - 1 - x];
        }
    }
    Ok(ans)
}

fn flip_y(image: &ImageType) -> Result<ImageType, PuzzleError> {
    let size = image_size(image)?;
    let mut ans = vec![0; image.len()];
    for y in 0..size {
        for x in 0..size {
            ans[y * size + x] = image[(size - 1 - y) * size + x];
        }
    }
    Ok(ans)
}

fn get_orientations(image: &ImageType) -> Result<Vec<ImageType>, PuzzleError> {
    let flips = [
        image.to_owned(),
        flip_x(image)?,
        flip_y(image)?,
        flip_y(&flip_x(image)?)?,
    ];
    let mut ans = Vec::with_capacity(4);
    for flipped_image in flips {
        ans.push(flipped_image.to_owned());
        let mut rotated_image = flipped_image.to_owned();
        for _ in 1..4 {
            rotated_image = rotate_right(&rotated_image)?;
            ans.push(rotated_image.clone());
        }
    }
    Ok(ans)
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
    fn invalid_missing_arrow() {
        test_invalid(&["a"], solve);
    }

    #[test]
    fn invalid_missing_rule() {
        test_invalid(&["../.# => ##./#../..."], solve);
    }
}
