//! [aoc](https://adventofcode.com/2015/day/14)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::cmp;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 14,
        title: "Reindeer Olympic",
        solution: ("2696", "1084"),
        example_solutions: vec![("1120", "689")],
    }
}

type ItemType = i32;

const STEPS: ItemType = 2503;
const EXAMPLE_STEPS: ItemType = 1000;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Check input
    // detect puzzle example as input
    let max_steps = if input.len() == 2 {
        EXAMPLE_STEPS
    } else {
        STEPS
    };
    // ---------- Part 1
    let mut reindeers = input
        .iter()
        .map(|&row| Reindeer::try_from(row))
        .collect::<Result<Vec<_>, PuzzleError>>()?;
    let ans1 = reindeers
        .iter()
        .map(|r| r.distance_at(max_steps))
        .max()
        .unwrap();
    // ---------- Part 2
    for second in 1..=max_steps {
        let max = reindeers
            .iter()
            .map(|r| r.distance_at(second))
            .max()
            .unwrap();
        for reindeer in reindeers.iter_mut() {
            if reindeer.distance_at(second) == max {
                reindeer.points += 1;
            }
        }
    }
    let ans2 = reindeers.iter().max_by_key(|&x| x.points).unwrap().points;
    Ok((ans1.to_string(), ans2.to_string()))
}

#[derive(Default)]
struct Reindeer {
    name: String,
    speed: ItemType,
    fly_time: ItemType,
    rest_time: ItemType,
    points: ItemType,
}

impl TryFrom<&str> for Reindeer {
    type Error = PuzzleError;

    #[expect(clippy::field_reassign_with_default)]
    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let a = line.split(' ').collect::<Vec<_>>();
        if a.len() != 15
            || a[1] != "can"
            || a[2] != "fly"
            || a[4] != "km/s"
            || a[5] != "for"
            || a[14] != "seconds."
            || !line.contains(" seconds, but then must rest for ")
        {
            Err("invalid line")?;
        }
        let mut r = Reindeer::default();
        r.name = a[0].to_owned();
        r.speed = a[3]
            .parse::<ItemType>()
            .map_err(|_| format!("speed must be integer, found `{}`", a[3]))?;
        r.fly_time = a[6]
            .parse::<ItemType>()
            .map_err(|_| format!("fly time must be integer, found `{}`", a[6]))?;
        r.rest_time = a[13]
            .parse::<ItemType>()
            .map_err(|_| format!("rest time must be integer, found `{}`", a[13]))?;
        Ok(r)
    }
}

impl Reindeer {
    fn distance_at(&self, seconds: ItemType) -> ItemType {
        let cycles = seconds / (self.fly_time + self.rest_time);
        let remaining = cmp::min(self.fly_time, seconds % (self.fly_time + self.rest_time));
        self.speed * (cycles * self.fly_time + remaining)
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
    fn invalid_input_line() {
        test_invalid(&["a"], solve);
    }

    #[test]
    fn invalid_speed_must_be_integer() {
        test_invalid(
            &["A can fly X km/s for 10 seconds, but then must rest for 127 seconds."],
            solve,
        );
    }

    #[test]
    fn invalid_fly_time_must_be_integer() {
        test_invalid(
            &["A can fly 14 km/s for X seconds, but then must rest for 127 seconds."],
            solve,
        );
    }

    #[test]
    fn invalid_rest_time_must_be_integer() {
        test_invalid(
            &["A can fly 14 km/s for 10 seconds, but then must rest for X seconds."],
            solve,
        );
    }
}
