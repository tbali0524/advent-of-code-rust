//! [aoc](https://adventofcode.com/2024/day/15)

use crate::aoc::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 15,
        title: "Warehouse Woes",
        solution: ("1383666", "1412866"),
        example_solutions: vec![("2028", "0"), ("10092", "9021"), ("0", "618")],
    }
}

const DEBUG: bool = false;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    let mut grid = Vec::new();
    let mut max_y = 0;
    let mut max_x = 0;
    let mut start_x = 0;
    let mut start_y = 0;
    let mut has_start = false;
    let mut i = 0;
    while i < input.len() && !input[i].is_empty() {
        grid.push(Vec::new());
        for (x, c) in input[i].chars().enumerate() {
            let cell = match c {
                '@' => {
                    start_x = x as i32;
                    start_y = i as i32;
                    has_start = true;
                    Cell::Empty
                }
                '.' => Cell::Empty,
                '#' => Cell::Wall,
                'O' => Cell::Box,
                _ => Err("invalid character in grid")?,
            };
            grid[i].push(cell);
        }
        if i == 0 {
            max_x = grid[0].len() as i32;
        } else if grid[i].len() as i32 != max_x {
            Err("grid must be rectangular")?;
        }
        max_y += 1;
        i += 1;
    }
    if !has_start {
        Err("missing start position in grid")?;
    }
    if i == input.len() {
        Err("instructions must be separated from the map by an empty line")?;
    }
    let mut instructions = Vec::new();
    while i < input.len() {
        for c in input[i].chars() {
            let dxy = match c {
                '>' => (1, 0),
                'v' => (0, 1),
                '<' => (-1, 0),
                '^' => (0, -1),
                _ => Err("invalid character in instruction")?,
            };
            instructions.push(dxy);
        }
        i += 1;
    }
    // ---------- Part 1
    let start_grid = grid.clone();
    let mut x = start_x;
    let mut y = start_y;
    for (dx, dy) in instructions.iter() {
        let mut is_ok = true;
        let mut x1 = x;
        let mut y1 = y;
        loop {
            x1 += dx;
            y1 += dy;
            if y1 < 0 || y1 >= max_y || x1 < 0 || x1 >= max_x {
                is_ok = false;
                break;
            }
            match grid[y1 as usize][x1 as usize] {
                Cell::Empty => break,
                Cell::Box => continue,
                Cell::Wall => {
                    is_ok = false;
                    break;
                }
                _ => unreachable!(),
            }
        }
        if !is_ok {
            continue;
        }
        x += dx;
        y += dy;
        if y != y1 || x != x1 {
            grid[y as usize][x as usize] = Cell::Empty;
            grid[y1 as usize][x1 as usize] = Cell::Box;
        }
    }
    let mut ans1 = 0;
    for y in 0..max_y {
        for x in 0..max_x {
            if grid[y as usize][x as usize] == Cell::Box {
                ans1 += 100 * y + x;
            }
        }
    }
    // ---------- Part 2
    grid = Vec::new();
    for y in 0..max_y {
        grid.push(Vec::new());
        for x in 0..max_x {
            match start_grid[y as usize][x as usize] {
                Cell::Empty => {
                    grid[y as usize].push(Cell::Empty);
                    grid[y as usize].push(Cell::Empty);
                }
                Cell::Box => {
                    grid[y as usize].push(Cell::Box);
                    grid[y as usize].push(Cell::BoxRight);
                }
                Cell::Wall => {
                    grid[y as usize].push(Cell::Wall);
                    grid[y as usize].push(Cell::Wall);
                }
                _ => unreachable!(),
            }
        }
    }
    start_x *= 2;
    max_x *= 2;
    let mut x = start_x;
    let mut y = start_y;
    let mut k = 1;
    for (dx, dy) in instructions.iter() {
        if DEBUG {
            show_grid(&grid, x, y);
            println!("------ Turn # {k} : ({dx}, {dy})");
            k += 1;
        }
        let mut x1 = x;
        let mut y1 = y;
        x1 += dx;
        y1 += dy;
        if y1 < 0 || y1 >= max_y || x1 < 0 || x1 >= max_x {
            continue;
        }
        let mut moving_boxes = Vec::new();
        match grid[y1 as usize][x1 as usize] {
            Cell::Wall => continue,
            Cell::Empty => {
                x = x1;
                y = y1;
                continue;
            }
            Cell::Box => {
                moving_boxes.push((x1, y1));
            }
            Cell::BoxRight => {
                moving_boxes.push((x1 - 1, y1));
            }
        }
        let mut is_ok = true;
        let mut idx = 0;
        while idx < moving_boxes.len() {
            let (xb, yb) = moving_boxes[idx];
            idx += 1;
            x1 = xb + dx;
            y1 = yb + dy;
            if y1 < 0 || y1 >= max_y || x1 < 0 || x1 >= max_x - 1 {
                is_ok = false;
                break;
            }
            let c1 = &grid[y1 as usize][x1 as usize];
            let c2 = &grid[y1 as usize][(x1 + 1) as usize];
            if *c1 == Cell::Wall || *c2 == Cell::Wall {
                is_ok = false;
                break;
            }
            if *c1 == Cell::Box {
                moving_boxes.push((x1, y1));
            }
            if *c1 == Cell::BoxRight && (x1 != xb + 1 || y1 != yb) {
                moving_boxes.push((x1 - 1, y1));
            }
            if *c2 == Cell::Box && (x1 + 1 != xb || y1 != yb) {
                moving_boxes.push((x1 + 1, y1));
            }
        }
        if DEBUG {
            println!("trying to move boxes: {:?}:", &moving_boxes);
        }
        if !is_ok {
            if DEBUG {
                println!("...blocked");
            }
            continue;
        }
        for (xb, yb) in moving_boxes.iter() {
            grid[*yb as usize][*xb as usize] = Cell::Empty;
            grid[*yb as usize][(*xb + 1) as usize] = Cell::Empty;
        }
        for (xb, yb) in moving_boxes.iter() {
            x1 = xb + dx;
            y1 = yb + dy;
            grid[y1 as usize][x1 as usize] = Cell::Box;
            grid[y1 as usize][(x1 + 1) as usize] = Cell::BoxRight;
        }
        x += dx;
        y += dy;
    }
    let mut ans2 = 0;
    for y in 0..max_y {
        for x in 0..max_x {
            if grid[y as usize][x as usize] == Cell::Box {
                ans2 += 100 * y + x;
            }
        }
    }
    if DEBUG {
        show_grid(&grid, x, y);
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

#[derive(PartialEq, Clone)]
enum Cell {
    Empty,
    Wall,
    Box,
    BoxRight,
}

fn show_grid(grid: &[Vec<Cell>], pos_x: i32, pos_y: i32) {
    for (y, row) in grid.iter().enumerate() {
        let mut s = String::new();
        for (x, c) in row.iter().enumerate() {
            let char = if pos_x == x as i32 && pos_y == y as i32 {
                '@'
            } else {
                match c {
                    Cell::Empty => '.',
                    Cell::Wall => '#',
                    Cell::Box => '[',
                    Cell::BoxRight => ']',
                }
            };
            s.push(char);
        }
        println!("{s}");
    }
    println!();
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
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_missing_pos() {
        test_invalid_msg(
            &["#.", ".#", "", ">"],
            solve,
            "missing start position in grid",
        );
    }

    #[test]
    fn invalid_character() {
        test_invalid_msg(&["a@", "", ">"], solve, "invalid character in grid");
    }

    #[test]
    fn invalid_grid_not_rectangular() {
        test_invalid_msg(&["#.", "@", "", ">"], solve, "grid must be rectangular");
    }

    #[test]
    fn invalid_character_instruction() {
        test_invalid_msg(&["@.", "", ">a"], solve, "invalid character in instruction");
    }
}
