//! Day 18: Many-Worlds Interpretation
//! https://adventofcode.com/2019/day/18

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::error::Error;
use std::hash::Hash;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    i: usize,
    j: usize,
}

impl Point {
    fn new(i: usize, j: usize) -> Self {
        Self { i, j }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Space {
    Empty,
    Wall,
    Entrance,
    Key(char),
    Door(char),
}

impl Space {
    fn from_char(c: char) -> Result<Self, SimpleError> {
        let space = match c {
            '.' => Self::Empty,
            '#' => Self::Wall,
            '@' => Self::Entrance,
            c @ 'a'..='z' => Self::Key(c),
            c @ 'A'..='Z' => Self::Door(c),
            _ => return Err(SimpleError::new(format!("invalid space char: {c}")))
        };
        Ok(space)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct KeyBitSet {
    bits: u32,
}

impl KeyBitSet {
    fn new() -> Self {
        Self { bits: 0 }
    }

    fn plus(&self, key: char) -> Self {
        Self {
            bits: self.bits | (1 << (key as u32 - 'a' as u32)),
        }
    }

    fn contains(&self, key: char) -> bool {
        self.bits & (1 << (key as u32 - 'a' as u32)) != 0
    }

    fn len(&self) -> u32 {
        self.bits.count_ones()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct HeapEntry<T> {
    state: T,
    keys: KeyBitSet,
    steps: usize,
}

impl<T: Eq> PartialOrd for HeapEntry<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq> Ord for HeapEntry<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.steps.cmp(&other.steps).reverse()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct VisitedKey<T: Hash> {
    state: T,
    keys: KeyBitSet,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct PathToKey {
    key: char,
    position: Point,
    distance: usize,
    doors_in_path: Vec<char>,
}


fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let map = parse_input(input)?;

    let num_keys = count_keys(&map);

    let (entrance_i, entrance_j) = find_entrance(&map).ok_or_else(
        || SimpleError::new(String::from("map does not contain an entrance"))
    )?;
    let entrance = Point::new(entrance_i, entrance_j);

    let mut heap = BinaryHeap::new();
    heap.push(HeapEntry {
        state: entrance,
        keys: KeyBitSet::new(),
        steps: 0,
    });

    let mut min_distance_to_point_keys: HashMap<VisitedKey<Point>, usize> = HashMap::new();

    let position_to_key_paths = build_reachable_keys_map(&map, &vec![entrance]);

    while let Some(HeapEntry { state: position, keys, steps }) = heap.pop() {
        if keys.len() == num_keys {
            return Ok(steps);
        }

        let visited_key = VisitedKey { state: position, keys };
        if let Some(&distance) = min_distance_to_point_keys.get(&visited_key) {
            if distance <= steps {
                continue;
            }
        }

        min_distance_to_point_keys.insert(visited_key, steps);

        let reachable_keys = position_to_key_paths.get(&position).unwrap();
        for path in reachable_keys {
            if keys.contains(path.key) {
                continue;
            }

            if path.doors_in_path.iter().any(|&door| !keys.contains(door.to_ascii_lowercase())) {
                continue;
            }

            let new_keys = keys.plus(path.key);

            let new_visited_key = VisitedKey { state: path.position, keys: new_keys };
            if let Some(&existing_distance) = min_distance_to_point_keys.get(&new_visited_key) {
                if existing_distance <= steps + path.distance {
                    continue;
                }
            }

            heap.push(HeapEntry {
                state: path.position,
                keys: new_keys,
                steps: steps + path.distance,
            });
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let mut map = parse_input(input)?;

    let num_keys = count_keys(&map);

    let (entrance_i, entrance_j) = find_entrance(&map).ok_or_else(
        || SimpleError::new(String::from("map does not contain an entrance"))
    )?;

    let entrances = rewrite_entrance(&mut map, entrance_i, entrance_j);

    let position_to_key_paths = build_reachable_keys_map(&map, &entrances);

    let mut heap = BinaryHeap::new();
    heap.push(HeapEntry {
        state: entrances,
        keys: KeyBitSet::new(),
        steps: 0,
    });

    let mut min_distance_to_points_keys: HashMap<VisitedKey<Vec<Point>>, usize> = HashMap::new();


    while let Some(HeapEntry { state: positions, keys, steps }) = heap.pop() {
        if keys.len() == num_keys {
            return Ok(steps);
        }

        let visited_key = VisitedKey { state: positions.clone(), keys };
        if let Some(&distance) = min_distance_to_points_keys.get(&visited_key) {
            if distance <= steps {
                continue;
            }
        }

        min_distance_to_points_keys.insert(visited_key, steps);

        for (index, &position) in positions.iter().enumerate() {
            let reachable_keys = position_to_key_paths.get(&position).unwrap();
            for path in reachable_keys {
                if keys.contains(path.key) {
                    continue;
                }

                if path.doors_in_path.iter().any(|&door| !keys.contains(door.to_ascii_lowercase())) {
                    continue;
                }

                let mut new_positions = Vec::new();
                new_positions.extend_from_slice(&positions[..index]);
                new_positions.push(path.position);
                new_positions.extend_from_slice(&positions[index + 1..]);

                let new_keys = keys.plus(path.key);

                let new_visited_key = VisitedKey { state: new_positions.clone(), keys: new_keys };
                if let Some(&existing_distance) = min_distance_to_points_keys.get(&new_visited_key) {
                    if existing_distance <= steps + path.distance {
                        continue;
                    }
                }

                heap.push(HeapEntry {
                    state: new_positions,
                    keys: new_keys,
                    steps: steps + path.distance,
                })
            }
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn build_reachable_keys_map(map: &Vec<Vec<Space>>, starts: &Vec<Point>) -> HashMap<Point, Vec<PathToKey>> {
    let all_key_positions: Vec<_> = map.iter().enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(|(j, &space)| {
                if let Space::Key(..) = space {
                    Some((i, j))
                } else {
                    None
                }
            })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut position_to_paths = HashMap::new();
    for &(i, j) in &all_key_positions {
        let position = Point::new(i, j);
        let paths_to_other_keys = find_paths_to_keys(map, position);
        position_to_paths.insert(position, paths_to_other_keys);
    }

    for &position in starts {
        let paths_to_keys = find_paths_to_keys(map, position);
        position_to_paths.insert(position, paths_to_keys);
    }

    position_to_paths
}

fn find_paths_to_keys(map: &Vec<Vec<Space>>, position: Point) -> Vec<PathToKey> {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    visited[position.i][position.j] = true;

    let mut queue = VecDeque::new();
    queue.push_back((position, 0, Vec::new()));

    let mut paths_to_keys = Vec::new();

    while let Some((position, steps, doors_passed)) = queue.pop_front() {
        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_i = (position.i as i32 + di) as usize;
            let new_j = (position.j as i32 + dj) as usize;

            if !visited[new_i][new_j] {
                let new_position = Point::new(new_i, new_j);

                let mut new_doors_passed = doors_passed.clone();

                let new_space = map[new_i][new_j];
                match new_space {
                    Space::Wall => {
                        continue;
                    }
                    Space::Door(door) => {
                        new_doors_passed.push(door);
                    }
                    Space::Key(key) => {
                        paths_to_keys.push(PathToKey {
                            key,
                            position: new_position,
                            distance: steps + 1,
                            doors_in_path: new_doors_passed.clone(),
                        })
                    }
                    _ => {}
                }

                visited[new_i][new_j] = true;
                queue.push_back((new_position, steps + 1, new_doors_passed));
            }
        }
    }

    paths_to_keys
}

fn rewrite_entrance(map: &mut [Vec<Space>], entrance_i: usize, entrance_j: usize) -> Vec<Point> {
    for (di, dj) in [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)] {
        map[(entrance_i as i32 + di) as usize][(entrance_j as i32 + dj) as usize] = Space::Wall;
    }

    let mut new_entrances = Vec::new();
    for di in [-1, 1] {
        for dj in [-1, 1] {
            let i = (entrance_i as i32 + di) as usize;
            let j = (entrance_j as i32 + dj) as usize;

            map[i][j] = Space::Entrance;
            new_entrances.push(Point::new(i, j));
        }
    }

    new_entrances
}

fn count_keys(map: &[Vec<Space>]) -> u32 {
    map.iter().flatten()
        .filter_map(|&space| {
            if let Space::Key(key_char) = space {
                Some(key_char)
            } else {
                None
            }
        })
        .collect::<HashSet<_>>()
        .len() as u32
}

fn find_entrance(map: &[Vec<Space>]) -> Option<(usize, usize)> {
    map.iter().enumerate()
        .find_map(|(i, row)| {
            row.iter().enumerate().find_map(|(j, &space)| {
                if let Space::Entrance = space {
                    Some((i, j))
                } else {
                    None
                }
            })
        })
}

fn parse_input(input: &str) -> Result<Vec<Vec<Space>>, SimpleError> {
    input.lines().map(|line| {
        line.chars().map(Space::from_char).collect()
    })
        .collect()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = include_str!("sample_input/sample18.txt");
    const SAMPLE_INPUT_2: &str = include_str!("sample_input/sample18-2.txt");
    const SAMPLE_INPUT_3: &str = include_str!("sample_input/sample18-3.txt");
    const SAMPLE_INPUT_4: &str = include_str!("sample_input/sample18-4.txt");
    const SAMPLE_INPUT_5: &str = include_str!("sample_input/sample18-5.txt");

    const SAMPLE_INPUT_7: &str = include_str!("sample_input/sample18-7.txt");
    const SAMPLE_INPUT_8: &str = include_str!("sample_input/sample18-8.txt");
    const SAMPLE_INPUT_9: &str = include_str!("sample_input/sample18-9.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(8), solve_part_1(SAMPLE_INPUT_1));
        assert_eq!(Ok(86), solve_part_1(SAMPLE_INPUT_2));
        assert_eq!(Ok(132), solve_part_1(SAMPLE_INPUT_3));
        assert_eq!(Ok(136), solve_part_1(SAMPLE_INPUT_4));
        assert_eq!(Ok(81), solve_part_1(SAMPLE_INPUT_5));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(24), solve_part_2(SAMPLE_INPUT_7));
        assert_eq!(Ok(32), solve_part_2(SAMPLE_INPUT_8));
        assert_eq!(Ok(72), solve_part_2(SAMPLE_INPUT_9));
    }
}