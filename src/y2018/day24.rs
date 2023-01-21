//! Day 24: Immune System Simulator 20XX
//! https://adventofcode.com/2018/day/24

use crate::SimpleError;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum UnitType {
    ImmuneSystem,
    Infection,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum AttackType {
    Fire,
    Cold,
    Radiation,
    Slashing,
    Bludgeoning,
}

impl FromStr for AttackType {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fire" => Ok(Self::Fire),
            "cold" => Ok(Self::Cold),
            "radiation" => Ok(Self::Radiation),
            "slashing" => Ok(Self::Slashing),
            "bludgeoning" => Ok(Self::Bludgeoning),
            _ => Err(SimpleError::new(format!("invalid attack type string: {s}"))),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct UnitGroup {
    index: usize,
    unit_type: UnitType,
    units: u32,
    hit_points: u32,
    attack_power: u32,
    attack_type: AttackType,
    initiative: u32,
    weaknesses: Vec<AttackType>,
    immunities: Vec<AttackType>,
}

impl UnitGroup {
    fn effective_power(&self) -> u32 {
        self.units * self.attack_power
    }

    fn damage_amount_to(&self, other: &UnitGroup) -> u32 {
        if other.immunities.contains(&self.attack_type) {
            0
        } else if other.weaknesses.contains(&self.attack_type) {
            2 * self.effective_power()
        } else {
            self.effective_power()
        }
    }

    fn is_eliminated(&self) -> bool {
        self.units == 0
    }
}

fn solve_part_1(input: &str) -> Result<u32, SimpleError> {
    let (immune_system, infection) = parse_input(input)?;

    let (immune_system, infection) = run_simulation(immune_system, infection);

    let surviving_unit_groups = if !immune_system.is_empty() {
        &immune_system
    } else {
        &infection
    };

    let surviving_units = surviving_unit_groups
        .iter()
        .map(|unit_group| unit_group.units)
        .sum();

    Ok(surviving_units)
}

fn solve_part_2(input: &str) -> Result<u32, SimpleError> {
    let (immune_system, infection) = parse_input(input)?;

    for boost in 1.. {
        let mut boosted_immune_system = immune_system.clone();
        for unit_group in &mut boosted_immune_system {
            unit_group.attack_power += boost;
        }

        let (final_immune_system, final_infection) =
            run_simulation(boosted_immune_system, infection.clone());

        if !final_immune_system.is_empty() && final_infection.is_empty() {
            let surviving_units = final_immune_system
                .iter()
                .map(|unit_group| unit_group.units)
                .sum();
            return Ok(surviving_units);
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn run_simulation(
    mut immune_system: Vec<UnitGroup>,
    mut infection: Vec<UnitGroup>,
) -> (Vec<UnitGroup>, Vec<UnitGroup>) {
    while !immune_system.iter().all(UnitGroup::is_eliminated)
        && !infection.iter().all(UnitGroup::is_eliminated)
    {
        let (immune_system_targets, infection_targets) =
            target_selection_phase(&immune_system, &infection);

        if immune_system_targets.iter().all(Option::is_none)
            && infection_targets.iter().all(Option::is_none)
        {
            // No targets selected, simulation has deadlocked
            break;
        }

        let (next_immune_system, next_infection) = attack_phase(
            immune_system.clone(),
            infection.clone(),
            &immune_system_targets,
            &infection_targets,
        );

        if immune_system == next_immune_system && infection == next_infection {
            // Simulation has deadlocked, neither side can win
            break;
        }

        immune_system = next_immune_system;
        infection = next_infection;
    }

    immune_system.retain(|unit_group| !unit_group.is_eliminated());
    infection.retain(|unit_group| !unit_group.is_eliminated());

    (immune_system, infection)
}

fn target_selection_phase(
    immune_system: &[UnitGroup],
    infection: &[UnitGroup],
) -> (Vec<Option<usize>>, Vec<Option<usize>>) {
    let mut all_unit_groups: Vec<_> = immune_system.iter().chain(infection.iter()).collect();

    // Sort all unit groups by effective power desc, initiative desc
    all_unit_groups.sort_by(|&a, &b| {
        a.effective_power()
            .cmp(&b.effective_power())
            .reverse()
            .then(a.initiative.cmp(&b.initiative).reverse())
    });

    let mut immune_system_targeted = vec![false; immune_system.len()];
    let mut infection_targeted = vec![false; infection.len()];

    let mut immune_system_targets: Vec<Option<usize>> = vec![None; immune_system.len()];
    let mut infection_targets: Vec<Option<usize>> = vec![None; infection.len()];

    for &unit_group in &all_unit_groups {
        let (targets, targeted) = match unit_group.unit_type {
            UnitType::ImmuneSystem => (&infection, &mut infection_targeted),
            UnitType::Infection => (&immune_system, &mut immune_system_targeted),
        };

        // Sort enemies by damage desc, effective power desc, initiative desc
        let mut targets: Vec<_> = targets.iter().collect();
        targets.sort_by(|&a, &b| {
            unit_group
                .damage_amount_to(a)
                .cmp(&unit_group.damage_amount_to(b))
                .reverse()
                .then(a.effective_power().cmp(&b.effective_power()).reverse())
                .then(a.initiative.cmp(&b.initiative).reverse())
        });

        // Choose the first target that is not already targeted and can deal non-zero damage to
        let target = targets.into_iter().find_map(|enemy_group| {
            if !enemy_group.is_eliminated()
                && !targeted[enemy_group.index]
                && unit_group.damage_amount_to(enemy_group) > 0
            {
                Some(enemy_group.index)
            } else {
                None
            }
        });

        // If selected a target, mark it targeted
        if let Some(target) = target {
            targeted[target] = true;
        }

        match unit_group.unit_type {
            UnitType::ImmuneSystem => {
                immune_system_targets[unit_group.index] = target;
            }
            UnitType::Infection => {
                infection_targets[unit_group.index] = target;
            }
        }
    }

    (immune_system_targets, infection_targets)
}

fn attack_phase(
    mut immune_system: Vec<UnitGroup>,
    mut infection: Vec<UnitGroup>,
    immune_system_targets: &[Option<usize>],
    infection_targets: &[Option<usize>],
) -> (Vec<UnitGroup>, Vec<UnitGroup>) {
    let mut all_unit_groups: Vec<_> = immune_system
        .iter()
        .chain(infection.iter())
        .map(|unit_group| {
            (
                unit_group.unit_type,
                unit_group.index,
                unit_group.initiative,
            )
        })
        .collect();

    // Sort by initiatve desc
    all_unit_groups.sort_by(|&(_, _, a), &(_, _, b)| a.cmp(&b).reverse());

    for &(unit_type, index, _) in &all_unit_groups {
        let (unit_group, targets, target) = match unit_type {
            UnitType::ImmuneSystem => (
                &immune_system[index],
                &mut infection,
                immune_system_targets[index],
            ),
            UnitType::Infection => (
                &infection[index],
                &mut immune_system,
                infection_targets[index],
            ),
        };

        // Check if unit was eliminated earlier in the simulation
        if unit_group.is_eliminated() {
            continue;
        }

        let target = match target {
            Some(target) => &mut targets[target],
            None => {
                // Didn't select a target, skip
                continue;
            }
        };

        let damage = unit_group.damage_amount_to(target);
        let units_lost = damage / target.hit_points;
        target.units = target.units.saturating_sub(units_lost);
    }

    (immune_system, infection)
}

fn parse_input(input: &str) -> Result<(Vec<UnitGroup>, Vec<UnitGroup>), SimpleError> {
    let lines: Vec<_> = input.lines().collect();

    let split: Vec<_> = lines.split(|s| s.is_empty()).collect();
    if split.len() != 2 {
        return Err(SimpleError::new(String::from(
            "input does not have exactly one blank line",
        )));
    }

    let immune_system = parse_unit_group(split[0], UnitType::ImmuneSystem)?;
    let infection = parse_unit_group(split[1], UnitType::Infection)?;

    Ok((immune_system, infection))
}

fn parse_unit_group(lines: &[&str], unit_type: UnitType) -> Result<Vec<UnitGroup>, SimpleError> {
    lines
        .iter()
        .skip(1)
        .enumerate()
        .map(|(i, &line)| {
            let split: Vec<_> = line.splitn(8, ' ').collect();

            let units = split[0].parse()?;
            let hit_points = split[4].parse()?;

            let mut weaknesses = Vec::new();
            let mut immunities = Vec::new();
            let mut rest = split[7];
            if rest.starts_with('(') {
                let close_paren_index = rest.chars().position(|c| c == ')').ok_or_else(|| {
                    SimpleError::new(format!("line has open paren but no close paren: {line}"))
                })?;

                let (parsed_weaknesses, parsed_immunities) =
                    parse_resistances(&rest[1..close_paren_index])?;

                weaknesses = parsed_weaknesses;
                immunities = parsed_immunities;
                rest = &rest[close_paren_index + 2..];
            }

            let rest_split: Vec<_> = rest.split(' ').collect();
            if rest_split.len() != 11 {
                return Err(SimpleError::new(format!("malformed line: {line}")));
            }

            let attack_power = rest_split[5].parse()?;
            let attack_type = rest_split[6].parse()?;
            let initiative = rest_split[10].parse()?;

            Ok(UnitGroup {
                index: i,
                unit_type,
                units,
                hit_points,
                attack_power,
                attack_type,
                initiative,
                weaknesses,
                immunities,
            })
        })
        .collect()
}

fn parse_resistances(s: &str) -> Result<(Vec<AttackType>, Vec<AttackType>), SimpleError> {
    let resistance_strings = if s.contains(';') {
        s.split("; ").collect()
    } else {
        vec![s]
    };

    let mut weaknesses = Vec::new();
    let mut immunities = Vec::new();
    for &s in &resistance_strings {
        let split: Vec<_> = s.splitn(3, ' ').collect();
        let attack_types: Vec<_> = split[2]
            .split(", ")
            .map(|s| s.parse::<AttackType>())
            .collect::<Result<_, _>>()?;

        match split[0] {
            "weak" => weaknesses = attack_types,
            "immune" => immunities = attack_types,
            _ => return Err(SimpleError::new(format!("expected weak/immune: {s}"))),
        }
    }

    Ok((weaknesses, immunities))
}

pub fn solve(input: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample24.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(5216), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(51), solve_part_2(SAMPLE_INPUT));
    }
}
