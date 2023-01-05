//! Day 20: A Regular Map
//! https://adventofcode.com/2018/day/20

use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::hash::Hash;
use std::iter;
use std::ops::Add;
use std::str::FromStr;
use crate::SimpleError;

#[derive(Debug, Clone)]
struct Regex {
    parts: Vec<RegexPart>,
}

#[derive(Debug, Clone)]
enum RegexPart {
    Literal(String),
    Group(Vec<Regex>),
}

impl Regex {
    fn new(parts: Vec<RegexPart>) -> Self {
        Self { parts }
    }
}

impl FromStr for Regex {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Self::new(Vec::new()));
        }

        let mut levels = Vec::new();
        let mut current_branches = Vec::new();
        let mut current_parts = Vec::new();
        let mut current_chars = String::new();
        for c in s.chars() {
            match c {
                'N' | 'S' | 'E' | 'W' => {
                    current_chars.push(c);
                }
                '(' => {
                    if !current_chars.is_empty() {
                        current_parts.push(RegexPart::Literal(current_chars));
                        current_chars = String::new();
                    }

                    levels.push((current_parts, current_branches));
                    current_parts = Vec::new();
                    current_branches = Vec::new();
                }
                ')' => {
                    if levels.is_empty() {
                        return Err(SimpleError::new(format!("unbalanced parentheses in string: {s}")));
                    }

                    current_parts.push(RegexPart::Literal(current_chars));
                    current_chars = String::new();

                    current_branches.push(Self::new(current_parts));
                    let new_group = RegexPart::Group(current_branches);

                    let (p, b) = levels.pop().unwrap();
                    current_parts = p;
                    current_branches = b;

                    current_parts.push(new_group);
                }
                '|' => {
                    current_parts.push(RegexPart::Literal(current_chars));
                    current_chars = String::new();

                    current_branches.push(Self::new(current_parts));
                    current_parts = Vec::new();
                }
                _ => return Err(SimpleError::new(format!("unexpected char '{c}' in string: {s}")))
            }
        }

        if !levels.is_empty() {
            return Err(SimpleError::new(format!("string has too many open parentheses: {s}")));
        }

        if !current_branches.is_empty() {
            return Err(SimpleError::new(format!("lowest level should not have any branches: {s}")));
        }

        if !current_chars.is_empty() {
            current_parts.push(RegexPart::Literal(current_chars));
        }

        Ok(Self::new(current_parts))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn from_char(c: char) -> Result<Self, SimpleError> {
        let direction = match c {
            'N' => Self::North,
            'S' => Self::South,
            'E' => Self::East,
            'W' => Self::West,
            _ => return Err(SimpleError::new(format!("invalid direction char: {c}")))
        };

        Ok(direction)
    }

    fn invert(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct DirectionSet {
    directions_bits: u8,
}

impl DirectionSet {
    fn new() -> Self {
        Self { directions_bits: 0x00 }
    }

    fn get_directions(&self) -> Vec<Direction> {
        let mut directions = Vec::with_capacity(4);
        if self.directions_bits & 0x01 != 0 {
            directions.push(Direction::North);
        }
        if self.directions_bits & 0x02 != 0 {
            directions.push(Direction::South);
        }
        if self.directions_bits & 0x04 != 0 {
            directions.push(Direction::East);
        }
        if self.directions_bits & 0x08 != 0 {
            directions.push(Direction::West);
        }

        directions
    }

    fn insert(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.directions_bits |= 0x01,
            Direction::South => self.directions_bits |= 0x02,
            Direction::East => self.directions_bits |= 0x04,
            Direction::West => self.directions_bits |= 0x08,
        }
    }
}

impl Default for DirectionSet {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn get_adjacent_points(&self, direction_set: &DirectionSet) -> impl Iterator<Item = Point> + '_ {
        direction_set.get_directions().into_iter()
            .map(|direction| match direction {
                Direction::North => Point::new(self.x, self.y + 1),
                Direction::South => Point::new(self.x, self.y - 1),
                Direction::East => Point::new(self.x + 1, self.y),
                Direction::West => Point::new(self.x - 1, self.y),
            })
    }
}

