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
enum RegexPart {
    Literal { chars: String, next: Option<Box<RegexPart>> },
    Group { branches: Vec<RegexPart>, next: Option<Box<RegexPart>> },
}

impl FromStr for RegexPart {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Self::Literal { chars: String::new(), next: None });
        }

        let part = match s.chars().next() {
            Some('N') | Some('S') | Some('E') | Some('W') => {
                let end = s.chars().position(|c| c == '(').unwrap_or(s.len());
                let chars = String::from(&s[..end]);
                let next = if end != s.len() {
                    let next_part = s[end..].parse()?;
                    Some(Box::new(next_part))
                } else {
                    None
                };
                RegexPart::Literal { chars, next }
            }
            Some('(') => {
                parse_group(s)?
            }
            _ => return Err(SimpleError::new(format!("invalid regex part string: {s}")))
        };

        Ok(part)
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
    let regex: RegexPart = regex[1..regex.len() - 1].parse()?;

    let mut map: HashMap<_, _> = iter::once(
        (Point::new(0, 0), DirectionSet::new())
    ).collect();

    fill_map(&mut map, &regex, &vec![Point::new(0, 0)])?;

    let distance_to_farthest_room = find_distance_to_farthest_room(&map);
    let num_distant_rooms = find_num_distant_rooms(&map, 1000);

    Ok((distance_to_farthest_room, num_distant_rooms))
}

fn fill_map(map: &mut HashMap<Point, DirectionSet>, regex: &RegexPart, positions: &Vec<Point>) -> Result<Vec<Point>, SimpleError> {
    let mut next_positions = HashSet::new();

    match regex {
        RegexPart::Literal { chars, .. } => {
            for &position in positions {
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
        RegexPart::Group { branches, .. } => {
            for branch in branches {
                next_positions.extend(fill_map(map, branch, positions)?);
            }
        }
    }

    let next_positions = next_positions.into_iter().collect();
    match regex {
        RegexPart::Literal { next, .. } | RegexPart::Group { next, .. } => {
            match next {
                Some(regex_part) => fill_map(map, regex_part, &next_positions),
                None => Ok(next_positions),
            }
        }
    }
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

fn parse_group(s: &str) -> Result<RegexPart, SimpleError> {
    let (group_splits, group_end_index) = find_group_splits(s)?;

    let branches = group_splits.into_iter()
        .map(|group_split| group_split.parse())
        .collect::<Result<_, _>>()?;

    let next = if group_end_index != s.len() {
        let next_part = s[group_end_index..].parse()?;
        Some(Box::new(next_part))
    } else {
        None
    };

    Ok(RegexPart::Group { branches, next })
}

fn find_group_splits(s: &str) -> Result<(Vec<&str>, usize), SimpleError> {
    let mut nesting_count = 0;
    let mut splits = Vec::new();
    let mut last_split_start = 0;
    for (i, c) in s.chars().enumerate() {
        match c {
            '(' => {
                nesting_count += 1;
                if nesting_count == 1 {
                    last_split_start = i + 1;
                }
            }
            ')' => {
                nesting_count -= 1;
                if nesting_count == 0 {
                    splits.push(&s[last_split_start..i]);
                    return Ok((splits, i + 1));
                }
            }
            '|' => {
                if nesting_count == 1 {
                    splits.push(&s[last_split_start..i]);
                    last_split_start = i + 1;
                }
            }
            'N' | 'S' | 'E' | 'W' => {},
            _ => return Err(SimpleError::new(format!("invalid char {c} in group string: {s}")))
        }
    }

    Err(SimpleError::new(format!("parentheses are not balanced in group string: {s}")))
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