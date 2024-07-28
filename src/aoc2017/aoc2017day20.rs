//! [aoc](https://adventofcode.com/2017/day/20)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2017,
        day: 20,
        title: "Particle Swarm",
        solution: ("p243", "648"),
        // extra p prefix added, because example 1 part 1 solution is 0.
        example_solutions: vec![("p0", "2"), ("p2", "1")],
    }
}

type ItemType = i32;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Vector3D {
    x: ItemType,
    y: ItemType,
    z: ItemType,
}

impl Vector3D {
    fn new(x: ItemType, y: ItemType, z: ItemType) -> Self {
        Self { x, y, z }
    }

    fn manhattan(&self) -> ItemType {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Particle {
    id: usize,
    p: Vector3D,
    v: Vector3D,
    a: Vector3D,
}

impl Particle {
    fn new(id: usize, p: Vector3D, v: Vector3D, a: Vector3D) -> Self {
        Self { id, p, v, a }
    }

    #[allow(clippy::needless_range_loop)]
    fn from_string(id: usize, line: &str) -> Result<Self, PuzzleError> {
        let r = Regex::new(r"p=<( ?-?\d+),( ?-?\d+),( ?-?\d+)>, v=<( ?-?\d+),( ?-?\d+),( ?-?\d+)>, a=<( ?-?\d+),( ?-?\d+),( ?-?\d+)>").unwrap();
        let caps = r
            .captures(line)
            .ok_or(PuzzleError("Invalid input".into()))?;
        let mut values = [0; 9];
        for i in 0..9 {
            values[i] = caps
                .get(i + 1)
                .ok_or(PuzzleError("Invalid input: missing coordinate".into()))?
                .as_str()
                .trim()
                .parse::<ItemType>()
                .map_err(|_| PuzzleError("Invalid input: coordinate must be integer".into()))?;
        }
        Ok(Particle::new(
            id,
            Vector3D::new(values[0], values[1], values[2]),
            Vector3D::new(values[3], values[4], values[5]),
            Vector3D::new(values[6], values[7], values[8]),
        ))
    }

    fn tick(&mut self) {
        self.v.x += self.a.x;
        self.v.y += self.a.y;
        self.v.z += self.a.z;
        self.p.x += self.v.x;
        self.p.y += self.v.y;
        self.p.z += self.v.z;
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Check input
    let mut particles = Vec::new();
    for (i, line) in input.iter().enumerate() {
        particles.push(Particle::from_string(i, line)?);
    }
    // println!("{:?}", particles);
    // ---------- Part 1
    particles.sort_by(|a, b| {
        a.a.manhattan()
            .cmp(&b.a.manhattan())
            .then_with(|| a.v.manhattan().cmp(&b.v.manhattan()))
            .then_with(|| a.p.manhattan().cmp(&b.p.manhattan()))
    });
    let ans1 = "p".to_string() + &particles.first().unwrap().id.to_string();
    // ---------- Part 2
    let mut ans2 = particles.len();
    let mut last_col_turn = 0;
    particles.sort_by(|a, b| a.id.cmp(&b.id));
    let mut particles_map = HashMap::new();
    for particle in particles {
        particles_map.insert(particle.id, particle);
    }
    let mut t = 0;
    loop {
        for particle in particles_map.values_mut() {
            particle.tick();
        }
        t += 1;
        let mut collisions = HashSet::new();
        let mut visited = HashMap::new();
        for particle in particles_map.values() {
            let hash = particle.p.clone();
            if !visited.contains_key(&hash) {
                visited.insert(hash.to_owned(), particle.id);
                continue;
            }
            let vis_id = visited.get(&hash).unwrap();
            collisions.insert(*vis_id);
            collisions.insert(particle.id);
        }
        if !collisions.is_empty() {
            ans2 -= collisions.len();
            for idx in collisions {
                particles_map.remove(&idx);
            }
            last_col_turn = t;
        }
        if t - last_col_turn > 50 || particles_map.len() < 2 {
            break;
        }
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
    fn invalid_variable() {
        test_invalid(
            &vec![String::from("p=<0,0,0>, x=<1,2,3>, a=<0,0,0>")],
            solve,
        );
    }

    #[test]
    fn invalid_vector_must_have_ints() {
        test_invalid(
            &vec![String::from("p=<0,a,0>, a=<1,2,3>, a=<0,0,0>")],
            solve,
        );
    }
}
