//! Day 20: Donut Maze
//! https://adventofcode.com/2019/day/20

use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::iter;
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
enum RawSpace {
    Empty,
    Wall,
    HalfPortal(char),
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Space {
    Empty,
    Wall,
    Portal(String),
}

impl From<RawSpace> for Space {
    fn from(value: RawSpace) -> Self {
        match value {
            RawSpace::Empty => Self::Empty,
            RawSpace::Wall | RawSpace::HalfPortal(_) => Self::Wall,
        }
    }
}

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let raw_maze = parse_input(input)?;
    let maze = locate_portals(&raw_maze);

    let portal_connections = build_portal_connection_map(&maze)?;

    let (start, end) = find_start_and_end(&maze)?;

    let mut visited: HashSet<_> = iter::once(start).collect();

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some((position, steps)) = queue.pop_front() {
        if let Space::Portal(_) = &maze[position.i][position.j] {
            if let Some(&connected_point) = portal_connections.get(&position) {
                if visited.insert(connected_point) {
                    queue.push_back((connected_point, steps + 1));
                }
            }
        }

        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_i = (position.i as i32 + di) as usize;
            let new_j = (position.j as i32 + dj) as usize;

            match maze[new_i][new_j] {
                Space::Wall => {},
                Space::Empty | Space::Portal(_) => {
                    let new_position = Point::new(new_i, new_j);

                    if new_position == end {
                        return Ok(steps + 1);
                    }

                    if visited.insert(new_position) {
                        queue.push_back((new_position, steps + 1));
                    }
                }
            }
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let raw_maze = parse_input(input)?;
    let maze = locate_portals(&raw_maze);

    let portal_connections = build_portal_connection_map(&maze)?;

    let (start, end) = find_start_and_end(&maze)?;

    let mut visited: HashSet<_> = iter::once((start, 0)).collect();

    let mut queue = VecDeque::new();
    queue.push_back((start, 0, 0));

    while let Some((position, depth, steps)) = queue.pop_front() {
        if let Space::Portal(_) = &maze[position.i][position.j] {
            if let Some(&connected_point) = portal_connections.get(&position) {
                let i = position.i;
                let j = position.j;

                let on_outer_edge = i == 2 || i == maze.len() - 3 || j == 2 || j == maze[0].len() - 3;
                if !(on_outer_edge && depth == 0) {
                    let new_depth = if on_outer_edge { depth - 1 } else { depth + 1 };
                    if visited.insert((connected_point, new_depth)) {
                        queue.push_back((connected_point, new_depth, steps + 1));
                    }
                }
            }
        }

        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_i = (position.i as i32 + di) as usize;
            let new_j = (position.j as i32 + dj) as usize;

            match maze[new_i][new_j] {
                Space::Wall => {},
                Space::Empty | Space::Portal(_) => {
                    let new_position = Point::new(new_i, new_j);
                    if new_position == end && depth == 0 {
                        return Ok(steps + 1);
                    }

                    if visited.insert((new_position, depth)) {
                        queue.push_back((new_position, depth, steps + 1));
                    }
                }
            }
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn find_start_and_end(grid: &[Vec<Space>]) -> Result<(Point, Point), SimpleError> {
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;

    for (i, row) in grid.iter().enumerate() {
        for (j, space) in row.iter().enumerate() {
            if let Space::Portal(label) = space {
                match label.as_str() {
                    "AA" => start = Some(Point::new(i, j)),
                    "ZZ" => end = Some(Point::new(i, j)),
                    _ => {}
                }
            }
        }
    }

    match (start, end) {
        (Some(start), Some(end)) => Ok((start, end)),
        _ => Err(SimpleError::new(format!("maze is missing start and/or end points: start={start:?}, end={end:?}")))
    }
}

fn build_portal_connection_map(maze: &[Vec<Space>]) -> Result<HashMap<Point, Point>, SimpleError> {
    let mut portal_to_points: HashMap<String, Vec<Point>> = HashMap::new();

    for (i, row) in maze.iter().enumerate() {
        for (j, space) in row.iter().enumerate() {
            if let Space::Portal(label) = space {
                let p = Point::new(i, j);
                if let Some(points) = portal_to_points.get_mut(label) {
                    points.push(p);
                } else {
                    portal_to_points.insert(label.clone(), vec![p]);
                }
            }
        }
    }

    let mut point_to_connected = HashMap::new();
    for (i, row) in maze.iter().enumerate() {
        for (j, space) in row.iter().enumerate() {
            if let Space::Portal(label) = space {
                if label.as_str() == "AA" || label.as_str() == "ZZ" {
                    // start and end are not connected to anything
                    continue;
                }

                let p = Point::new(i, j);
                let connected_point = portal_to_points.get(label).unwrap().iter()
                    .find(|&&other_p| other_p != p)
                    .copied()
                    .ok_or_else(|| SimpleError::new(format!("label {label} only has one portal in map")))?;

                point_to_connected.insert(p, connected_point);
            }
        }
    }

    Ok(point_to_connected)
}

fn locate_portals(raw_maze: &Vec<Vec<RawSpace>>) -> Vec<Vec<Space>> {
    let mut maze: Vec<Vec<_>> = raw_maze.iter().map(|row| {
        row.iter().copied().map(Space::from).collect()
    })
        .collect();

    for i in 2..raw_maze.len() - 2 {
        for j in 2..raw_maze[0].len() - 2 {
            match raw_maze[i][j] {
                RawSpace::Empty => {},
                _ => continue,
            }

            if let (RawSpace::HalfPortal(lower), RawSpace::HalfPortal(upper)) = (raw_maze[i - 1][j], raw_maze[i - 2][j]) {
                maze[i][j] = Space::Portal(String::from_iter([upper, lower].into_iter()));
            }
            if let (RawSpace::HalfPortal(upper), RawSpace::HalfPortal(lower)) = (raw_maze[i + 1][j], raw_maze[i + 2][j]) {
                maze[i][j] = Space::Portal(String::from_iter([upper, lower].into_iter()));
            }
            if let (RawSpace::HalfPortal(right), RawSpace::HalfPortal(left)) = (raw_maze[i][j - 1], raw_maze[i][j - 2]) {
                maze[i][j] = Space::Portal(String::from_iter([left, right].into_iter()));
            }
            if let (RawSpace::HalfPortal(left), RawSpace::HalfPortal(right)) = (raw_maze[i][j + 1], raw_maze[i][j + 2]) {
                maze[i][j] = Space::Portal(String::from_iter([left, right].into_iter()));
            }
        }
    }

    maze
}

fn parse_input(input: &str) -> Result<Vec<Vec<RawSpace>>, SimpleError> {
    input.lines().map(|line| {
        line.chars().map(|c| {
            match c {
                '.' => Ok(RawSpace::Empty),
                ' ' | '#' => Ok(RawSpace::Wall),
                c @ 'A'..='Z' => Ok(RawSpace::HalfPortal(c)),
                _ => Err(SimpleError::new(format!("unexpected char: {c}")))
            }
        })
            .collect()
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

    const SAMPLE_INPUT_1: &str = include_str!("sample_input/sample20.txt");
    const SAMPLE_INPUT_2: &str = include_str!("sample_input/sample20-2.txt");
    const SAMPLE_INPUT_3: &str = include_str!("sample_input/sample20-3.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(23), solve_part_1(SAMPLE_INPUT_1));
        assert_eq!(Ok(58), solve_part_1(SAMPLE_INPUT_2));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(26), solve_part_2(SAMPLE_INPUT_1));
        assert_eq!(Ok(396), solve_part_2(SAMPLE_INPUT_3));
    }
}