//! [aoc](https://adventofcode.com/2015/day/22)

use crate::aoc::{PuzzleError, PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::cmp;
use std::collections::BinaryHeap;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2015,
        day: 22,
        title: "izard Simulator 20XX",
        solution: ("900", "1216"),
        example_solutions: vec![],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() != 2 {
        Err("input must have 2 lines")?;
    }
    if !input[0].starts_with("Hit Points: ") || !input[1].starts_with("Damage: ") {
        Err("invalid input format")?;
    }
    let hp = input[0][12..]
        .parse::<ItemType>()
        .map_err(|_| "hp must be an integer")?;
    let damage = input[1][8..]
        .parse::<ItemType>()
        .map_err(|_| "damage must be an integer")?;
    // ---------- Part 1 + 2
    let ans1 = WizardSimulator::new(hp, damage).simulate()?;
    let ans2 = WizardSimulator::new_hard_mode(hp, damage).simulate()?;
    Ok((ans1.to_string(), ans2.to_string()))
}

// enum SpellType {
//     MagicMissile,
//     Drain,
//     Shield,
//     Poison,
//     Recharge,
// }
// const SPELLS: [SpellType; 5] = [
//     SpellType::MagicMissile,
//     SpellType::Drain,
//     SpellType::Shield,
//     SpellType::Poison,
//     SpellType::Recharge,
// ];

const SPELL_MAGIC_MISSILE: usize = 0;
const SPELL_DRAIN: usize = 1;
const SPELL_SHIELD: usize = 2;
const SPELL_POISON: usize = 3;
const SPELL_RECHARGE: usize = 4;

const MAX_SPELLS: usize = 5;

#[expect(dead_code)]
struct Spell {
    spell_type: usize,
    cost: ItemType,
    duration: ItemType,
    value: ItemType,
}

const SPELLS: [Spell; MAX_SPELLS] = [
    Spell {
        spell_type: SPELL_MAGIC_MISSILE,
        cost: 53,
        duration: 0,
        value: 4,
    },
    Spell {
        spell_type: SPELL_DRAIN,
        cost: 73,
        duration: 0,
        value: 2,
    },
    Spell {
        spell_type: SPELL_SHIELD,
        cost: 113,
        duration: 6,
        value: 7,
    },
    Spell {
        spell_type: SPELL_POISON,
        cost: 173,
        duration: 6,
        value: 3,
    },
    Spell {
        spell_type: SPELL_RECHARGE,
        cost: 229,
        duration: 5,
        value: 101,
    },
];

const PLAYER_START_HP: ItemType = 50;
const PLAYER_START_MANA: ItemType = 500;

#[derive(Copy, Clone, Eq, PartialEq)]
struct WizardGameState {
    hard_mode: bool,
    enemy_hp: ItemType,
    enemy_damage: ItemType,
    hp: ItemType,
    mana: ItemType,
    spent_mana: ItemType,
    armor: ItemType,
    timers: [ItemType; MAX_SPELLS],
}

impl WizardGameState {
    fn new(enemy_hp: ItemType, enemy_damage: ItemType, hard_mode: bool) -> Self {
        WizardGameState {
            hard_mode,
            enemy_hp,
            enemy_damage,
            hp: PLAYER_START_HP,
            mana: PLAYER_START_MANA,
            spent_mana: 0,
            armor: 0,
            timers: [0; MAX_SPELLS],
        }
    }

    #[expect(clippy::needless_range_loop)]
    fn all_valid_spells(&self) -> Vec<usize> {
        let mut spells = Vec::new();
        if self.enemy_hp <= 0 || self.hp <= 0 {
            return spells;
        }
        for spell in 0..MAX_SPELLS {
            if self.mana < SPELLS[spell].cost {
                continue;
            }
            if self.timers[spell] > 1 {
                continue;
            }
            spells.push(spell);
        }
        spells
    }

    // simulates a full Player turn (with casting spell) and also a Boss turn
    fn apply_move(&mut self, spell: usize) {
        if spell >= MAX_SPELLS || self.mana < SPELLS[spell].cost || self.timers[spell] > 1 {
            return;
        }
        // Player turn
        if self.hard_mode {
            self.hp -= 1;
            if self.hp <= 0 {
                return;
            }
        }
        self.apply_effects();
        if self.enemy_hp <= 0 {
            return;
        }
        self.mana -= SPELLS[spell].cost;
        self.spent_mana += SPELLS[spell].cost;
        if spell == SPELL_MAGIC_MISSILE || spell == SPELL_DRAIN {
            self.enemy_hp = cmp::max(0, self.enemy_hp - SPELLS[spell].value);
        }
        if spell == SPELL_DRAIN {
            self.hp += SPELLS[SPELL_DRAIN].value;
        }
        if SPELLS[spell].duration > 0 {
            self.timers[spell] = SPELLS[spell].duration;
        }
        if self.enemy_hp <= 0 {
            return;
        }
        // Boss turn
        self.apply_effects();
        if self.enemy_hp <= 0 {
            return;
        }
        let shield = if self.timers[SPELL_SHIELD] > 0 {
            SPELLS[SPELL_SHIELD].value
        } else {
            0
        };
        self.hp = cmp::max(
            0,
            self.hp - cmp::max(1, self.enemy_damage - self.armor - shield),
        );
    }

    fn apply_effects(&mut self) {
        for (spell, timer) in self.timers.iter_mut().enumerate() {
            if *timer == 0 {
                continue;
            }
            if spell == SPELL_POISON {
                self.enemy_hp = cmp::max(0, self.enemy_hp - SPELLS[SPELL_POISON].value);
            } else if spell == SPELL_RECHARGE {
                self.mana += SPELLS[SPELL_RECHARGE].value;
            }
            *timer -= 1;
        }
    }
}

impl Ord for WizardGameState {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        other.spent_mana.cmp(&self.spent_mana)
    }
}

impl PartialOrd for WizardGameState {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct WizardSimulator {
    start_state: WizardGameState,
    priority_queue: BinaryHeap<WizardGameState>,
}

impl WizardSimulator {
    fn new(hp: ItemType, damage: ItemType) -> Self {
        Self {
            start_state: WizardGameState::new(hp, damage, false),
            priority_queue: BinaryHeap::new(),
        }
    }

    fn new_hard_mode(hp: ItemType, damage: ItemType) -> Self {
        Self {
            start_state: WizardGameState::new(hp, damage, true),
            priority_queue: BinaryHeap::new(),
        }
    }

    fn simulate(&mut self) -> Result<ItemType, PuzzleError> {
        self.priority_queue.push(self.start_state);
        loop {
            let current_state = self.priority_queue.pop().ok_or("no solution found")?;
            if current_state.enemy_hp <= 0 {
                return Ok(current_state.spent_mana);
            }
            let spells = current_state.all_valid_spells();
            for spell in spells {
                let mut next_state = current_state;
                next_state.apply_move(spell);
                self.priority_queue.push(next_state);
            }
        }
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
    fn invalid_must_be_2_lines() {
        test_invalid(&["a"], solve);
    }

    #[test]
    fn invalid_must_start_with_literals() {
        test_invalid(&["Hit Points: 109", "D: 8"], solve);
    }

    #[test]
    fn invalid_hp_must_be_integer() {
        test_invalid(&["Hit Points: X", "Damage: 8"], solve);
    }

    #[test]
    fn invalid_damage_must_be_integer() {
        test_invalid(&["Hit Points: 109", "Damage: X"], solve);
    }
}
