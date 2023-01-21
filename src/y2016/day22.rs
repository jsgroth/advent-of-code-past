//! Day 22: Grid Computing
//! https://adventofcode.com/2016/day/22

use crate::SimpleError;
use std::cmp;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::error::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    x: usize,
    y: usize,
    size_tb: usize,
    used_tb: usize,
    avail_tb: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum NodeType {
    Empty,
    Small,
    Large,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct VisitedKey {
    empty_location: (usize, usize),
    target_location: (usize, usize),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct SearchState {
    steps: usize,
    empty_location: (usize, usize),
    target_location: (usize, usize),
}

impl SearchState {
    fn new_initial_state(empty_location: (usize, usize), cols: usize) -> Self {
        Self {
            steps: 0,
            empty_location,
            target_location: (cols - 1, 0),
        }
    }

    fn a_star_heuristic(&self) -> usize {
        let distance_to_target = distance_between((0, 0), self.target_location);
        let empty_distance_from_target =
            distance_between(self.empty_location, self.target_location);

        self.steps + empty_distance_from_target - 1 + (distance_to_target - 1) * 5 - 2
    }

    fn make_visited_key(&self) -> VisitedKey {
        VisitedKey {
            empty_location: self.empty_location,
            target_location: self.target_location,
        }
    }
}

impl PartialOrd<Self> for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.a_star_heuristic()
            .cmp(&other.a_star_heuristic())
            .reverse()
    }
}

fn distance_between(a: (usize, usize), b: (usize, usize)) -> usize {
    ((a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()) as usize
}

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let nodes = parse_input(input)?;

    let mut viable_node_pairs = 0;
    for node in &nodes {
        for other_node in &nodes {
            if node != other_node && node.used_tb > 0 && node.used_tb <= other_node.avail_tb {
                viable_node_pairs += 1;
            }
        }
    }

    Ok(viable_node_pairs)
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let nodes = parse_input(input)?;

    let nodes = gridify_nodes(nodes);
    let node_types = classify_nodes(&nodes);

    let rows = nodes.len();
    let cols = nodes[0].len();

    let initial_empty_location = find_initial_empty_location(&node_types)?;
    let initial_state = SearchState::new_initial_state(initial_empty_location, nodes[0].len());

    let mut visited = HashSet::new();
    visited.insert(initial_state.make_visited_key());

    let mut heap = BinaryHeap::new();
    heap.push(initial_state);

    while !heap.is_empty() {
        let SearchState {
            steps,
            empty_location: (empty_x, empty_y),
            target_location,
        } = heap.pop().unwrap();

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if empty_x == 0 && dx == -1 || empty_y == 0 && dy == -1 {
                continue;
            }

            let new_empty_x = ((empty_x as i32) + dx) as usize;
            let new_empty_y = ((empty_y as i32) + dy) as usize;
            if new_empty_x >= cols || new_empty_y >= rows {
                continue;
            }

            if node_types[new_empty_y][new_empty_x] == NodeType::Large {
                continue;
            }

            let new_target_location = if (new_empty_x, new_empty_y) == target_location {
                (empty_x, empty_y)
            } else {
                target_location
            };

            if new_target_location == (0, 0) {
                return Ok(steps + 1);
            }

            let new_state = SearchState {
                steps: steps + 1,
                empty_location: (new_empty_x, new_empty_y),
                target_location: new_target_location,
            };
            let visited_key = new_state.make_visited_key();
            if !visited.contains(&visited_key) {
                visited.insert(visited_key);
                heap.push(new_state);
            }
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn find_initial_empty_location(
    node_types: &[Vec<NodeType>],
) -> Result<(usize, usize), SimpleError> {
    for (y, row) in node_types.iter().enumerate() {
        for (x, &node_type) in row.iter().enumerate() {
            if node_type == NodeType::Empty {
                return Ok((x, y));
            }
        }
    }

    Err(SimpleError::new(String::from(
        "input does not contain empty node",
    )))
}

fn gridify_nodes(mut nodes: Vec<Node>) -> Vec<Vec<Node>> {
    let (max_x, max_y) = nodes
        .iter()
        .fold((usize::MIN, usize::MIN), |(max_x, max_y), node| {
            (cmp::max(max_x, node.x), cmp::max(max_y, node.y))
        });

    nodes.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));

    let mut grid = Vec::new();
    for i in 0..=max_y {
        let start = i * (max_x + 1);
        let end = start + (max_x + 1);
        grid.push(Vec::from(&nodes[start..end]));
    }
    grid
}

fn classify_nodes(nodes: &[Vec<Node>]) -> Vec<Vec<NodeType>> {
    nodes
        .iter()
        .map(|row| {
            row.iter()
                .map(|node| {
                    if node.used_tb == 0 {
                        NodeType::Empty
                    } else if node.used_tb < 100 {
                        NodeType::Small
                    } else {
                        NodeType::Large
                    }
                })
                .collect()
        })
        .collect()
}

fn parse_input(input: &str) -> Result<Vec<Node>, SimpleError> {
    input
        .lines()
        .skip(2)
        .map(|line| {
            let words = split_whitespace(line);
            if words.len() < 4 {
                return Err(SimpleError::new(format!(
                    "invalid line, not enough words: {line}"
                )));
            }

            let node_word = &words[0]["/dev/grid/node-".len()..];
            let (x, y) = node_word.split_once('-').ok_or_else(|| {
                SimpleError::new(format!("invalid line, no '-' in node word: {line}"))
            })?;
            let x: usize = x[1..].parse()?;
            let y: usize = y[1..].parse()?;

            let size_tb = parse_size(words[1])?;
            let used_tb = parse_size(words[2])?;
            let avail_tb = parse_size(words[3])?;

            Ok(Node {
                x,
                y,
                size_tb,
                used_tb,
                avail_tb,
            })
        })
        .collect()
}

fn split_whitespace(s: &str) -> Vec<&str> {
    let chars: Vec<_> = s.chars().collect();

    let mut words = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while j < s.len() {
        if chars[j] == ' ' {
            if i != j {
                words.push(&s[i..j]);
            }
            j += 1;
            i = j;
        } else {
            j += 1;
        }
    }
    if i != j {
        words.push(&s[i..j]);
    }

    words
}

fn parse_size(s: &str) -> Result<usize, SimpleError> {
    if !s.ends_with('T') {
        return Err(SimpleError::new(format!(
            "invalid size string, does not end in 'T': {s}"
        )));
    }

    Ok(s[..s.len() - 1].parse()?)
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample22.txt");

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(7), solve_part_2(SAMPLE_INPUT));
    }
}
