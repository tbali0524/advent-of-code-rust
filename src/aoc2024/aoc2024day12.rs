//! [aoc](https://adventofcode.com/2024/day/12)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 12,
        title: "Garden Groups",
        solution: ("1344578", "814302"),
        example_solutions: vec![
            ("140", "80"),
            ("772", "436"),
            ("1930", "1206"),
            ("0", "236"),
            ("0", "368"),
        ],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let max_y = input.len() as i32;
    let max_x = input[0].len() as i32;
    let grid = input
        .iter()
        .map(|&x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    if grid.iter().any(|line| line.len() != max_x as usize) {
        Err("grid must be rectangular")?;
    }
    // ---------- Part 1 + 2
    let mut region_grid = vec![vec![None; max_x as usize]; max_y as usize];
    let mut side_grid = vec![vec![[None; 4]; max_x as usize]; max_y as usize];
    let mut areas = Vec::new();
    let mut perimeters = Vec::new();
    let mut count_regions = 0usize;
    for start_y in 0..max_y {
        for start_x in 0..max_x {
            if region_grid[start_y as usize][start_x as usize].is_some() {
                continue;
            }
            count_regions += 1;
            let region_char = grid[start_y as usize][start_x as usize];
            areas.push(0);
            perimeters.push(0);
            let mut q = Vec::new();
            q.push((start_x, start_y));
            region_grid[start_y as usize][start_x as usize] = Some(count_regions - 1);
            let mut idx_read = 0;
            while idx_read < q.len() {
                let (x, y) = q[idx_read];
                idx_read += 1;
                areas[count_regions - 1] += 1;
                for (dir, (dx, dy)) in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter().enumerate() {
                    let x1 = x + dx;
                    let y1 = y + dy;
                    if x1 < 0
                        || x1 >= max_x
                        || y1 < 0
                        || y1 >= max_y
                        || grid[y1 as usize][x1 as usize] != region_char
                    {
                        perimeters[count_regions - 1] += 1;
                        side_grid[y as usize][x as usize][dir] = Some(count_regions - 1);
                        continue;
                    }
                    if region_grid[y1 as usize][x1 as usize].is_some() {
                        continue;
                    }
                    region_grid[y1 as usize][x1 as usize] = Some(count_regions - 1);
                    q.push((x1, y1));
                }
            }
        }
    }
    let mut ans1 = 0;
    for i in 0..count_regions {
        ans1 += areas[i] * perimeters[i];
    }
    let mut sides = vec![0; count_regions];
    for dir in 0..4 {
        if dir % 2 == 0 {
            for x in 0..max_x {
                for y in 0..max_y {
                    if let Some(idx_region) = side_grid[y as usize][x as usize][dir] {
                        if y == 0
                            || side_grid[(y - 1) as usize][x as usize][dir] != Some(idx_region)
                        {
                            sides[idx_region] += 1;
                        }
                    }
                }
            }
        } else {
            for y in 0..max_y {
                for x in 0..max_x {
                    if let Some(idx_region) = side_grid[y as usize][x as usize][dir] {
                        if x == 0
                            || side_grid[y as usize][(x - 1) as usize][dir] != Some(idx_region)
                        {
                            sides[idx_region] += 1;
                        }
                    }
                }
            }
        }
    }
    let mut ans2 = 0;
    for i in 0..count_regions {
        ans2 += areas[i] * sides[i];
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
    fn example3() {
        test_case(metadata, solve, 3);
    }

    #[test]
    fn example4() {
        test_case(metadata, solve, 4);
    }

    #[test]
    fn example5() {
        test_case(metadata, solve, 5);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_grid_not_rectangular() {
        test_invalid_msg(&[&"AB", &"A"], solve, "grid must be rectangular");
    }
}
