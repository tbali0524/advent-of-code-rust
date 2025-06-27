//! [aoc](https://adventofcode.com/2015/day/21)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::cmp;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 21,
        title: "RPG Simulator 20XX",
        solution: ("111", "188"),
        example_solutions: vec![],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Part 1 + 2
    let enemy = Character::new(input)?;
    let mut ans1 = ItemType::MAX;
    let mut ans2 = 0;
    for weapon in WEAPONS {
        for armor in ARMORS {
            for (id1, ring1) in RINGS.iter().enumerate() {
                for (id2, ring2) in RINGS.iter().enumerate() {
                    if id1 != 0 && id1 == id2 {
                        continue;
                    }
                    let cost = weapon[0] + armor[0] + ring1[0] + ring2[0];
                    let damage = weapon[1] + armor[1] + ring1[1] + ring2[1];
                    let sum_armor = weapon[2] + armor[2] + ring1[2] + ring2[2];
                    let player = Character {
                        hp: PLAYER_HP,
                        damage,
                        armor: sum_armor,
                    };
                    if player.can_win(&enemy) {
                        ans1 = cmp::min(ans1, cost);
                    } else {
                        ans2 = cmp::max(ans2, cost);
                    }
                }
            }
        }
    }
    Ok((ans1.to_string(), ans2.to_string()))
}

const PLAYER_HP: ItemType = 100;
// cost, damage, armor
const WEAPONS: [[ItemType; 3]; 5] = [[8, 4, 0], [10, 5, 0], [25, 6, 0], [40, 7, 0], [74, 8, 0]];
const ARMORS: [[ItemType; 3]; 6] = [
    [0, 0, 0], // armor is optional
    [13, 0, 1],
    [31, 0, 2],
    [53, 0, 3],
    [75, 0, 4],
    [102, 0, 5],
];
const RINGS: [[ItemType; 3]; 7] = [
    [0, 0, 0], // ring is optional
    [25, 1, 0],
    [50, 2, 0],
    [100, 3, 0],
    [20, 0, 1],
    [40, 0, 2],
    [80, 0, 3],
];

struct Character {
    hp: ItemType,
    damage: ItemType,
    armor: ItemType,
}

impl Character {
    fn new(input: PuzzleInput) -> Result<Self, PuzzleError> {
        if input.len() != 3 {
            Err("input must have 3 lines")?;
        }
        if !input[0].starts_with("Hit Points: ")
            || !input[1].starts_with("Damage: ")
            || !input[2].starts_with("Armor: ")
        {
            Err("invalid input format")?;
        }
        let hp = input[0][12..]
            .parse::<ItemType>()
            .map_err(|_| "hp must be an integer")?;
        let damage = input[1][8..]
            .parse::<ItemType>()
            .map_err(|_| "damage must be an integer")?;
        let armor = input[2][7..]
            .parse::<ItemType>()
            .map_err(|_| "armor must be an integer")?;
        Ok(Character { hp, damage, armor })
    }

    fn can_win(&self, enemy: &Character) -> bool {
        let turns_to_win =
            (enemy.hp as f64 / cmp::max(1, self.damage - enemy.armor) as f64).ceil() as ItemType;
        let turns_to_loose =
            (self.hp as f64 / cmp::max(1, enemy.damage - self.armor) as f64).ceil() as ItemType;
        turns_to_win <= turns_to_loose
    }
}

// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::runner::tests::*;

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    fn invalid_must_be_3_lines() {
        test_invalid(&["a"], solve);
    }

    #[test]
    fn invalid_must_start_with_literals() {
        test_invalid(&["Hit Points: 109", "D: 8", "Armor: 2"], solve);
    }

    #[test]
    fn invalid_hp_must_be_integer() {
        test_invalid(&["Hit Points: X", "Damage: 8", "Armor: 2"], solve);
    }

    #[test]
    fn invalid_damage_must_be_integer() {
        test_invalid(&["Hit Points: 109", "Damage: X", "Armor: 2"], solve);
    }

    #[test]
    fn invalid_armor_must_be_integer() {
        test_invalid(&["Hit Points: 109", "Damage: 8", "Armor: X"], solve);
    }
}
