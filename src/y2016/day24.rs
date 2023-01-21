//! Day 24: Air Duct Spelunking
//! https://adventofcode.com/2016/day/24

use crate::SimpleError;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;

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

#[derive(Debug)]
struct Maze {
    walls: Vec<Vec<bool>>,
    locations: HashMap<Point, usize>,
    start: Point,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct VisitedKey {
    position: Point,
    visited_locations: Vec<usize>,
}

#[derive(Debug)]
struct SearchState {
    steps: usize,
    position: Point,
    visited_locations: HashSet<usize>,
}

impl SearchState {
    fn new_initial_state(start: &Point) -> Self {
        Self {
            steps: 0,
            position: *start,
            visited_locations: HashSet::new(),
        }
    }

    fn make_visited_key(&self) -> VisitedKey {
        VisitedKey {
            position: self.position,
            visited_locations: self.visited_locations.iter().copied().collect(),
        }
    }
}

fn solve_part(input: &str, robot_must_return: bool) -> Result<usize, SimpleError> {
    let maze = parse_input(input)?;

    let rows = maze.walls.len();
    let cols = maze.walls[0].len();

    let initial_state = SearchState::new_initial_state(&maze.start);

    let mut visited = HashSet::new();
    visited.insert(initial_state.make_visited_key());

    let mut queue = VecDeque::new();
    queue.push_back(initial_state);

    while !queue.is_empty() {
        let SearchState {
            steps,
            position,
            visited_locations,
        } = queue.pop_front().unwrap();

        for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if position.i == 0 && di == -1 || position.j == 0 && dj == -1 {
                continue;
            }

            let i = ((position.i as i32) + di) as usize;
            let j = ((position.j as i32) + dj) as usize;
            if i >= rows || j >= cols || maze.walls[i][j] {
                continue;
            }

            let new_position = Point::new(i, j);

            let mut new_visited_locations = visited_locations.clone();
            if let Some(&location) = maze.locations.get(&new_position) {
                new_visited_locations.insert(location);
            }

            if new_visited_locations.len() == maze.locations.len()
                && (!robot_must_return || new_position == maze.start)
            {
                return Ok(steps + 1);
            }

            let new_state = SearchState {
                steps: steps + 1,
                position: new_position,
                visited_locations: new_visited_locations,
            };
            let visited_key = new_state.make_visited_key();
            if !visited.contains(&visited_key) {
                visited.insert(visited_key);
                queue.push_back(new_state);
            }
        }
    }

    Err(SimpleError::new(String::from("no solutions found")))
}

fn parse_input(input: &str) -> Result<Maze, SimpleError> {
    let walls: Result<Vec<Vec<_>>, _> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Ok(true),
                    _c @ '.' | _c @ '0'..='9' => Ok(false),
                    _ => Err(SimpleError::new(format!(
                        "invalid char '{c}' in line: {line}"
                    ))),
                })
                .collect()
        })
        .collect();

    let mut start: Option<Point> = None;
    let mut locations = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '0' => {
                    start = Some(Point::new(i, j));
                }
                c @ '1'..='9' => {
                    locations.insert(Point::new(i, j), c as usize);
                }
                _ => {}
            }
        }
    }

    if start.is_none() {
        return Err(SimpleError::new(String::from(
            "maze does not contain a '0'",
        )));
    }

    Ok(Maze {
        walls: walls?,
        locations,
        start: start.unwrap(),
    })
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part(input, false)?;
    let solution2 = solve_part(input, true)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample24.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(14), solve_part(SAMPLE_INPUT, false));
    }
}
