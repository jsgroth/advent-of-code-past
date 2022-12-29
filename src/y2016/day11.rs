//! Day 11: Radioisotope Thermoelectric Generators
//! https://adventofcode.com/2016/day/11

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::error::Error;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum FloorItem<'a> {
    Microchip(&'a str),
    Generator(&'a str),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct FloorState {
    microchips: Vec<String>,
    generators: Vec<String>,
}

impl FloorState {
    fn new() -> Self {
        Self {
            microchips: Vec::new(),
            generators: Vec::new(),
        }
    }

    fn with_floor_items(&self, floor_items: &Vec<FloorItem>) -> FloorState {
        let mut new_state = self.clone();

        for &floor_item in floor_items {
            match floor_item {
                FloorItem::Microchip(microchip) => new_state.microchips.push(String::from(microchip)),
                FloorItem::Generator(generator) => new_state.generators.push(String::from(generator)),
            }
        }

        new_state.microchips.sort();
        new_state.generators.sort();

        new_state
    }

    fn without_floor_items(&self, floor_items: &Vec<FloorItem>) -> FloorState {
        let microchips_to_remove: HashSet<_> = floor_items.iter()
            .filter_map(|floor_item| match floor_item {
                FloorItem::Microchip(microchip) => Some(microchip),
                FloorItem::Generator(_) => None,
            })
            .copied()
            .collect();

        let generators_to_remove: HashSet<_> = floor_items.iter()
            .filter_map(|floor_item| match floor_item {
                FloorItem::Generator(generator) => Some(generator),
                FloorItem::Microchip(_) => None,
            })
            .copied()
            .collect();

        let mut new_state = self.clone();
        new_state.microchips.retain(|microchip| !microchips_to_remove.contains(microchip.as_str()));
        new_state.generators.retain(|generator| !generators_to_remove.contains(generator.as_str()));

        new_state
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct VisitedKey {
    elevator_pos: usize,
    floors: Vec<(usize, usize, usize)>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct IsolationAreaState {
    floors: Vec<FloorState>,
    elevator_pos: usize,
    steps: usize,
}

impl IsolationAreaState {
    fn new(floors: Vec<FloorState>) -> Self {
        Self {
            floors,
            elevator_pos: 0,
            steps: 0,
        }
    }

    fn is_valid_state(&self) -> bool {
        self.floors.iter().all(|floor| {
            floor.microchips.iter().all(|microchip| {
                floor.generators.is_empty() || floor.generators.contains(microchip)
            })
        })
    }

    fn is_win_state(&self) -> bool {
        self.floors[..self.floors.len() - 1].iter().all(|floor| {
            floor.microchips.is_empty() && floor.generators.is_empty()
        })
    }

    fn a_star_heuristic(&self) -> usize {
        let mut lower_bound = 0;

        let floor_count = self.floors.len();
        for i in 0..floor_count - 1 {
            let floor = &self.floors[i];
            let items = floor.microchips.len() + floor.generators.len();
            if items > 0 {
                let cost_per_floor = if i == self.elevator_pos {
                    1 + 2 * items.saturating_sub(2)
                } else {
                    1 + 2 * (items - 1)
                };
                lower_bound += cost_per_floor * (floor_count - 1 - i);
            }
        }

        self.steps + lower_bound
    }

    fn make_visited_key(&self) -> VisitedKey {
        let floors: Vec<_> = self.floors.iter().map(|floor| {
            let (paired, lone_microchips): (Vec<_>, Vec<_>) = floor.microchips.iter()
                .partition(|&microchip| floor.generators.contains(microchip));

            let lone_generator_count = floor.generators.iter()
                .filter(|&generator| !floor.microchips.contains(generator))
                .count();

            (paired.len(), lone_microchips.len(), lone_generator_count)
        })
            .collect();

        VisitedKey {
            elevator_pos: self.elevator_pos,
            floors,
        }
    }
}

impl PartialOrd<Self> for IsolationAreaState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IsolationAreaState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.a_star_heuristic().cmp(&other.a_star_heuristic()).reverse()
    }
}

fn solve_part(input: &str, add_additional_items: bool) -> Result<usize, SimpleError> {
    let initial_state = parse_input(input, add_additional_items)?;

    let mut visited: HashSet<VisitedKey> = HashSet::new();
    visited.insert(initial_state.make_visited_key());

    let mut heap: BinaryHeap<IsolationAreaState> = BinaryHeap::new();
    heap.push(initial_state);

    while !heap.is_empty() {
        let state = heap.pop().unwrap();

        let current_floor = &state.floors[state.elevator_pos];

        let move_combinations = item_combinations(&current_floor.microchips, &current_floor.generators);

        for move_combination in &move_combinations {
            let mut states_to_consider = Vec::new();

            if state.elevator_pos > 0 {
                let mut new_state = IsolationAreaState {
                    floors: state.floors.clone(),
                    elevator_pos: state.elevator_pos - 1,
                    steps: state.steps + 1,
                };
                new_state.floors[state.elevator_pos] = current_floor.without_floor_items(move_combination);
                new_state.floors[state.elevator_pos - 1] = state.floors[state.elevator_pos - 1].with_floor_items(&move_combination);
                states_to_consider.push(new_state);

            }

            if state.elevator_pos < state.floors.len() - 1 {
                let mut new_state = IsolationAreaState {
                    floors: state.floors.clone(),
                    elevator_pos: state.elevator_pos + 1,
                    steps: state.steps + 1,
                };
                new_state.floors[state.elevator_pos] = current_floor.without_floor_items(move_combination);
                new_state.floors[state.elevator_pos + 1] = state.floors[state.elevator_pos + 1].with_floor_items(&move_combination);
                states_to_consider.push(new_state);
            }

            for new_state in states_to_consider {
                if new_state.is_valid_state() {
                    if new_state.is_win_state() {
                        return Ok(new_state.steps);
                    }

                    let visited_key = new_state.make_visited_key();
                    if !visited.contains(&visited_key) {
                        visited.insert(visited_key);
                        heap.push(new_state);
                    }
                }
            }
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn item_combinations<'a>(microchips: &'a Vec<String>, generators: &'a Vec<String>) -> Vec<Vec<FloorItem<'a>>> {
    let (paired, lone_microchips): (Vec<_>, Vec<_>) = microchips.iter()
        .partition(|&microchip| generators.contains(microchip));

    let lone_generators: Vec<_> = generators.iter()
        .filter(|&generator| !microchips.contains(generator))
        .collect();

    let mut combinations: Vec<Vec<FloorItem>> = Vec::new();

    if !paired.is_empty() {
        combinations.push(vec![FloorItem::Microchip(paired[0].as_str())]);
        combinations.push(vec![FloorItem::Microchip(paired[0].as_str()), FloorItem::Generator(paired[0].as_str())]);

        if paired.len() == 1 && lone_generators.is_empty() {
            combinations.push(vec![FloorItem::Generator(paired[0].as_str())]);

            if !lone_microchips.is_empty() {
                combinations.push(vec![FloorItem::Generator(paired[0].as_str()), FloorItem::Microchip(lone_microchips[0].as_str())])
            }
        }

        if paired.len() == 1 && lone_generators.len() == 1 {
            combinations.push(vec![FloorItem::Generator(paired[0].as_str()), FloorItem::Generator(lone_generators[0].as_str())]);
        }
    }

    if !lone_microchips.is_empty() {
        combinations.push(vec![FloorItem::Microchip(lone_microchips[0].as_str())]);

        if !lone_generators.is_empty() {
            combinations.push(vec![FloorItem::Microchip(lone_microchips[0].as_str()), FloorItem::Generator(lone_generators[0].as_str())]);
        }
    }

    if !lone_generators.is_empty() {
        combinations.push(vec![FloorItem::Generator(lone_generators[0].as_str())]);
    }

    if paired.len() >= 2 {
        combinations.push(vec![FloorItem::Microchip(paired[0].as_str()), FloorItem::Microchip(paired[1].as_str())]);

        if paired.len() == 2 && lone_generators.is_empty() {
            combinations.push(vec![FloorItem::Generator(paired[0].as_str()), FloorItem::Generator(paired[1].as_str())]);
        }
    }

    if lone_microchips.len() >= 2 {
        combinations.push(vec![FloorItem::Microchip(lone_microchips[0].as_str()), FloorItem::Microchip(lone_microchips[1].as_str())]);
    }

    if lone_generators.len() >= 2 {
        combinations.push(vec![FloorItem::Generator(lone_generators[0].as_str()), FloorItem::Generator(lone_generators[1].as_str())]);
    }

    combinations
}

fn parse_input(input: &str, add_additional_items: bool) -> Result<IsolationAreaState, SimpleError> {
    let mut floors: Vec<FloorState> = Vec::new();
    for line in input.lines() {
        let floor = parse_line(line)?;
        floors.push(floor);
    }

    if floors.is_empty() {
        return Err(SimpleError::new(String::from("input is empty")));
    }

    if add_additional_items {
        let additional_items = &[String::from("elerium"), String::from("dilithium")];
        floors[0].microchips.extend_from_slice(additional_items);
        floors[0].generators.extend_from_slice(additional_items);

        floors[0].microchips.sort();
        floors[0].generators.sort();
    }

    Ok(IsolationAreaState::new(floors))
}

fn parse_line(line: &str) -> Result<FloorState, SimpleError> {
    if line.ends_with("nothing relevant.") {
        return Ok(FloorState::new());
    }

    let mut floor = FloorState::new();

    let split: Vec<_> = line.split(' ').collect();
    if split.len() <= 4 {
        return Err(SimpleError::new(format!("line does not have enough spaces: {line}")));
    }

    let line = split[4..].join(" ");
    let split_iter = if line.contains(',') {
        line.split(", ")
    } else {
        line.split(" and ")
    };
    for object_str in split_iter {
        let split: Vec<_> = object_str.split(' ').collect();
        if split.len() < 3 {
            return Err(SimpleError::new(format!("invalid object string '{object_str}' in line: {line}")));
        }

        let mut obj_type = *split.last().unwrap();
        if obj_type.ends_with(['.', ',']) {
            obj_type = &obj_type[..obj_type.len() - 1];
        }

        if obj_type == "generator" {
            let generator_type = split[split.len() - 2];
            floor.generators.push(String::from(generator_type));
        } else if obj_type == "microchip" {
            let microchip_type = split[split.len() - 2];
            if !microchip_type.ends_with("-compatible") {
                return Err(SimpleError::new(format!("invalid microchip type '{microchip_type}' in line: {line}")));
            }

            let type_len = microchip_type.len() - "-compatible".len();
            let microchip_type = &microchip_type[..type_len];
            floor.microchips.push(String::from(microchip_type));
        } else {
            return Err(SimpleError::new(format!("invalid object type '{obj_type}' in line: {line}")));
        }
    }

    floor.microchips.sort();
    floor.generators.sort();

    Ok(floor)
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part(input, false)?;
    let solution2 = solve_part(input, true)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample11.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(11), solve_part(SAMPLE_INPUT, false));
    }
}