impl Add<Direction> for Point {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::North => Point::new(self.x, self.y + 1),
            Direction::South => Point::new(self.x, self.y - 1),
            Direction::East => Point::new(self.x + 1, self.y),
            Direction::West => Point::new(self.x - 1, self.y),
        }
    }
}

fn solve_both_parts(input: &str) -> Result<(usize, usize), SimpleError> {
    let regex = crate::read_single_line(input)?;
    let regex: Regex = regex[1..regex.len() - 1].parse()?;

    let mut map: HashMap<_, _> = iter::once(
        (Point::new(0, 0), DirectionSet::new())
    ).collect();

    fill_map(&mut map, &regex, &vec![Point::new(0, 0)])?;

    let distance_to_farthest_room = find_distance_to_farthest_room(&map);
    let num_distant_rooms = find_num_distant_rooms(&map, 1000);

    Ok((distance_to_farthest_room, num_distant_rooms))
}

fn fill_map(map: &mut HashMap<Point, DirectionSet>, regex: &Regex, positions: &Vec<Point>) -> Result<Vec<Point>, SimpleError> {
    let mut positions = positions.clone();

    for regex_part in &regex.parts {
        let mut next_positions = HashSet::new();

        match regex_part {
            RegexPart::Literal(chars) => {
                for &position in &positions {
                    let mut current_pos = position;
                    for c in chars.chars() {
                        let direction = Direction::from_char(c)?;
                        get_or_insert(map, &current_pos).insert(direction);

                        let next_pos = current_pos + direction;
                        get_or_insert(map, &next_pos).insert(direction.invert());

                        current_pos = next_pos;
                    }
                    next_positions.insert(current_pos);
                }
            }
            RegexPart::Group(branches) => {
                for branch in branches {
                    next_positions.extend(fill_map(map, branch, &positions)?);
                }
            }
        }

        positions = next_positions.into_iter().collect();
    }

    Ok(positions)
}

fn get_or_insert<'a, K, V>(map: &'a mut HashMap<K, V>, k: &K) -> &'a mut V
where
    K: Eq + Hash + Clone,
    V: Default,
{
    if map.get(k).is_none() {
        map.insert(k.clone(), Default::default());
    }

    map.get_mut(k).unwrap()
}

fn find_distance_to_farthest_room(map: &HashMap<Point, DirectionSet>) -> usize {
    let mut visited = HashSet::new();
    visited.insert(Point::new(0, 0));

    let mut queue = VecDeque::new();
    queue.push_back((Point::new(0, 0), 0));

    let mut last_distance = 0;
    while !queue.is_empty() {
        let (position, distance) = queue.pop_front().unwrap();

        last_distance = distance;

        for adjacent_point in position.get_adjacent_points(map.get(&position).unwrap()) {
            if !visited.contains(&adjacent_point) {
                visited.insert(adjacent_point);
                queue.push_back((adjacent_point, distance + 1));
            }
        }
    }

    last_distance
}

fn find_num_distant_rooms(map: &HashMap<Point, DirectionSet>, distance_threshold: usize) -> usize {
    let mut visited = HashSet::new();
    visited.insert(Point::new(0, 0));

    let mut queue = VecDeque::new();
    queue.push_back((Point::new(0, 0), 0));

    let mut distant_rooms = HashSet::new();
    while !queue.is_empty() {
        let (position, distance) = queue.pop_front().unwrap();

        if distance >= distance_threshold {
            distant_rooms.insert(position);
        }

        for adjacent_point in position.get_adjacent_points(map.get(&position).unwrap()) {
            if !visited.contains(&adjacent_point) {
                visited.insert(adjacent_point);
                queue.push_back((adjacent_point, distance + 1));
            }
        }
    }

    distant_rooms.len()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let (solution1, solution2) = solve_both_parts(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok((3, 0)), solve_both_parts("^WNE$"));
        assert_eq!(Ok((10, 0)), solve_both_parts("^ENWWW(NEEE|SSE(EE|N))$"));
        assert_eq!(Ok((18, 0)), solve_both_parts("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"));
        assert_eq!(Ok((23, 0)), solve_both_parts("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"));
        assert_eq!(Ok((31, 0)), solve_both_parts("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"));
    }
}