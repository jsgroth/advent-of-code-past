//! Day 22: Wizard Simulator 20XX
//! https://adventofcode.com/2015/day/22

use std::cmp;
use std::error::Error;
use crate::SimpleError;

struct Boss {
    hit_points: i32,
    damage: i32,
}

#[derive(Debug, PartialEq, Eq)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    const SPELLS: [Self; 5] = [Self::MagicMissile, Self::Drain, Self::Shield, Self::Poison, Self::Recharge];

    const MAGIC_MISSILE_DAMAGE: i32 = 4;
    const DRAIN_DAMAGE: i32 = 2;
    const SHIELD_ARMOR: i32 = 7;
    const POISON_DAMAGE: i32 = 3;
    const RECHARGE_BOON: i32 = 101;

    const SHIELD_DURATION: usize = 6;
    const POISON_DURATION: usize = 6;
    const RECHARGE_DURATION: usize = 5;

    fn mana_cost(&self) -> i32 {
        match self {
            Self::MagicMissile => 53,
            Self::Drain => 73,
            Self::Shield => 113,
            Self::Poison => 173,
            Self::Recharge => 229,
        }
    }

    fn min_mana_cost() -> i32 {
        Self::SPELLS.iter().map(|spell| spell.mana_cost()).min().unwrap()
    }
}

struct SearchState {
    player_hp: i32,
    player_mana: i32,
    boss_hp: i32,
    spent_mana: i32,
    shield_timer: usize,
    poison_timer: usize,
    recharge_timer: usize,
}

impl SearchState {
    fn new(boss_hp: i32) -> Self {
        Self {
            player_hp: 50,
            player_mana: 500,
            boss_hp,
            spent_mana: 0,
            shield_timer: 0,
            poison_timer: 0,
            recharge_timer: 0,
        }
    }

    fn process_spell_effects(&mut self) {
        if self.recharge_timer > 0 {
            self.player_mana += Spell::RECHARGE_BOON;
            self.recharge_timer -= 1;
        }

        if self.poison_timer > 0 {
            self.boss_hp -= Spell::POISON_DAMAGE;
            self.poison_timer -= 1;
        }

        self.shield_timer = self.shield_timer.saturating_sub(1);
    }
}

fn solve_part_1(input: &str) -> Result<i32, SimpleError> {
    let boss = parse_input(input)?;

    Ok(search_for_min_cost(&boss, SearchState::new(boss.hit_points), false, &mut i32::MAX.clone()))
}

fn solve_part_2(input: &str) -> Result<i32, SimpleError> {
    let boss = parse_input(input)?;

    Ok(search_for_min_cost(&boss, SearchState::new(boss.hit_points), true, &mut i32::MAX.clone()))
}

fn search_for_min_cost(boss: &Boss, state: SearchState, hard_mode: bool, min_so_far: &mut i32) -> i32 {
    if state.spent_mana >= *min_so_far {
        return i32::MAX;
    }

    let mut state = state;

    // Player turn timer effects
    if hard_mode {
        state.player_hp -= 1;
        if state.player_hp <= 0 {
            return i32::MAX;
        }
    }

    state.process_spell_effects();

    if state.boss_hp <= 0 {
        *min_so_far = cmp::min(*min_so_far, state.spent_mana);
        return state.spent_mana;
    }

    if state.player_mana < Spell::min_mana_cost() {
        return i32::MAX;
    }

    let mut result = i32::MAX;
    for spell in &Spell::SPELLS {
        if (*spell == Spell::Shield && state.shield_timer > 0) ||
            (*spell == Spell::Poison && state.poison_timer > 0) ||
            (*spell == Spell::Recharge && state.recharge_timer > 0) ||
            state.player_mana < spell.mana_cost() {
            continue;
        }

        // Player turn action
        let mut new_state = SearchState {
            player_mana: state.player_mana - spell.mana_cost(),
            spent_mana: state.spent_mana + spell.mana_cost(),
            ..state
        };

        match spell {
            Spell::MagicMissile => {
                new_state.boss_hp -= Spell::MAGIC_MISSILE_DAMAGE;
            }
            Spell::Drain => {
                new_state.boss_hp -= Spell::DRAIN_DAMAGE;
                new_state.player_hp += Spell::DRAIN_DAMAGE;
            }
            Spell::Shield => {
                new_state.shield_timer = Spell::SHIELD_DURATION;
            }
            Spell::Poison => {
                new_state.poison_timer = Spell::POISON_DURATION;
            }
            Spell::Recharge => {
                new_state.recharge_timer = Spell::RECHARGE_DURATION;
            }
        }

        // Boss turn timer effects
        new_state.process_spell_effects();

        if new_state.boss_hp <= 0 {
            result = cmp::min(result, new_state.spent_mana);
            continue;
        }

        // Boss turn action
        let player_armor = if new_state.shield_timer > 0 { Spell::SHIELD_ARMOR } else { 0 };
        let boss_damage = cmp::max(boss.damage - player_armor, 1);
        new_state.player_hp -= boss_damage;
        if new_state.player_hp <= 0 {
            continue;
        }

        result = cmp::min(result, search_for_min_cost(boss, new_state, hard_mode, min_so_far));
    }

    *min_so_far = cmp::min(*min_so_far, result);
    result
}

fn parse_input(input: &str) -> Result<Boss, SimpleError> {
    let mut lines = input.lines();

    let hit_points = match lines.next() {
        Some(line) => line.split(' ').last().ok_or(
            SimpleError::new(String::from("hit points line has no space"))
        )?,
        None => return Err(SimpleError::new(String::from("missing hit points line"))),
    };
    let hit_points: i32 = hit_points.parse()?;

    let damage = match lines.next() {
        Some(line) => line.split(' ').last().ok_or(
            SimpleError::new(String::from("damage line has no space"))
        )?,
        None => return Err(SimpleError::new(String::from("missing damage line"))),
    };
    let damage: i32 = damage.parse()?;

    Ok(Boss { hit_points, damage })
}

pub fn solve(input: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}