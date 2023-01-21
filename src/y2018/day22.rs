//! Day 22: Mode Maze
//! https://adventofcode.com/2018/day/22

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::error::Error;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RegionType {
    Rocky,
    Wet,
    Narrow,
}

impl RegionType {
    fn from_geologic_index(geologic_index: u64, depth: u64) -> Self {
        Self::from_erosion_level((geologic_index + depth) % EROSION_LEVEL_MODULO)
    }

    fn from_erosion_level(erosion_level: u64) -> Self {
        match erosion_level % 3 {
            0 => Self::Rocky,
            1 => Self::Wet,
            2 => Self::Narrow,
            _ => panic!("an unsigned integer mod 3 cannot be anything other than 0/1/2")
        }
    }

    fn risk_level(&self) -> u32 {
        match self {
            Self::Rocky => 0,
            Self::Wet => 1,
            Self::Narrow => 2,
        }
    }

    fn can_enter_with(&self, tool: Tool) -> bool {
        match self {
            Self::Rocky => tool == Tool::ClimbingGear || tool == Tool::Torch,
            Self::Wet => tool == Tool::ClimbingGear || tool == Tool::None,
            Self::Narrow => tool == Tool::Torch || tool == Tool::None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tool {
    None,
    Torch,
    ClimbingGear,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct HeapEntry {
    position: Point,
    tool: Tool,
    minutes_spent: u32,
}

impl HeapEntry {
    fn make_visited_key(&self) -> VisitedKey {
        VisitedKey {
            position: self.position,
            tool: self.tool,
        }
    }
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.minutes_spent.cmp(&other.minutes_spent).reverse()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct VisitedKey {
    position: Point,
    tool: Tool,
}

const EROSION_LEVEL_MODULO: u64 = 20183;

fn solve_part_1(input: &str) -> Result<u32, SimpleError> {
    let (depth, target) = parse_input(input)?;

    let geologic_indices = build_geologic_index_map(depth, target);

    let region_types: Vec<Vec<_>> = geologic_indices.iter()
        .map(|row| {
            row.iter().copied()
                .map(|geologic_index| {
                    RegionType::from_geologic_index(geologic_index, depth)
                })
                .collect()
        })
        .collect();

    let total_risk_level = region_types.iter().map(|row| {
        row.iter().map(|region_type| region_type.risk_level()).sum::<u32>()
    })
        .sum();

    Ok(total_risk_level)
}

fn solve_part_2(input: &str) -> Result<u32, SimpleError> {
    let (depth, target) = parse_input(input)?;

    let geologic_indices = build_geologic_index_map(depth, target);

    let shortest_path = find_shortest_path_to_target(geologic_indices, depth, target);

    shortest_path.ok_or_else(|| SimpleError::new(String::from("no path found to target")))
}

fn build_geologic_index_map(depth: u64, target: Point) -> Vec<Vec<u64>> {
    let mut geologic_indices = vec![vec![0]];

    for _ in 1..=target.x {
        expand_geologic_map_horizontally(&mut geologic_indices, depth);
    }
    for _ in 1..=target.y {
        expand_geologic_map_vertically(&mut geologic_indices, depth);
    }

    geologic_indices[target.y][target.x] = 0;

    geologic_indices
}

fn expand_geologic_map_vertically(map: &mut Vec<Vec<u64>>, depth: u64) {
    map.push(vec![0; map[0].len()]);

    let y = map.len() - 1;
    map[y][0] = (y as u64 * 48271) % EROSION_LEVEL_MODULO;
    for x in 1..map[0].len() {
        map[y][x] = ((map[y - 1][x] + depth) * (map[y][x - 1] + depth)) % EROSION_LEVEL_MODULO;
    }
}

fn expand_geologic_map_horizontally(map: &mut Vec<Vec<u64>>, depth: u64) {
    let x = map[0].len();
    map[0].push((x as u64 * 16807) % EROSION_LEVEL_MODULO);
    for y in 1..map.len() {
        let geologic_index = ((map[y - 1][x] + depth) * (map[y][x - 1] + depth)) % EROSION_LEVEL_MODULO;
        map[y].push(geologic_index);
    }
}

fn find_shortest_path_to_target(mut geologic_indices: Vec<Vec<u64>>, depth: u64, target: Point) -> Option<u32> {
    let initial_state = HeapEntry {
        position: Point::new(0, 0),
        tool: Tool::Torch,
        minutes_spent: 0,
    };

    let mut heap = BinaryHeap::new();
    heap.push(initial_state);

    let mut visited = HashSet::new();

    while !heap.is_empty() {
        let HeapEntry { position, tool, minutes_spent } = heap.pop().unwrap();

        if position == target && tool == Tool::Torch {
            return Some(minutes_spent);
        }

        if !visited.insert(VisitedKey { position, tool }) {
            continue;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if position.x == 0 && dx == -1 || position.y == 0 && dy == -1 {
                continue;
            }

            let x = (position.x as i32 + dx) as usize;
            let y = (position.y as i32 + dy) as usize;
            if x >= geologic_indices[0].len() {
                expand_geologic_map_horizontally(&mut geologic_indices, depth);
            }
            if y >= geologic_indices.len() {
                expand_geologic_map_vertically(&mut geologic_indices, depth);
            }

            let region_type = RegionType::from_geologic_index(geologic_indices[y][x], depth);
            if region_type.can_enter_with(tool) {
                let new_state = HeapEntry {
                    position: Point::new(x, y),
                    tool,
                    minutes_spent: minutes_spent + 1,
                };

                if !visited.contains(&new_state.make_visited_key()) {
                    heap.push(new_state);
                }
            }
        }

        let current_region_type = RegionType::from_geologic_index(
            geologic_indices[position.y][position.x], depth
        );
        let tool_change = get_possible_tool_change(tool, current_region_type);

        let new_state = HeapEntry {
            position,
            tool: tool_change,
            minutes_spent: minutes_spent + 7,
        };

        if !visited.contains(&new_state.make_visited_key()) {
            heap.push(new_state);
        }
    }

    None
}

fn get_possible_tool_change(tool: Tool, region_type: RegionType) -> Tool {
    match (tool, region_type) {
        (Tool::Torch, RegionType::Rocky) => Tool::ClimbingGear,
        (Tool::ClimbingGear, RegionType::Rocky) => Tool::Torch,
        (Tool::ClimbingGear, RegionType::Wet) => Tool::None,
        (Tool::None, RegionType::Wet) => Tool::ClimbingGear,
        (Tool::Torch, RegionType::Narrow) => Tool::None,
        (Tool::None, RegionType::Narrow) => Tool::Torch,
        _ => panic!("invalid tool/region combination of {tool:?} and {region_type:?}")
    }
}

fn parse_input(input: &str) -> Result<(u64, Point), SimpleError> {
    let first_line = crate::read_single_line(input)?;
    let second_line = input.lines().nth(1).ok_or_else(
        || SimpleError::new(String::from("input should have two lines"))
    )?;

    let depth = first_line["depth: ".len()..].parse()?;

    let (target_x, target_y) = second_line["target: ".len()..].split_once(',').ok_or_else(
        || SimpleError::new(format!("target string should contain one comma: {second_line}"))
    )?;

    let target_x = target_x.parse()?;
    let target_y = target_y.parse()?;
    let target = Point::new(target_x, target_y);

    Ok((depth, target))
}

pub fn solve(input: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample22.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(114), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(45), solve_part_2(SAMPLE_INPUT));
    }
}