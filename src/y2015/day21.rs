//! Day 21: RPG Simulator 20XX
//! https://adventofcode.com/2015/day/21

use std::cmp;
use std::error::Error;
use crate::SimpleError;

#[derive(Debug)]
struct Weapon {
    cost: u32,
    damage: i32,
}

#[derive(Debug)]
struct Armor {
    cost: u32,
    armor: i32,
}

#[derive(Debug)]
struct Ring {
    cost: u32,
    damage: i32,
    armor: i32,
}

struct Boss {
    hit_points: i32,
    damage: i32,
    armor: i32,
}

const WEAPONS: [Weapon; 5] = [
    Weapon { cost: 8, damage: 4 },
    Weapon { cost: 10, damage: 5 },
    Weapon { cost: 25, damage: 6 },
    Weapon { cost: 40, damage: 7 },
    Weapon { cost: 74, damage: 8 },
];

const ARMOR: [Armor; 6] = [
    Armor { cost: 0, armor: 0 },
    Armor { cost: 13, armor: 1 },
    Armor { cost: 31, armor: 2 },
    Armor { cost: 53, armor: 3 },
    Armor { cost: 75, armor: 4 },
    Armor { cost: 102, armor: 5 },
];

const RINGS: [Ring; 6] = [
    Ring { cost: 25, damage: 1, armor: 0 },
    Ring { cost: 50, damage: 2, armor: 0 },
    Ring { cost: 100, damage: 3, armor: 0 },
    Ring { cost: 20, damage: 0, armor: 1 },
    Ring { cost: 40, damage: 0, armor: 2 },
    Ring { cost: 80, damage: 0, armor: 3 },
];

fn solve_both_parts(input: &str) -> Result<(u32, u32), SimpleError> {
    let boss = parse_input(input)?;

    let ring_combinations = generate_ring_combinations(&RINGS, 2);

    let mut min_to_win = u32::MAX;
    let mut max_to_lose = u32::MIN;
    for weapon in &WEAPONS {
        for armor in &ARMOR {
            for ring_combination in &ring_combinations {
                let ring_cost = ring_combination.iter().map(|ring| ring.cost).sum::<u32>();
                let total_cost = weapon.cost + armor.cost + ring_cost;
                if can_win(&boss, weapon, armor, ring_combination) {
                    min_to_win = cmp::min(min_to_win, total_cost);
                } else {
                    max_to_lose = cmp::max(max_to_lose, total_cost);
                }
            }
        }
    }

    Ok((min_to_win, max_to_lose))
}

fn generate_ring_combinations(rings: &[Ring], remaining: usize) -> Vec<Vec<&Ring>> {
    if remaining == 0 || rings.is_empty() {
        return vec![Vec::new()];
    }

    let mut combinations = Vec::new();

    combinations.extend(generate_ring_combinations(&rings[1..], remaining));
    for mut combination in generate_ring_combinations(&rings[1..], remaining - 1) {
        combination.push(&rings[0]);
        combinations.push(combination);
    }

    combinations
}

fn can_win(boss: &Boss, weapon: &Weapon, armor: &Armor, rings: &[&Ring]) -> bool {
    let mut your_hp = 100;
    let mut boss_hp = boss.hit_points;

    let your_damage = weapon.damage + rings.iter().map(|ring| ring.damage).sum::<i32>();
    let your_armor = armor.armor + rings.iter().map(|ring| ring.armor).sum::<i32>();

    while your_hp > 0 && boss_hp > 0 {
        boss_hp -= cmp::max(your_damage - boss.armor, 1);
        if boss_hp <= 0 {
            break;
        }

        your_hp -= cmp::max(boss.damage - your_armor, 1);
    }

    your_hp > 0
}

fn parse_input(input: &str) -> Result<Boss, SimpleError> {
    let mut lines = input.lines();

    let hit_points = match lines.next() {
        Some(line) => parse_number_end_of_line(line)?,
        None => return Err(SimpleError::new(String::from("missing hit points line"))),
    };

    let damage = match lines.next() {
        Some(line) => parse_number_end_of_line(line)?,
        None => return Err(SimpleError::new(String::from("missing damage line"))),
    };

    let armor = match lines.next() {
        Some(line) => parse_number_end_of_line(line)?,
        None => return Err(SimpleError::new(String::from("missing armor line"))),
    };

    Ok(Boss { hit_points, damage, armor })
}

fn parse_number_end_of_line(line: &str) -> Result<i32, SimpleError> {
    let num = line.split(' ').last().ok_or_else(
        || SimpleError::new(format!("invalid line format: {line}"))
    )?;
    Ok(num.parse()?)
}

pub fn solve(input: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let (solution1, solution2) = solve_both_parts(input)?;

    Ok((solution1, solution2))
}